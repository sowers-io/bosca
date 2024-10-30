use candle_core::{Device, Error, IndexOp, Tensor};
use candle_nn::ops::softmax;
use crate::ml::whisper::model::Model;
use tokenizers::Tokenizer;
use candle_transformers::models::whisper::{self as m};
use log::{info, warn};
use crate::ml::whisper::decoding_result::DecodingResult;
use rand::{distributions::Distribution, SeedableRng};
use serde_json::json;
use crate::activity;
use crate::ml::whisper::segment::{Segment, SegmentTiming};

#[derive(Clone, Copy, Debug)]
pub enum Task {
    Transcribe,
    Translate,
}

pub fn token_id(tokenizer: &Tokenizer, token: &str) -> candle_core::Result<u32> {
    match tokenizer.token_to_id(token) {
        None => candle_core::bail!("no token-id for {token}"),
        Some(id) => Ok(id),
    }
}

pub struct Decoder {
    model: Model,
    rng: rand::rngs::StdRng,
    task: Option<Task>,
    timestamps: bool,
    verbose: bool,
    tokenizer: Tokenizer,
    suppress_tokens: Tensor,
    sot_token: u32,
    transcribe_token: u32,
    translate_token: u32,
    eot_token: u32,
    no_speech_token: u32,
    no_timestamps_token: u32,
    language_token: Option<u32>,
}

impl Decoder {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        model: Model,
        tokenizer: Tokenizer,
        seed: u64,
        device: &Device,
        language_token: Option<u32>,
        task: Option<Task>,
        timestamps: bool,
        verbose: bool,
    ) -> Result<Self, Error> {
        let no_timestamps_token = token_id(&tokenizer, m::NO_TIMESTAMPS_TOKEN)?;
        // Suppress the notimestamps token when in timestamps mode.
        // https://github.com/openai/whisper/blob/e8622f9afc4eba139bf796c210f5c01081000472/whisper/decoding.py#L452
        let suppress_tokens: Vec<f32> = (0..model.config().vocab_size as u32)
            .map(|i| {
                if model.config().suppress_tokens.contains(&i)
                    || timestamps && i == no_timestamps_token
                {
                    f32::NEG_INFINITY
                } else {
                    0f32
                }
            })
            .collect();
        let suppress_tokens = Tensor::new(suppress_tokens.as_slice(), device)?;
        let sot_token = token_id(&tokenizer, m::SOT_TOKEN)?;
        let transcribe_token = token_id(&tokenizer, m::TRANSCRIBE_TOKEN)?;
        let translate_token = token_id(&tokenizer, m::TRANSLATE_TOKEN)?;
        let eot_token = token_id(&tokenizer, m::EOT_TOKEN)?;
        let no_speech_token = m::NO_SPEECH_TOKENS
            .iter()
            .find_map(|token| token_id(&tokenizer, token).ok());
        let no_speech_token = match no_speech_token {
            None => return Err(Error::Msg("unable to find any non-speech token".to_owned())),
            Some(n) => n,
        };
        Ok(Self {
            model,
            rng: rand::rngs::StdRng::seed_from_u64(seed),
            tokenizer,
            task,
            timestamps,
            verbose,
            suppress_tokens,
            sot_token,
            transcribe_token,
            translate_token,
            eot_token,
            no_speech_token,
            language_token,
            no_timestamps_token,
        })
    }

    fn decode(&mut self, mel: &Tensor, t: f64) -> Result<DecodingResult, activity::Error> {
        let model = &mut self.model;
        let audio_features = model.encoder_forward(mel, true).map_err(|e| activity::Error::new(e.to_string()))?;
        if self.verbose {
            println!("audio features: {:?}", audio_features.dims());
        }
        let sample_len = model.config().max_target_positions / 2;
        let mut sum_logprob = 0f64;
        let mut no_speech_prob = f64::NAN;
        let mut tokens = vec![self.sot_token];
        if let Some(language_token) = self.language_token {
            tokens.push(language_token);
        }
        match self.task {
            None | Some(Task::Transcribe) => tokens.push(self.transcribe_token),
            Some(Task::Translate) => tokens.push(self.translate_token),
        }
        if !self.timestamps {
            tokens.push(self.no_timestamps_token);
        }
        for i in 0..sample_len {
            let tokens_t = Tensor::new(tokens.as_slice(), mel.device()).map_err(|e| activity::Error::new(e.to_string()))?;

            // The model expects a batch dim but this inference loop does not handle
            // it so we add it at this point.
            let tokens_t = tokens_t.unsqueeze(0).map_err(|e| activity::Error::new(e.to_string()))?;
            let ys = model.decoder_forward(&tokens_t, &audio_features, i == 0).map_err(|e| activity::Error::new(e.to_string()))?;

            // Extract the no speech probability on the first iteration by looking at the first
            // token logits and the probability for the according token.
            if i == 0 {
                let logits = model.decoder_final_linear(&ys.i(..1).map_err(|e| activity::Error::new(e.to_string()))?).map_err(|e| activity::Error::new(e.to_string()))?
                    .i(0).map_err(|e| activity::Error::new(e.to_string()))?
                    .i(0).map_err(|e| activity::Error::new(e.to_string()))?;
                no_speech_prob = softmax(&logits, 0).map_err(|e| activity::Error::new(e.to_string()))?
                    .i(self.no_speech_token as usize).map_err(|e| activity::Error::new(e.to_string()))?
                    .to_scalar::<f32>().map_err(|e| activity::Error::new(e.to_string()))? as f64;
            }

            let (_, seq_len, _) = ys.dims3().map_err(|e| activity::Error::new(e.to_string()))?;
            let logits = model
                .decoder_final_linear(&ys.i((..1, seq_len - 1..)).map_err(|e| activity::Error::new(e.to_string()))?).map_err(|e| activity::Error::new(e.to_string()))?
                .i(0).map_err(|e| activity::Error::new(e.to_string()))?
                .i(0).map_err(|e| activity::Error::new(e.to_string()))?;
            // TODO: Besides suppress tokens, we should apply the heuristics from
            // ApplyTimestampRules, i.e.:
            // - Timestamps come in pairs, except before EOT.
            // - Timestamps should be non-decreasing.
            // - If the sum of the probabilities of timestamps is higher than any other tokens,
            //   only consider timestamps when sampling.
            // https://github.com/openai/whisper/blob/e8622f9afc4eba139bf796c210f5c01081000472/whisper/decoding.py#L439
            let logits = logits.broadcast_add(&self.suppress_tokens).map_err(|e| activity::Error::new(e.to_string()))?;
            let next_token = if t > 0f64 {
                let prs = softmax(&(&logits / t).map_err(|e| activity::Error::new(e.to_string()))?, 0).map_err(|e| activity::Error::new(e.to_string()))?;
                let logits_v: Vec<f32> = prs.to_vec1().map_err(|e| activity::Error::new(e.to_string()))?;
                let distr = rand::distributions::WeightedIndex::new(&logits_v).map_err(|e| activity::Error::new(e.to_string()))?;
                distr.sample(&mut self.rng) as u32
            } else {
                let logits_v: Vec<f32> = logits.to_vec1().map_err(|e| activity::Error::new(e.to_string()))?;
                logits_v
                    .iter()
                    .enumerate()
                    .max_by(|(_, u), (_, v)| u.total_cmp(v))
                    .map(|(i, _)| i as u32)
                    .unwrap()
            };
            tokens.push(next_token);
            let prob = softmax(&logits, candle_core::D::Minus1).map_err(|e| activity::Error::new(e.to_string()))?
                .i(next_token as usize).map_err(|e| activity::Error::new(e.to_string()))?
                .to_scalar::<f32>().map_err(|e| activity::Error::new(e.to_string()))? as f64;
            if next_token == self.eot_token || tokens.len() > model.config().max_target_positions {
                break;
            }
            sum_logprob += prob.ln();
        }
        let text = self.tokenizer.decode(&tokens, true).map_err(|e| activity::Error::new(e.to_string()))?;
        let avg_logprob = sum_logprob / tokens.len() as f64;

        Ok(DecodingResult {
            tokens,
            text,
            avg_logprob,
            no_speech_prob,
            temperature: t,
            compression_ratio: f64::NAN,
        })
    }

    fn decode_with_fallback(&mut self, segment: &Tensor) -> Result<DecodingResult, activity::Error> {
        for (i, &t) in m::TEMPERATURES.iter().enumerate() {
            let dr: Result<DecodingResult, activity::Error> = self.decode(segment, t);
            if i == m::TEMPERATURES.len() - 1 {
                return dr;
            }
            // On errors, we try again with a different temperature.
            match dr {
                Ok(dr) => {
                    let needs_fallback = dr.compression_ratio > m::COMPRESSION_RATIO_THRESHOLD
                        || dr.avg_logprob < m::LOGPROB_THRESHOLD;
                    if !needs_fallback || dr.no_speech_prob > m::NO_SPEECH_THRESHOLD {
                        return Ok(dr);
                    }
                }
                Err(err) => {
                    println!("Error running at {t}: {err}")
                }
            }
        }
        unreachable!()
    }

    pub fn run(&mut self, mel: &Tensor) -> Result<Vec<Segment>, activity::Error> {
        let (_, _, content_frames) = mel.dims3().map_err(|e| activity::Error::new(e.to_string()))?;
        let mut seek = 0;
        let mut segments = vec![];
        while seek < content_frames + m::N_FRAMES {
            // TODO: sometimes the end gets cut off, rolling back a bit to try and grab it.
            //       I'm sure there's a smarter way to achieve this, but this is quick and dirty... and should suffice for now.
            let start = ((seek as f32) * 0.8) as usize;
            let time_offset = (start * m::HOP_LENGTH) as f64 / m::SAMPLE_RATE as f64;
            let segment_size = usize::min(content_frames - start, m::N_FRAMES);
            let mel_segment = mel.narrow(2, start, segment_size).map_err(|e| activity::Error::new(e.to_string()))?;
            let segment_duration = (segment_size * m::HOP_LENGTH) as f64 / m::SAMPLE_RATE as f64;

            let dr = self.decode_with_fallback(&mel_segment)?;
            seek += segment_size;
            if dr.no_speech_prob > m::NO_SPEECH_THRESHOLD && dr.avg_logprob < m::LOGPROB_THRESHOLD {
                warn!("no speech detected, skipping {seek} -> {start} {dr:?}");
                continue;
            }
            let mut segment = Segment {
                start: time_offset,
                duration: segment_duration,
                dr,
                timing: Vec::new(),
            };
            if self.timestamps {
                info!(
                    "{:.1}s -- {:.1}s",
                    segment.start,
                    segment.start + segment.duration,
                );
                let mut tokens_to_decode = vec![];
                let mut prev_timestamp_s = 0f32;
                for &token in segment.dr.tokens.iter() {
                    if token == self.sot_token || token == self.eot_token {
                        continue;
                    }
                    // The no_timestamp_token is the last before the timestamp ones.
                    if token > self.no_timestamps_token {
                        let timestamp_s = (token - self.no_timestamps_token + 1) as f32 / 50.;
                        if !tokens_to_decode.is_empty() {
                            let text = self
                                .tokenizer
                                .decode(&tokens_to_decode, true)
                                .map_err(|e| activity::Error::new(e.to_string()))?;
                            info!("decoding 1:  {:.1}s-{:.1}s: {}", prev_timestamp_s, timestamp_s, text);
                            segment.timing.push(SegmentTiming {
                                start: prev_timestamp_s,
                                duration: timestamp_s,
                                text,
                            });
                            tokens_to_decode.clear()
                        }
                        prev_timestamp_s = timestamp_s;
                    } else {
                        tokens_to_decode.push(token)
                    }
                }
                if !tokens_to_decode.is_empty() {
                    let text = self
                        .tokenizer
                        .decode(&tokens_to_decode, true)
                        .map_err(|e| activity::Error::new(e.to_string()))?;
                    if !text.is_empty() {
                        info!("decoding 2:  {:.1}s-...: {}", prev_timestamp_s, text);
                        segment.timing.push(SegmentTiming {
                            start: prev_timestamp_s,
                            duration: (segment.duration as f32) - prev_timestamp_s,
                            text,
                        });
                    }
                    tokens_to_decode.clear()
                }
            } else {
                info!(
                    "{:.1}s -- {:.1}s: {}",
                    segment.start,
                    segment.start + segment.duration,
                    segment.dr.text,
                );
                segment.timing.push(SegmentTiming {
                    start: segment.start as f32,
                    duration: (segment.start + segment.duration) as f32,
                    text: segment.dr.text.to_owned(),
                });
            }
            segments.push(segment)
        }
        if self.verbose {
            info!("Result: {}", json!(segments).to_string())
        }
        Ok(segments)
    }
}