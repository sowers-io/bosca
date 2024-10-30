use std::path::Path;
use candle_core::Tensor;
use crate::ml::{device, multilingual};
use crate::ml::whisper::decoder::{token_id, Decoder, Task};
use crate::ml::whisper::model::{Model, WhichModel};
use hf_hub::{api::sync::Api, Repo, RepoType};
use candle_transformers::models::whisper::{self as m, audio, Config};
use tokenizers::Tokenizer;
use candle_nn::VarBuilder;
use crate::activity;
use crate::ml::whisper::segment::Segment;

mod pcm_decode;
pub mod model;
pub mod decoding_result;
pub mod segment;
pub mod decoder;
pub mod mp4_to_wav;

#[allow(clippy::too_many_arguments)]
pub fn create_segments(
    input: &Path,
    cpu: bool,
    seed: Option<u64>,
    model_id: Option<String>,
    revision: Option<String>,
    model: Option<WhichModel>,
    task: Option<Task>,
    language: Option<String>,
    quantized: bool,
    timestamps: bool,
    verbose: bool,
) -> Result<Vec<Segment>, activity::Error> {
    let seed = seed.unwrap_or(299792458);
    let model = model.unwrap_or(WhichModel::TinyEn);

    let device = device(cpu)?;
    let (default_model, default_revision) = if quantized {
        ("lmz/candle-whisper", "main")
    } else {
        model.model_and_revision()
    };
    let default_revision = default_revision.to_string();
    let (model_id, revision) = match (model_id, revision) {
        (Some(model_id), Some(revision)) => (model_id, revision),
        (Some(model_id), None) => (model_id, "main".to_string()),
        (None, Some(revision)) => (default_model.to_owned(), revision),
        (None, None) => (default_model.to_owned(), default_revision),
    };

    let (config_filename, tokenizer_filename, weights_filename) = {
        let api = Api::new().map_err(|e| activity::Error::new(e.to_string()))?;
        let repo = api.repo(Repo::with_revision(model_id, RepoType::Model, revision));
        let (config, tokenizer, model) = if quantized {
            let ext = match model {
                WhichModel::TinyEn => "tiny-en",
                WhichModel::Tiny => "tiny",
                _ => unimplemented!("no quantized support for {:?}", model),
            };
            (
                repo.get(&format!("config-{ext}.json")).map_err(|e| activity::Error::new(e.to_string()))?,
                repo.get(&format!("tokenizer-{ext}.json")).map_err(|e| activity::Error::new(e.to_string()))?,
                repo.get(&format!("model-{ext}-q80.gguf")).map_err(|e| activity::Error::new(e.to_string()))?,
            )
        } else {
            let config = repo.get("config.json").map_err(|e| activity::Error::new(e.to_string()))?;
            let tokenizer = repo.get("tokenizer.json").map_err(|e| activity::Error::new(e.to_string()))?;
            let model = repo.get("model.safetensors").map_err(|e| activity::Error::new(e.to_string()))?;
            (config, tokenizer, model)
        };
        (config, tokenizer, model)
    };
    let config: Config = serde_json::from_str(&std::fs::read_to_string(config_filename)?).map_err(|e| activity::Error::new(e.to_string()))?;
    let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(|e| activity::Error::new(e.to_string()))?;

    let mel_bytes = match config.num_mel_bins {
        80 => include_bytes!("melfilters.bytes").as_slice(),
        128 => include_bytes!("melfilters128.bytes").as_slice(),
        nmel => return Err(activity::Error::new(format!("unexpected num_mel_bins {nmel}"))),
    };
    let mut mel_filters = vec![0f32; mel_bytes.len() / 4];
    <byteorder::LittleEndian as byteorder::ByteOrder>::read_f32_into(mel_bytes, &mut mel_filters);

    let (pcm_data, sample_rate) = pcm_decode::pcm_decode(input).map_err(|e| activity::Error::new(e.to_string()))?;
    if sample_rate != m::SAMPLE_RATE as u32 {
        return Err(activity::Error::new(format!("input file must have a {} sampling rate", m::SAMPLE_RATE)));
    }
    println!("pcm data loaded {}", pcm_data.len());
    let mel = audio::pcm_to_mel(&config, &pcm_data, &mel_filters);
    let mel_len = mel.len();
    let mel = Tensor::from_vec(
        mel,
        (1, config.num_mel_bins, mel_len / config.num_mel_bins),
        &device,
    ).map_err(|e| activity::Error::new(e.to_string()))?;
    println!("loaded mel: {:?}", mel.dims());

    let m = model;
    let mut model = if quantized {
        let vb = candle_transformers::quantized_var_builder::VarBuilder::from_gguf(
            &weights_filename,
            &device,
        ).map_err(|e| activity::Error::new(e.to_string()))?;
        Model::Quantized(m::quantized_model::Whisper::load(&vb, config).map_err(|e| activity::Error::new(e.to_string()))?)
    } else {
        let vb =
            unsafe { VarBuilder::from_mmaped_safetensors(&[weights_filename], m::DTYPE, &device).map_err(|e| activity::Error::new(e.to_string()))? };
        Model::Normal(m::model::Whisper::load(&vb, config).map_err(|e| activity::Error::new(e.to_string()))?)
    };

    let language_token = match (m.is_multilingual(), language) {
        (true, None) => Some(multilingual::detect_language(&mut model, &tokenizer, &mel).map_err(|e| activity::Error::new(e.to_string()))?),
        (false, None) => None,
        (true, Some(language)) => match token_id(&tokenizer, &format!("<|{language}|>")) {
            Ok(token_id) => Some(token_id),
            Err(_) => return Err(activity::Error::new(format!("language {language} is not supported"))),
        },
        (false, Some(_)) => {
            return Err(activity::Error::new("a language cannot be set for non-multilingual models".to_string()))
        }
    };
    let mut dc = Decoder::new(
        model,
        tokenizer,
        seed,
        &device,
        language_token,
        task,
        timestamps,
        verbose,
    ).map_err(|e| activity::Error::new(e.to_string()))?;
    dc.run(&mel)
}