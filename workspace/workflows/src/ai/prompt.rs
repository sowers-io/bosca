use std::collections::{HashMap, HashSet};
use std::env;
use crate::activity::{Activity, ActivityContext, Error};
use async_trait::async_trait;
use bytes::Bytes;
use langchain_rust::chain::{Chain, ConversationalRetrieverChainBuilder, LLMChainBuilder};
use langchain_rust::{fmt_message, fmt_template, message_formatter};
use langchain_rust::llm::{OpenAI, OpenAIConfig};
use langchain_rust::output_parsers::{MarkdownParser, OutputParser};
use langchain_rust::prompt::{HumanMessagePromptTemplate, PromptTemplate, TemplateFormat};
use langchain_rust::schemas::messages::Message;
use serde_json::Value;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::client::add_metadata_supplementary::MetadataSupplementaryInput;
use bosca_client::client::plan::StorageSystemType;
use bosca_client::download::download_supplementary_path;
use bosca_client::upload::upload_multipart_supplementary_bytes;

pub struct PromptActivity {
    id: String,
}

impl Default for PromptActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl PromptActivity {
    pub fn new() -> PromptActivity {
        PromptActivity {
            id: "ai.prompt".to_string(),
        }
    }
}

#[async_trait]
impl Activity for PromptActivity {
    fn id(&self) -> &String {
        &self.id
    }

    async fn execute(&self, client: &Client, context: &mut ActivityContext, job: &WorkflowJob) -> Result<(), Error> {
        let metadata_id = &job.metadata.as_ref().unwrap().id;
        let prompt_definition = job.prompts.first().unwrap();
        let inputs: HashSet<String> = job.workflow_activity.inputs.iter().map(|input| {
            input.value.to_owned()
        }).collect();
        let prompt = message_formatter![
            fmt_message!(Message::new_system_message(
              prompt_definition.prompt.system_prompt.to_owned()
            )),
            fmt_template!(HumanMessagePromptTemplate::new(PromptTemplate::new(
                prompt_definition.prompt.user_prompt.to_owned(),
                inputs.iter().map(|s| s.to_owned()).collect(),
                TemplateFormat::FString,
            ))),
        ];
        let model_definition = job.models.first().unwrap();
        let model = match model_definition.model.type_.as_str() {
            "openai-llm" => {
                let api_key = env::var("OPENAI_API_KEY")?;
                OpenAI::new(OpenAIConfig::default().with_api_key(api_key.as_str()))
            }
            _ => {
                return Err(Error::new("unsupported model type".to_owned()));
            }
        };
        let chain: Box<dyn Chain> = if job.storage_systems.is_empty() {
            LLMChainBuilder::new()
                .prompt(prompt)
                .llm(model)
                .build()
                .map_err(|e| Error::new(format!("error: {}", e)))?
                .into()
        } else {
            let chain_builder = ConversationalRetrieverChainBuilder::new()
                .prompt(prompt)
                .llm(model);
            for storage in job.storage_systems.iter() {
                if storage.system.type_ == StorageSystemType::VECTOR {
                    let type_ = storage.configuration.get("type").unwrap().as_str().unwrap();
                    if type_ == "qdrant" {
                        // let qdrant_url = env::var("QDRANT_URL")?;
                        // let client = Qdrant::from_url(&qdrant_url).build().unwrap();
                        // let collection = storage.configuration.get("indexName").unwrap().as_str().unwrap();
                        // // TODO: configure different embedders based on storage model
                        // let embedder = FastEmbed::try_new().map_err(|e| Error::new(format!("error: {}", e)))?;
                        // let store = StoreBuilder::new()
                        //     .embedder(embedder)
                        //     .client(client)
                        //     .collection_name(collection)
                        //     .build()
                        //     .await
                        //     .map_err(|e| Error::new(format!("error: {}", e)))?;
                        // chain_builder = chain_builder.retriever(Retriever::new(store, 25));
                        todo!();
                    }
                }
            }
            chain_builder
                .build()
                .map_err(|e| Error::new(format!("error: {}", e)))?
                .into()
        };
        let mut args = HashMap::<String, Value>::new();
        for supplementary in job.metadata.as_ref().unwrap().supplementary.iter() {
            if !inputs.contains(&supplementary.key) {
                continue;
            }
            let download = client.get_metadata_supplementary_download(metadata_id, &supplementary.key).await?;
            if download.is_none() {
                return Err(Error::new("missing supplementary file".to_owned()));
            }
            let file = download_supplementary_path(metadata_id, &download.unwrap()).await?;
            context.add_file_clean(&file);
            let mut file = File::open(file).await?;
            let mut result = String::new();
            file.read_to_string(&mut result).await?;
            args.insert(supplementary.key.to_owned(), Value::String(result));
        }
        let result = chain.execute(args).await.map_err(|e| Error::new(format!("error: {}", e)))?;
        let output = result.get("output").unwrap();
        let content = MarkdownParser::new().parse(output.as_str().unwrap()).await.map_err(|e| Error::new(format!("error: {}", e)))?;
        let result_bytes = Bytes::from(content);
        let key = &job.workflow_activity.outputs.first().unwrap().value;
        if !job.metadata.as_ref().unwrap().supplementary.iter().any(|s| s.key == *key) {
            client.add_metadata_supplementary(MetadataSupplementaryInput {
                metadata_id: metadata_id.to_owned(),
                key: key.to_owned(),
                attributes: None,
                name: "Prompt Result".to_owned(),
                content_type: prompt_definition.prompt.output_type.to_owned(),
                content_length: None,
                source_id: None,
                source_identifier: None,
            }).await?;
        }
        let upload_url = client.get_metadata_supplementary_upload(metadata_id, key).await?;
        upload_multipart_supplementary_bytes(metadata_id, &prompt_definition.prompt.output_type, &upload_url, result_bytes).await?;
        Ok(())
    }
}
