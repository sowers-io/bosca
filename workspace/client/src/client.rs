use std::sync::Arc;
use std::time::Duration;
use crate::Error;
use crate::client::plan::{
    PlanWorkflowsNextWorkflowExecution, PlanWorkflowsNextWorkflowExecutionOnWorkflowJob,
};

use crate::client::add_collection::{AddCollectionContentCollection, CollectionInput};
use crate::client::enqueue_child_workflows::EnqueueChildWorkflowsWorkflowsEnqueueChildWorkflows;
use crate::client::enqueue_job::{EnqueueJobWorkflowsEnqueueJob, WorkflowExecutionIdInput};
use crate::client::find_collection::{FindAttributeInput, FindCollectionContentFindCollection};
use crate::client::find_metadata::FindMetadataContentFindMetadata;
use crate::client::get_collection::GetCollectionContentCollection;
use crate::client::metadata_download_url::MetadataDownloadUrlContentMetadataContentUrlsDownload;
use crate::client::metadata_upload_url::MetadataUploadUrlContentMetadataContentUrlsUpload;

use crate::client::source_by_id::SourceByIdContentSource;
use crate::client::trait_by_id::TraitByIdContentTrait;

use crate::client::add_metadata::{AddMetadataContentMetadata, MetadataInput};
use crate::client::add_metadata_supplementary::{
    AddMetadataSupplementaryContentMetadataAddSupplementary, MetadataSupplementaryInput,
};
use crate::client::supplementary_upload_url::SupplementaryUploadUrlContentMetadataSupplementary;

use crate::client::supplementary_download_url::SupplementaryDownloadUrlContentMetadataSupplementaryContentUrlsDownload;
use graphql_client::{GraphQLQuery, QueryBody};
use log::warn;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::Mutex;
use crate::client::add_metadata_bulk::AddMetadataBulkContentMetadataAddBulk;
use crate::client::add_search_documents::SearchDocumentInput;

#[derive(Clone)]
pub struct Client {
    token: Arc<Mutex<String>>,
    url: String,
    client: reqwest::Client,
}

pub type Trait = TraitByIdContentTrait;
pub type Source = SourceByIdContentSource;
pub type WorkflowExecution = PlanWorkflowsNextWorkflowExecution;
pub type WorkflowJob = PlanWorkflowsNextWorkflowExecutionOnWorkflowJob;
pub type EnqueuedJobId = EnqueueJobWorkflowsEnqueueJob;
pub type EnqueuedChildWorkflowId = EnqueueChildWorkflowsWorkflowsEnqueueChildWorkflows;
pub type MetadataContentDownloadUrl = MetadataDownloadUrlContentMetadataContentUrlsDownload;
pub type MetadataContentUploadUrl = MetadataUploadUrlContentMetadataContentUrlsUpload;
pub type MetadataSupplementaryDownloadUrl =
SupplementaryDownloadUrlContentMetadataSupplementaryContentUrlsDownload;
pub type MetadataSupplementaryUploadUrl = SupplementaryUploadUrlContentMetadataSupplementary;
pub type FindCollectionResult = FindCollectionContentFindCollection;
pub type FindMetadataResult = FindMetadataContentFindMetadata;
pub type AddedCollection = AddCollectionContentCollection;
pub type AddedMetadata = AddMetadataContentMetadata;
pub type AddedMetadataSupplementary = AddMetadataSupplementaryContentMetadataAddSupplementary;
pub type Collection = GetCollectionContentCollection;
pub type DateTime = String;

impl Client {
    pub fn new(url: &str) -> Client {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(600))
            .read_timeout(Duration::from_secs(600))
            .connect_timeout(Duration::from_secs(600))
            .build()
            .unwrap();
        Self {
            token: Arc::new(Mutex::new("".to_owned())),
            client,
            url: url.to_owned(),
        }
    }

    pub async fn execute_rest<Variables: Serialize, Response: DeserializeOwned + Clone>(
        &self,
        token: &str,
        url: &str,
        query: Option<Variables>,
    ) -> Result<Response, Error> {
        let response = if token.is_empty() {
            if query.is_some() {
                self.client
                    .post(url)
                    .json(query.as_ref().unwrap())
                    .send()
                    .await?
            } else {
                self.client
                    .post(url)
                    .send()
                    .await?
            }
        }  else if query.is_some() {
            self.client
                .post(url)
                .bearer_auth(token.to_owned())
                .json(query.as_ref().unwrap())
                .send()
                .await?
        } else {
            self.client
                .post(url)
                .bearer_auth(token.to_owned())
                .send()
                .await?
        };
        let text = response.text().await?;
        match serde_json::from_str::<Response>(&text) {
            Ok(response) => Ok(response),
            Err(err) => Err(Error::new(format!("error decoding message: {} -> {}", err, text)))
        }
    }

    async fn execute<Variables: Serialize, Response: DeserializeOwned + Clone>(
        &self,
        query: &QueryBody<Variables>,
    ) -> Result<Response, Error> {
        let mut builder = self.client.post(self.url.as_str()).json(query);
        let token = self.token.lock().await;
        if !token.is_empty() {
            let token = token.to_owned();
            builder = builder.bearer_auth(token);
        }
        let response = builder.json(&query).send().await?;
        let text = response.text().await?;
        let response = match serde_json::from_str::<graphql_client::Response<Response>>(&text) {
            Ok(response) => response,
            Err(err) => return Err(Error::new(format!("error decoding message: {} -> {}", err, text)))
        };
        if let Some(errors) = response.errors {
            return Err(errors.first().unwrap().clone().into());
        }
        if let Some(response) = response.data {
            return Ok(response);
        }
        Err(Error::new("missing data".to_owned()))
    }

    pub async fn login(&self, identifier: &str, password: &str) -> Result<(), Error> {
        let variables = login::Variables {
            identifier: identifier.to_owned(),
            password: password.to_owned(),
        };
        let query = Login::build_query(variables);
        let response: login::ResponseData = self.execute(&query).await?;
        let mut token = self.token.lock().await;
        token.clear();
        token.push_str(&response.security.login.password.token.token);
        Ok(())
    }

    pub async fn set_plan_context(
        &self,
        id: i64,
        queue: &str,
        context: &Value,
    ) -> Result<bool, Error> {
        let variables = set_workflow_plan_context::Variables {
            plan_id: set_workflow_plan_context::WorkflowExecutionIdInput {
                id,
                queue: queue.to_owned(),
            },
            context: context.clone(),
        };
        let query = SetWorkflowPlanContext::build_query(variables);
        let response: set_workflow_plan_context::ResponseData = self.execute(&query).await?;
        Ok(response.workflows.set_execution_plan_context)
    }

    pub async fn set_job_context(
        &self,
        id: i64,
        queue: &str,
        context: &Value,
    ) -> Result<bool, Error> {
        let variables = set_workflow_job_context::Variables {
            job_id: set_workflow_job_context::WorkflowExecutionIdInput {
                id,
                queue: queue.to_owned(),
            },
            context: context.clone(),
        };
        let query = SetWorkflowJobContext::build_query(variables);
        let response: set_workflow_job_context::ResponseData = self.execute(&query).await?;
        Ok(response.workflows.set_execution_job_context)
    }

    pub async fn set_workflow_state(
        &self,
        metadata_id: &str,
        state: &str,
        status: &str,
        immediate: bool,
    ) -> Result<bool, Error> {
        let variables = set_workflow_state::Variables {
            state: set_workflow_state::MetadataWorkflowState {
                metadata_id: metadata_id.to_owned(),
                state_id: state.to_owned(),
                status: status.to_owned(),
                immediate,
            },
        };
        let query = SetWorkflowState::build_query(variables);
        let response: set_workflow_state::ResponseData = self.execute(&query).await?;
        Ok(response.content.metadata.set_workflow_state)
    }

    pub async fn set_collection_workflow_state(
        &self,
        collection_id: &str,
        state: &str,
        status: &str,
        immediate: bool,
    ) -> Result<bool, Error> {
        let variables = set_collection_workflow_state::Variables {
            state: set_collection_workflow_state::CollectionWorkflowState {
                collection_id: collection_id.to_owned(),
                state_id: state.to_owned(),
                status: status.to_owned(),
                immediate,
            },
        };
        let query = SetCollectionWorkflowState::build_query(variables);
        let response: set_collection_workflow_state::ResponseData = self.execute(&query).await?;
        Ok(response.content.collection.set_workflow_state)
    }

    pub async fn set_workflow_state_complete(
        &self,
        metadata_id: &str,
        status: &str,
    ) -> Result<bool, Error> {
        let variables = set_workflow_state_complete::Variables {
            state: set_workflow_state_complete::MetadataWorkflowCompleteState {
                metadata_id: metadata_id.to_owned(),
                status: status.to_owned(),
            },
        };
        let query = SetWorkflowStateComplete::build_query(variables);
        let response: set_workflow_state_complete::ResponseData = self.execute(&query).await?;
        Ok(response.content.metadata.set_workflow_state_complete)
    }

    pub async fn set_collection_workflow_state_complete(
        &self,
        collection_id: &str,
        status: &str,
    ) -> Result<bool, Error> {
        let variables = set_collection_workflow_state_complete::Variables {
            state: set_collection_workflow_state_complete::CollectionWorkflowCompleteState {
                collection_id: collection_id.to_owned(),
                status: status.to_owned(),
            },
        };
        let query = SetCollectionWorkflowStateComplete::build_query(variables);
        let response: set_collection_workflow_state_complete::ResponseData = self.execute(&query).await?;
        Ok(response.content.collection.set_workflow_state_complete)
    }

    pub async fn set_workflow_job_checkin(
        &self,
        job_id: i64,
        index: i64,
        queue: &str,
    ) -> Result<bool, Error> {
        let variables = set_execution_plan_job_checkin::Variables {
            job_id: set_execution_plan_job_checkin::WorkflowJobIdInput {
                id: job_id,
                index,
                queue: queue.to_owned(),
            },
        };
        let query = SetExecutionPlanJobCheckin::build_query(variables);
        let response: set_execution_plan_job_checkin::ResponseData = self.execute(&query).await?;
        Ok(response.workflows.set_execution_plan_job_checkin)
    }

    pub async fn set_workflow_job_complete(
        &self,
        job_id: i64,
        index: i64,
        queue: &str,
    ) -> Result<bool, Error> {
        let variables = set_workflow_job_complete::Variables {
            job_id: set_workflow_job_complete::WorkflowJobIdInput {
                id: job_id,
                index,
                queue: queue.to_owned(),
            },
        };
        let query = SetWorkflowJobComplete::build_query(variables);
        let response: set_workflow_job_complete::ResponseData = self.execute(&query).await?;
        Ok(response.workflows.set_execution_plan_job_complete)
    }

    pub async fn set_workflow_job_failed(
        &self,
        job_id: i64,
        index: i64,
        queue: &str,
        error: &str,
    ) -> Result<bool, Error> {
        warn!(target: "workflow", "notifying that job failed: {}", error);
        let variables = set_workflow_job_failed::Variables {
            job_id: set_workflow_job_failed::WorkflowJobIdInput {
                id: job_id,
                index,
                queue: queue.to_owned(),
            },
            error: error.to_owned(),
        };
        let query = SetWorkflowJobFailed::build_query(variables);
        let response: set_workflow_job_failed::ResponseData = self.execute(&query).await?;
        Ok(response.workflows.set_execution_plan_job_failed)
    }

    pub async fn get_trait(&self, id: &str) -> Result<Option<Trait>, Error> {
        let variables = trait_by_id::Variables { id: id.to_owned() };
        let query = TraitById::build_query(variables);
        let response: trait_by_id::ResponseData = self.execute(&query).await?;
        Ok(response.content.trait_)
    }

    pub async fn get_source(&self, id: &str) -> Result<Option<Source>, Error> {
        let variables = source_by_id::Variables { id: id.to_string() };
        let query = SourceById::build_query(variables);
        let response: source_by_id::ResponseData = self.execute(&query).await?;
        Ok(response.content.source)
    }

    pub async fn set_metadata_attributes(
        &self,
        id: &str,
        attributes: &Value,
    ) -> Result<bool, Error> {
        let variables = set_metadata_attributes::Variables {
            id: id.to_owned(),
            attributes: attributes.clone(),
        };
        let query = SetMetadataAttributes::build_query(variables);
        let response: set_metadata_attributes::ResponseData = self.execute(&query).await?;
        Ok(response.content.metadata.set_metadata_attributes)
    }

    pub async fn set_metadata_system_attributes(
        &self,
        id: &str,
        attributes: &Value,
    ) -> Result<bool, Error> {
        let variables = set_metadata_system_attributes::Variables {
            id: id.to_owned(),
            attributes: attributes.clone(),
        };
        let query = SetMetadataSystemAttributes::build_query(variables);
        let response: set_metadata_system_attributes::ResponseData = self.execute(&query).await?;
        Ok(response.content.metadata.set_metadata_system_attributes)
    }

    pub async fn get_metadata_download_url(
        &self,
        id: &str,
    ) -> Result<MetadataContentDownloadUrl, Error> {
        let variables = metadata_download_url::Variables { id: id.to_owned() };
        let query = MetadataDownloadUrl::build_query(variables);
        let response: metadata_download_url::ResponseData = self.execute(&query).await?;
        Ok(response.content.metadata.unwrap().content.urls.download)
    }

    pub async fn get_metadata_upload_url(
        &self,
        id: &str,
    ) -> Result<MetadataContentUploadUrl, Error> {
        let variables = metadata_upload_url::Variables { id: id.to_owned() };
        let query = MetadataUploadUrl::build_query(variables);
        let response: metadata_upload_url::ResponseData = self.execute(&query).await?;
        Ok(response.content.metadata.unwrap().content.urls.upload)
    }

    pub async fn enqueue_job(
        &self,
        plan_id: i64,
        queue: &str,
        job_index: i64,
    ) -> Result<Option<EnqueuedJobId>, Error> {
        let variables = enqueue_job::Variables {
            job_index,
            plan_id: WorkflowExecutionIdInput {
                id: plan_id,
                queue: queue.to_owned(),
            },
        };
        let query = EnqueueJob::build_query(variables);
        let response: enqueue_job::ResponseData = self.execute(&query).await?;
        let job = response.workflows.enqueue_job;
        Ok(job)
    }

    pub async fn enqueue_child_workflows(
        &self,
        job_id: i64,
        queue: &str,
        workflow_ids: Vec<String>,
    ) -> Result<Vec<EnqueuedChildWorkflowId>, Error> {
        let variables = enqueue_child_workflows::Variables {
            job_id: enqueue_child_workflows::WorkflowExecutionIdInput {
                id: job_id,
                queue: queue.to_owned(),
            },
            workflow_ids: workflow_ids.clone(),
        };
        let query = EnqueueChildWorkflows::build_query(variables);
        let response: enqueue_child_workflows::ResponseData = self.execute(&query).await?;
        let job = response.workflows.enqueue_child_workflows;
        Ok(job)
    }

    pub async fn enqueue_child_workflow(
        &self,
        job_id: i64,
        queue: &str,
        workflow_id: &str,
        configurations: Vec<enqueue_child_workflow::WorkflowConfigurationInput>
    ) -> Result<enqueue_child_workflow::EnqueueChildWorkflowWorkflowsEnqueueChildWorkflow, Error> {
        let variables = enqueue_child_workflow::Variables {
            job_id: enqueue_child_workflow::WorkflowExecutionIdInput {
                id: job_id,
                queue: queue.to_owned(),
            },
            workflow_id: workflow_id.to_owned(),
            configurations,
        };
        let query = EnqueueChildWorkflow::build_query(variables);
        let response: enqueue_child_workflow::ResponseData = self.execute(&query).await?;
        let job = response.workflows.enqueue_child_workflow;
        Ok(job)
    }

    pub async fn get_next_execution(
        &self,
        queue: &str,
    ) -> Result<Option<WorkflowExecution>, Error> {
        let variables = plan::Variables {
            queue: queue.to_owned(),
        };
        let query = Plan::build_query(variables);
        let response: plan::ResponseData = self.execute(&query).await?;
        Ok(response.workflows.next_workflow_execution)
    }

    pub async fn add_collection(
        &self,
        collection: CollectionInput,
    ) -> Result<AddedCollection, Error> {
        let variables = add_collection::Variables { collection };
        let query = AddCollection::build_query(variables);
        let response: add_collection::ResponseData = self.execute(&query).await?;
        Ok(response.content.collection)
    }

    pub async fn add_child_collection(
        &self,
        id: &str,
        collection_id: &str,
    ) -> Result<String, Error> {
        let variables = add_child_collection::Variables {
            id: id.to_owned(),
            collection_id: collection_id.to_owned(),
        };
        let query = AddChildCollection::build_query(variables);
        let response: add_child_collection::ResponseData = self.execute(&query).await?;
        Ok(response.content.collection.add_child_collection.id)
    }

    pub async fn add_child_metadata(&self, id: &str, metadata_id: &str) -> Result<String, Error> {
        let variables = add_child_metadata::Variables {
            id: id.to_owned(),
            metadata_id: metadata_id.to_owned(),
        };
        let query = AddChildMetadata::build_query(variables);
        let response: add_child_metadata::ResponseData = self.execute(&query).await?;
        Ok(response.content.collection.add_child_metadata.id)
    }

    pub async fn get_collection(&self, id: &str) -> Result<Option<Collection>, Error> {
        let variables = get_collection::Variables { id: id.to_owned() };
        let query = GetCollection::build_query(variables);
        let response: get_collection::ResponseData = self.execute(&query).await?;
        Ok(response.content.collection)
    }

    pub async fn get_collection_items(&self, id: &str, offset: i64, limit: i64) -> Result<Vec<get_collection_items::GetCollectionItemsContentCollectionItems>, Error> {
        let variables = get_collection_items::Variables { id: id.to_owned(), offset, limit };
        let query = GetCollectionItems::build_query(variables);
        let response: get_collection_items::ResponseData = self.execute(&query).await?;
        Ok(response.content.collection.unwrap().items)
    }

    pub async fn find_collection(
        &self,
        attributes: Vec<FindAttributeInput>,
    ) -> Result<Vec<FindCollectionResult>, Error> {
        let variables = find_collection::Variables { attributes };
        let query = FindCollection::build_query(variables);
        let response: find_collection::ResponseData = self.execute(&query).await?;
        Ok(response.content.find_collection)
    }

    pub async fn add_metadata(&self, metadata: MetadataInput) -> Result<AddedMetadata, Error> {
        let variables = add_metadata::Variables { metadata };
        let query = AddMetadata::build_query(variables);
        let response: add_metadata::ResponseData = self.execute(&query).await?;
        Ok(response.content.metadata)
    }

    pub async fn add_metadata_bulk(&self, metadatas: Vec<add_metadata_bulk::MetadataInput>) -> Result<Vec<AddMetadataBulkContentMetadataAddBulk>, Error> {
        let variables = add_metadata_bulk::Variables { metadatas };
        let query = AddMetadataBulk::build_query(variables);
        let response: add_metadata_bulk::ResponseData = self.execute(&query).await?;
        Ok(response.content.metadata.add_bulk)
    }

    pub async fn add_metadata_supplementary(
        &self,
        supplementary: MetadataSupplementaryInput,
    ) -> Result<AddedMetadataSupplementary, Error> {
        let variables = add_metadata_supplementary::Variables { supplementary };
        let query = AddMetadataSupplementary::build_query(variables);
        let response: add_metadata_supplementary::ResponseData = self.execute(&query).await?;
        Ok(response.content.metadata.add_supplementary)
    }

    pub async fn get_metadata_supplementary_download(
        &self,
        id: &str,
        key: &str,
    ) -> Result<Option<MetadataSupplementaryDownloadUrl>, Error> {
        let variables = supplementary_download_url::Variables {
            id: id.to_owned(),
            key: key.to_owned(),
        };
        let query = SupplementaryDownloadUrl::build_query(variables);
        let response: supplementary_download_url::ResponseData = self.execute(&query).await?;
        let supplementary = response.content.metadata_supplementary;
        if supplementary.is_none() {
            return Ok(None);
        }
        Ok(Some(supplementary
            .unwrap()
            .content
            .urls
            .download))
    }

    pub async fn get_metadata_supplementary_upload(
        &self,
        id: &str,
        key: &str,
    ) -> Result<MetadataSupplementaryUploadUrl, Error> {
        let variables = supplementary_upload_url::Variables {
            id: id.to_owned(),
            key: key.to_owned(),
        };
        let query = SupplementaryUploadUrl::build_query(variables);
        let response: supplementary_upload_url::ResponseData = self.execute(&query).await?;
        Ok(response.content.metadata_supplementary.unwrap())
    }

    pub async fn find_metadata(
        &self,
        attributes: Vec<find_metadata::FindAttributeInput>,
    ) -> Result<Vec<FindMetadataResult>, Error> {
        let variables = find_metadata::Variables { attributes };
        let query = FindMetadata::build_query(variables);
        let response: find_metadata::ResponseData = self.execute(&query).await?;
        Ok(response.content.find_metadata)
    }

    pub async fn set_metadata_ready(&self, id: &str) -> Result<(), Error> {
        let variables = set_metadata_ready::Variables { id: id.to_owned() };
        let query = SetMetadataReady::build_query(variables);
        let _: set_metadata_ready::ResponseData = self.execute(&query).await?;
        Ok(())
    }

    pub async fn set_metadata_public(&self, id: &str, public: bool) -> Result<(), Error> {
        let variables = set_metadata_public::Variables { id: id.to_owned(), public };
        let query = SetMetadataPublic::build_query(variables);
        let _: set_metadata_public::ResponseData = self.execute(&query).await?;
        Ok(())
    }

    pub async fn set_collection_ready(&self, id: &str) -> Result<(), Error> {
        let variables = set_collection_ready::Variables { id: id.to_owned() };
        let query = SetCollectionReady::build_query(variables);
        let _: set_collection_ready::ResponseData = self.execute(&query).await?;
        Ok(())
    }

    pub async fn set_collection_public(&self, id: &str, public: bool) -> Result<(), Error> {
        let variables = set_collection_public::Variables { id: id.to_owned(), public };
        let query = SetCollectionPublic::build_query(variables);
        let _: set_collection_public::ResponseData = self.execute(&query).await?;
        Ok(())
    }

    pub async fn set_collection_public_list(&self, id: &str, public: bool) -> Result<(), Error> {
        let variables = set_collection_public_list::Variables { id: id.to_owned(), public };
        let query = SetCollectionPublicList::build_query(variables);
        let _: set_collection_public_list::ResponseData = self.execute(&query).await?;
        Ok(())
    }

    pub async fn add_search_documents(
        &self,
        storage_system_id: &str,
        documents: Vec<SearchDocumentInput>,
    ) -> Result<(), Error> {
        let variables = add_search_documents::Variables {
            storage_system_id: storage_system_id.to_owned(),
            documents,
        };
        let query = AddSearchDocuments::build_query(variables);
        let _: add_search_documents::ResponseData = self.execute(&query).await?;
        Ok(())
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/login.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct Login;

#[allow(clippy::upper_case_acronyms)]
type JSON = Value;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/set_metadata_attributes.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct SetMetadataAttributes;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/set_metadata_system_attributes.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct SetMetadataSystemAttributes;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/plan.graphql",
    response_derives = "Serialize, Deserialize, Debug, PartialEq, Eq, Clone"
)]
pub struct Plan;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/enqueue_job.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct EnqueueJob;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/enqueue_child_workflows.graphql",
    response_derives = "Serialize, Deserialize, Debug, PartialEq, Eq, Clone"
)]
pub struct EnqueueChildWorkflows;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/enqueue_child_workflow.graphql",
    response_derives = "Serialize, Deserialize, Debug, PartialEq, Eq, Clone"
)]
pub struct EnqueueChildWorkflow;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/trait_by_id.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct TraitById;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/source_by_id.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct SourceById;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/set_workflow_plan_context.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct SetWorkflowPlanContext;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/set_workflow_job_context.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct SetWorkflowJobContext;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/set_workflow_job_complete.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct SetWorkflowJobComplete;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/set_workflow_job_failed.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct SetWorkflowJobFailed;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/set_workflow_state.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct SetWorkflowState;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/set_collection_workflow_state.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct SetCollectionWorkflowState;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/set_workflow_state_complete.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct SetWorkflowStateComplete;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/set_collection_workflow_state_complete.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct SetCollectionWorkflowStateComplete;

#[derive(GraphQLQuery, Serialize, Deserialize)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/metadata_download_url.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct MetadataDownloadUrl;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/metadata_upload_url.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct MetadataUploadUrl;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/supplementary_upload_url.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct SupplementaryUploadUrl;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/supplementary_download_url.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct SupplementaryDownloadUrl;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/find_collection.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct FindCollection;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/find_metadata.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct FindMetadata;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/add_metadata.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct AddMetadata;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/add_metadata_bulk.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct AddMetadataBulk;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/add_metadata_supplementary.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct AddMetadataSupplementary;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/add_collection.graphql",
    variables_derives = "Deserialize, Debug, PartialEq, Eq, Clone",
    response_derives = "Serialize, Deserialize, Debug, PartialEq, Eq, Clone"
)]
pub struct AddCollection;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/add_child_collection.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct AddChildCollection;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/add_child_metadata.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct AddChildMetadata;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/collection.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct GetCollection;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/get_collection_items.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct GetCollectionItems;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/set_metadata_ready.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct SetMetadataReady;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/set_metadata_public.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct SetMetadataPublic;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/set_collection_ready.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct SetCollectionReady;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/set_collection_public.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct SetCollectionPublic;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/set_collection_public_list.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct SetCollectionPublicList;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/add_search_documents.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct AddSearchDocuments;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/set_workflow_job_checkin.graphql",
    response_derives = "Debug, PartialEq, Eq, Clone"
)]
pub struct SetExecutionPlanJobCheckin;
