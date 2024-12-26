#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(clippy::wrong_self_convention)]
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct SetWorkflowJobComplete_Workflows {
}
#[derive(Serialize, Deserialize)]
pub struct GetCollectionItems_Collection {
  items: Vec<GetCollectionItems_CollectionItem>,
}
#[derive(Serialize, Deserialize)]
pub enum ISetWorkflowStateComplete_Query {
  SetWorkflowStateComplete_Query(SetWorkflowStateComplete_Query),
}
impl ISetWorkflowStateComplete_Query {
  fn content(&self) -> &SetWorkflowStateComplete_Content {
    match self {
      ISetWorkflowStateComplete_Query::SetWorkflowStateComplete_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct MetadataSupplementary {
  metadata_id: String,
  key: String,
  name: String,
  created: String,
  modified: String,
  attributes: Option<Value>,
  uploaded: Option<String>,
  content: MetadataSupplementaryContent,
  source: MetadataSupplementarySource,
}
#[derive(Serialize, Deserialize)]
pub enum ISetCollectionPublic_Content {
  SetCollectionPublic_Content(SetCollectionPublic_Content),
}
impl ISetCollectionPublic_Content {
  fn collection(&self) -> &Option<SetCollectionPublic_Collection> {
    match self {
      ISetCollectionPublic_Content::SetCollectionPublic_Content(m) => &m.collection,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SupplementaryUploadUrl_Query {
  content: SupplementaryUploadUrl_Content,
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataUploadUrl_MetadataContent {
  MetadataUploadUrl_MetadataContent(MetadataUploadUrl_MetadataContent),
}
impl IMetadataUploadUrl_MetadataContent {
  fn urls(&self) -> &MetadataUploadUrl_MetadataContentUrls {
    match self {
      IMetadataUploadUrl_MetadataContent::MetadataUploadUrl_MetadataContent(m) => &m.urls,
    }
  }
  fn type_(&self) -> &String {
    match self {
      IMetadataUploadUrl_MetadataContent::MetadataUploadUrl_MetadataContent(m) => &m.type_,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowActivityPromptInput {
  prompt_id: String,
  configuration: Value,
}
#[derive(Serialize, Deserialize)]
pub struct ActivityParameter {
  name: String,
  type_: ActivityParameterType,
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowJob {
  WorkflowJob(WorkflowJob),
}
impl IWorkflowJob {
  fn prompts(&self) -> &Vec<WorkflowActivityPrompt> {
    match self {
      IWorkflowJob::WorkflowJob(m) => &m.prompts,
    }
  }
  fn error(&self) -> &Option<String> {
    match self {
      IWorkflowJob::WorkflowJob(m) => &m.error,
    }
  }
  fn collection_id(&self) -> &Option<String> {
    match self {
      IWorkflowJob::WorkflowJob(m) => &m.collection_id,
    }
  }
  fn collection(&self) -> &Option<Collection> {
    match self {
      IWorkflowJob::WorkflowJob(m) => &m.collection,
    }
  }
  fn children(&self) -> &Vec<WorkflowExecutionId> {
    match self {
      IWorkflowJob::WorkflowJob(m) => &m.children,
    }
  }
  fn context(&self) -> &Value {
    match self {
      IWorkflowJob::WorkflowJob(m) => &m.context,
    }
  }
  fn storage_systems(&self) -> &Vec<WorkflowActivityStorageSystem> {
    match self {
      IWorkflowJob::WorkflowJob(m) => &m.storage_systems,
    }
  }
  fn failed_children(&self) -> &Vec<WorkflowExecutionId> {
    match self {
      IWorkflowJob::WorkflowJob(m) => &m.failed_children,
    }
  }
  fn workflow(&self) -> &Workflow {
    match self {
      IWorkflowJob::WorkflowJob(m) => &m.workflow,
    }
  }
  fn activity(&self) -> &Activity {
    match self {
      IWorkflowJob::WorkflowJob(m) => &m.activity,
    }
  }
  fn version(&self) -> &Option<i64> {
    match self {
      IWorkflowJob::WorkflowJob(m) => &m.version,
    }
  }
  fn id(&self) -> &WorkflowJobId {
    match self {
      IWorkflowJob::WorkflowJob(m) => &m.id,
    }
  }
  fn completed_children(&self) -> &Vec<WorkflowExecutionId> {
    match self {
      IWorkflowJob::WorkflowJob(m) => &m.completed_children,
    }
  }
  fn workflow_activity(&self) -> &WorkflowActivity {
    match self {
      IWorkflowJob::WorkflowJob(m) => &m.workflow_activity,
    }
  }
  fn metadata(&self) -> &Option<Metadata> {
    match self {
      IWorkflowJob::WorkflowJob(m) => &m.metadata,
    }
  }
  fn supplementary_id(&self) -> &Option<String> {
    match self {
      IWorkflowJob::WorkflowJob(m) => &m.supplementary_id,
    }
  }
  fn models(&self) -> &Vec<WorkflowActivityModel> {
    match self {
      IWorkflowJob::WorkflowJob(m) => &m.models,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISetMetadataPublic_Content {
  SetMetadataPublic_Content(SetMetadataPublic_Content),
}
impl ISetMetadataPublic_Content {
  fn metadata(&self) -> &Option<SetMetadataPublic_Metadata> {
    match self {
      ISetMetadataPublic_Content::SetMetadataPublic_Content(m) => &m.metadata,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IAddMetadataBulk_Query {
  AddMetadataBulk_Query(AddMetadataBulk_Query),
}
impl IAddMetadataBulk_Query {
  fn content(&self) -> &AddMetadataBulk_Content {
    match self {
      IAddMetadataBulk_Query::AddMetadataBulk_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct AddChildCollection_Content {
  collection: Option<AddChildCollection_Collection>,
}
#[derive(Serialize, Deserialize)]
pub struct ActivityInput {
  id: String,
  name: String,
  description: String,
  child_workflow_id: Option<String>,
  configuration: Value,
  inputs: ActivityParameterInput,
  outputs: ActivityParameterInput,
}
#[derive(Serialize, Deserialize)]
pub struct SetWorkflowState_Content {
  metadata: Option<SetWorkflowState_Metadata>,
}
#[derive(Serialize, Deserialize)]
pub enum IPermission {
  Permission(Permission),
}
impl IPermission {
  fn action(&self) -> &PermissionAction {
    match self {
      IPermission::Permission(m) => &m.action,
    }
  }
  fn group(&self) -> &Group {
    match self {
      IPermission::Permission(m) => &m.group,
    }
  }
  fn group_id(&self) -> &String {
    match self {
      IPermission::Permission(m) => &m.group_id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SetWorkflowJobFailed_Query {
  workflows: SetWorkflowJobFailed_Workflows,
}
#[derive(Serialize, Deserialize)]
pub enum ISetExecutionPlanJobCheckin_Workflows {
  SetExecutionPlanJobCheckin_Workflows(SetExecutionPlanJobCheckin_Workflows),
}
impl ISetExecutionPlanJobCheckin_Workflows {
}
#[derive(Serialize, Deserialize)]
pub enum IQueues {
  Queues(Queues),
}
impl IQueues {
  fn message_queues(&self) -> &Vec<MessageQueue> {
    match self {
      IQueues::Queues(m) => &m.message_queues,
    }
  }
  fn get_messages(&self) -> &Vec<Message> {
    match self {
      IQueues::Queues(m) => &m.get_messages,
    }
  }
  fn get_message(&self) -> &Option<Message> {
    match self {
      IQueues::Queues(m) => &m.get_message,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct Activities {
  all: Vec<Activity>,
}
#[derive(Serialize, Deserialize)]
pub struct MessageQueue {
  name: String,
  stats: MessageQueueStats,
  archived_stats: MessageQueueStats,
}
#[derive(Serialize, Deserialize)]
pub struct Security {
  principal: Principal,
  login: Login,
}
#[derive(Serialize, Deserialize)]
pub struct MetadataUploadUrl_Content {
  metadata: Option<MetadataUploadUrl_Metadata>,
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowExecutionPlan {
  id: i64,
  parent: Option<WorkflowExecutionId>,
  workflow: Workflow,
  next: Option<WorkflowJobId>,
  jobs: Vec<WorkflowJob>,
  metadata_id: Option<String>,
  metadata: Option<Metadata>,
  version: Option<i64>,
  collection_id: Option<String>,
  supplementary_id: Option<String>,
  context: Value,
  pending: Vec<WorkflowJobId>,
  current: Vec<WorkflowJobId>,
  running: Vec<WorkflowJobId>,
  complete: Vec<WorkflowJobId>,
  failed: Vec<WorkflowJobId>,
  error: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub enum IMessageQueueStats {
  MessageQueueStats(MessageQueueStats),
}
impl IMessageQueueStats {
  fn max(&self) -> &Option<DateTime<Utc>> {
    match self {
      IMessageQueueStats::MessageQueueStats(m) => &m.max,
    }
  }
  fn available(&self) -> &i64 {
    match self {
      IMessageQueueStats::MessageQueueStats(m) => &m.available,
    }
  }
  fn pending(&self) -> &i64 {
    match self {
      IMessageQueueStats::MessageQueueStats(m) => &m.pending,
    }
  }
  fn min(&self) -> &Option<DateTime<Utc>> {
    match self {
      IMessageQueueStats::MessageQueueStats(m) => &m.min,
    }
  }
  fn size(&self) -> &i64 {
    match self {
      IMessageQueueStats::MessageQueueStats(m) => &m.size,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IAddChildMetadata_Content {
  AddChildMetadata_Content(AddChildMetadata_Content),
}
impl IAddChildMetadata_Content {
  fn collection(&self) -> &Option<AddChildMetadata_Collection> {
    match self {
      IAddChildMetadata_Content::AddChildMetadata_Content(m) => &m.collection,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct MetadataSupplementarySource {
  id: String,
  identifier: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct GetCollection_Content {
  collection: Option<GetCollection_Collection>,
}
#[derive(Serialize, Deserialize)]
pub enum ISetWorkflowJobContext_Workflows {
  SetWorkflowJobContext_Workflows(SetWorkflowJobContext_Workflows),
}
impl ISetWorkflowJobContext_Workflows {
}
#[derive(Serialize, Deserialize)]
pub enum ICollectionItem {
  Metadata(Metadata),
  Collection(Collection),
}
impl ICollectionItem {
}
#[derive(Serialize, Deserialize)]
pub enum IGetMetadata_MetadataSupplementaryContent {
  GetMetadata_MetadataSupplementaryContent(GetMetadata_MetadataSupplementaryContent),
}
impl IGetMetadata_MetadataSupplementaryContent {
  fn length(&self) -> &Option<i64> {
    match self {
      IGetMetadata_MetadataSupplementaryContent::GetMetadata_MetadataSupplementaryContent(m) => &m.length,
    }
  }
  fn type_(&self) -> &String {
    match self {
      IGetMetadata_MetadataSupplementaryContent::GetMetadata_MetadataSupplementaryContent(m) => &m.type_,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISupplementaryUploadUrl_SignedUrl {
  SupplementaryUploadUrl_SignedUrl(SupplementaryUploadUrl_SignedUrl),
}
impl ISupplementaryUploadUrl_SignedUrl {
  fn headers(&self) -> &Vec<SupplementaryUploadUrl_SignedUrlHeader> {
    match self {
      ISupplementaryUploadUrl_SignedUrl::SupplementaryUploadUrl_SignedUrl(m) => &m.headers,
    }
  }
  fn url(&self) -> &String {
    match self {
      ISupplementaryUploadUrl_SignedUrl::SupplementaryUploadUrl_SignedUrl(m) => &m.url,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISetWorkflowJobComplete_Query {
  SetWorkflowJobComplete_Query(SetWorkflowJobComplete_Query),
}
impl ISetWorkflowJobComplete_Query {
  fn workflows(&self) -> &SetWorkflowJobComplete_Workflows {
    match self {
      ISetWorkflowJobComplete_Query::SetWorkflowJobComplete_Query(m) => &m.workflows,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SetWorkflowStateComplete_Query {
  content: SetWorkflowStateComplete_Content,
}
#[derive(Serialize, Deserialize)]
pub enum IContent {
  Content(Content),
  AddChildCollection_Content(AddChildCollection_Content),
  AddChildMetadata_Content(AddChildMetadata_Content),
  AddCollection_Content(AddCollection_Content),
  AddMetadata_Content(AddMetadata_Content),
  AddMetadataBulk_Content(AddMetadataBulk_Content),
  AddMetadataSupplementary_Content(AddMetadataSupplementary_Content),
  AddSearchDocuments_Content(AddSearchDocuments_Content),
  GetCollection_Content(GetCollection_Content),
  FindCollection_Content(FindCollection_Content),
  FindMetadata_Content(FindMetadata_Content),
  GetCollectionItems_Content(GetCollectionItems_Content),
  GetMetadata_Content(GetMetadata_Content),
  MetadataDownloadUrl_Content(MetadataDownloadUrl_Content),
  MetadataUploadUrl_Content(MetadataUploadUrl_Content),
  SetCollectionPublic_Content(SetCollectionPublic_Content),
  SetCollectionPublicList_Content(SetCollectionPublicList_Content),
  SetCollectionReady_Content(SetCollectionReady_Content),
  SetCollectionWorkflowState_Content(SetCollectionWorkflowState_Content),
  SetCollectionWorkflowStateComplete_Content(SetCollectionWorkflowStateComplete_Content),
  SetMetadataAttributes_Content(SetMetadataAttributes_Content),
  SetMetadataPublic_Content(SetMetadataPublic_Content),
  SetMetadataReady_Content(SetMetadataReady_Content),
  SetMetadataSystemAttributes_Content(SetMetadataSystemAttributes_Content),
  SetWorkflowState_Content(SetWorkflowState_Content),
  SetWorkflowStateComplete_Content(SetWorkflowStateComplete_Content),
  SourceById_Content(SourceById_Content),
  SupplementaryDownloadUrl_Content(SupplementaryDownloadUrl_Content),
  SupplementaryUploadUrl_Content(SupplementaryUploadUrl_Content),
  TraitById_Content(TraitById_Content),
}
impl IContent {
}
#[derive(Serialize, Deserialize)]
pub enum IFindMetadata_Metadata {
  FindMetadata_Metadata(FindMetadata_Metadata),
}
impl IFindMetadata_Metadata {
  fn id(&self) -> &String {
    match self {
      IFindMetadata_Metadata::FindMetadata_Metadata(m) => &m.id,
    }
  }
  fn created(&self) -> &DateTime<Utc> {
    match self {
      IFindMetadata_Metadata::FindMetadata_Metadata(m) => &m.created,
    }
  }
  fn modified(&self) -> &DateTime<Utc> {
    match self {
      IFindMetadata_Metadata::FindMetadata_Metadata(m) => &m.modified,
    }
  }
  fn source(&self) -> &FindMetadata_MetadataSource {
    match self {
      IFindMetadata_Metadata::FindMetadata_Metadata(m) => &m.source,
    }
  }
  fn content(&self) -> &FindMetadata_MetadataContent {
    match self {
      IFindMetadata_Metadata::FindMetadata_Metadata(m) => &m.content,
    }
  }
  fn supplementary(&self) -> &Vec<FindMetadata_MetadataSupplementary> {
    match self {
      IFindMetadata_Metadata::FindMetadata_Metadata(m) => &m.supplementary,
    }
  }
  fn name(&self) -> &String {
    match self {
      IFindMetadata_Metadata::FindMetadata_Metadata(m) => &m.name,
    }
  }
  fn labels(&self) -> &Vec<String> {
    match self {
      IFindMetadata_Metadata::FindMetadata_Metadata(m) => &m.labels,
    }
  }
  fn attributes(&self) -> &Value {
    match self {
      IFindMetadata_Metadata::FindMetadata_Metadata(m) => &m.attributes,
    }
  }
  fn version(&self) -> &i64 {
    match self {
      IFindMetadata_Metadata::FindMetadata_Metadata(m) => &m.version,
    }
  }
  fn trait_ids(&self) -> &Vec<String> {
    match self {
      IFindMetadata_Metadata::FindMetadata_Metadata(m) => &m.trait_ids,
    }
  }
  fn language_tag(&self) -> &String {
    match self {
      IFindMetadata_Metadata::FindMetadata_Metadata(m) => &m.language_tag,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SetCollectionPublic_Content {
  collection: Option<SetCollectionPublic_Collection>,
}
#[derive(Serialize, Deserialize)]
pub enum IEnqueueChildWorkflow_Query {
  EnqueueChildWorkflow_Query(EnqueueChildWorkflow_Query),
}
impl IEnqueueChildWorkflow_Query {
  fn workflows(&self) -> &EnqueueChildWorkflow_Workflows {
    match self {
      IEnqueueChildWorkflow_Query::EnqueueChildWorkflow_Query(m) => &m.workflows,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SetCollectionWorkflowStateComplete_Collection {
}
#[derive(Serialize, Deserialize)]
pub enum IAddMetadataSupplementary_Metadata {
  AddMetadataSupplementary_Metadata(AddMetadataSupplementary_Metadata),
}
impl IAddMetadataSupplementary_Metadata {
}
#[derive(Serialize, Deserialize)]
pub enum IGetCollectionItems_CollectionItem {
}
impl IGetCollectionItems_CollectionItem {
}
#[derive(Serialize, Deserialize)]
pub struct Plan_Workflows {
  next_workflow_execution: Option<Plan_WorkflowExecution>,
}
#[derive(Serialize, Deserialize)]
pub enum IEnqueueChildWorkflow_Workflows {
  EnqueueChildWorkflow_Workflows(EnqueueChildWorkflow_Workflows),
}
impl IEnqueueChildWorkflow_Workflows {
}
#[derive(Serialize, Deserialize)]
pub struct MetadataUploadUrl_MetadataContent {
  type_: String,
  urls: MetadataUploadUrl_MetadataContentUrls,
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowActivityModel {
  configuration: Value,
  model: Model,
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowsMutation {
  WorkflowsMutation(WorkflowsMutation),
}
impl IWorkflowsMutation {
  fn set_execution_plan_context(&self) -> &bool {
    match self {
      IWorkflowsMutation::WorkflowsMutation(m) => &m.set_execution_plan_context,
    }
  }
  fn begin_transition(&self) -> &bool {
    match self {
      IWorkflowsMutation::WorkflowsMutation(m) => &m.begin_transition,
    }
  }
  fn edit(&self) -> &Workflow {
    match self {
      IWorkflowsMutation::WorkflowsMutation(m) => &m.edit,
    }
  }
  fn set_execution_plan_job_checkin(&self) -> &bool {
    match self {
      IWorkflowsMutation::WorkflowsMutation(m) => &m.set_execution_plan_job_checkin,
    }
  }
  fn add(&self) -> &Workflow {
    match self {
      IWorkflowsMutation::WorkflowsMutation(m) => &m.add,
    }
  }
  fn prompts(&self) -> &PromptsMutation {
    match self {
      IWorkflowsMutation::WorkflowsMutation(m) => &m.prompts,
    }
  }
  fn set_execution_job_context(&self) -> &bool {
    match self {
      IWorkflowsMutation::WorkflowsMutation(m) => &m.set_execution_job_context,
    }
  }
  fn delete(&self) -> &bool {
    match self {
      IWorkflowsMutation::WorkflowsMutation(m) => &m.delete,
    }
  }
  fn enqueue_workflow(&self) -> &WorkflowExecutionId {
    match self {
      IWorkflowsMutation::WorkflowsMutation(m) => &m.enqueue_workflow,
    }
  }
  fn activities(&self) -> &ActivitiesMutation {
    match self {
      IWorkflowsMutation::WorkflowsMutation(m) => &m.activities,
    }
  }
  fn states(&self) -> &WorkflowStatesMutation {
    match self {
      IWorkflowsMutation::WorkflowsMutation(m) => &m.states,
    }
  }
  fn models(&self) -> &ModelsMutation {
    match self {
      IWorkflowsMutation::WorkflowsMutation(m) => &m.models,
    }
  }
  fn set_execution_plan_job_failed(&self) -> &bool {
    match self {
      IWorkflowsMutation::WorkflowsMutation(m) => &m.set_execution_plan_job_failed,
    }
  }
  fn set_execution_plan_job_complete(&self) -> &bool {
    match self {
      IWorkflowsMutation::WorkflowsMutation(m) => &m.set_execution_plan_job_complete,
    }
  }
  fn find_and_enqueue_workflow(&self) -> &Vec<WorkflowExecutionId> {
    match self {
      IWorkflowsMutation::WorkflowsMutation(m) => &m.find_and_enqueue_workflow,
    }
  }
  fn enqueue_child_workflows(&self) -> &Vec<WorkflowExecutionId> {
    match self {
      IWorkflowsMutation::WorkflowsMutation(m) => &m.enqueue_child_workflows,
    }
  }
  fn enqueue_job(&self) -> &Option<WorkflowExecutionId> {
    match self {
      IWorkflowsMutation::WorkflowsMutation(m) => &m.enqueue_job,
    }
  }
  fn enqueue_child_workflow(&self) -> &WorkflowExecutionId {
    match self {
      IWorkflowsMutation::WorkflowsMutation(m) => &m.enqueue_child_workflow,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct CollectionInput {
  parent_collection_id: Option<String>,
  collection_type: Option<CollectionType>,
  name: String,
  description: Option<String>,
  labels: Vec<String>,
  attributes: Option<Value>,
  ordering: Option<Value>,
  state: Option<CollectionWorkflowInput>,
  index: Option<bool>,
  collections: CollectionChildInput,
  metadata: MetadataChildInput,
  ready: Option<bool>,
}
#[derive(Serialize, Deserialize)]
pub struct Model {
  id: String,
  type_: String,
  name: String,
  description: String,
  configuration: Value,
}
#[derive(Serialize, Deserialize)]
pub struct FindMetadata_MetadataSource {
  id: Option<String>,
  identifier: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct MetadataUploadUrl_MetadataContentUrls {
  upload: MetadataUploadUrl_SignedUrl,
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowActivityPrompt {
  configuration: Value,
  prompt: Prompt,
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowStates {
  all: Vec<WorkflowState>,
  state: Option<WorkflowState>,
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowStatesMutation {
  WorkflowStatesMutation(WorkflowStatesMutation),
}
impl IWorkflowStatesMutation {
  fn add(&self) -> &Option<WorkflowState> {
    match self {
      IWorkflowStatesMutation::WorkflowStatesMutation(m) => &m.add,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowInput {
  id: String,
  name: String,
  description: String,
  queue: String,
  configuration: Value,
  activities: WorkflowActivityInput,
}
#[derive(Serialize, Deserialize)]
pub struct FindMetadata_Content {
  find_metadata: Vec<FindMetadata_Metadata>,
}
#[derive(Serialize, Deserialize)]
pub struct CollectionWorkflowState {
  collection_id: String,
  state_id: String,
  status: String,
  immediate: bool,
}
#[derive(Serialize, Deserialize)]
pub struct SupplementaryDownloadUrl_SignedUrlHeader {
  name: String,
  value: String,
}
#[derive(Serialize, Deserialize)]
pub enum ISetMetadataReady_Query {
  SetMetadataReady_Query(SetMetadataReady_Query),
}
impl ISetMetadataReady_Query {
  fn content(&self) -> &SetMetadataReady_Content {
    match self {
      ISetMetadataReady_Query::SetMetadataReady_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISetMetadataSystemAttributes_Metadata {
  SetMetadataSystemAttributes_Metadata(SetMetadataSystemAttributes_Metadata),
}
impl ISetMetadataSystemAttributes_Metadata {
}
#[derive(Serialize, Deserialize)]
pub enum IPlan_WorkflowExecution {
}
impl IPlan_WorkflowExecution {
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowState {
  id: String,
  type_: WorkflowStateType,
  name: String,
  description: String,
  configuration: Value,
  workflow_id: Option<String>,
  entry_workflow_id: Option<String>,
  exit_workflow_id: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub enum ISetCollectionWorkflowState_Collection {
  SetCollectionWorkflowState_Collection(SetCollectionWorkflowState_Collection),
}
impl ISetCollectionWorkflowState_Collection {
}
#[derive(Serialize, Deserialize)]
pub enum IAddCollection_Content {
  AddCollection_Content(AddCollection_Content),
}
impl IAddCollection_Content {
  fn collection(&self) -> &Option<AddCollection_Collection> {
    match self {
      IAddCollection_Content::AddCollection_Content(m) => &m.collection,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISignedUrlHeader {
  SignedUrlHeader(SignedUrlHeader),
  MetadataDownloadUrl_SignedUrlHeader(MetadataDownloadUrl_SignedUrlHeader),
  MetadataUploadUrl_SignedUrlHeader(MetadataUploadUrl_SignedUrlHeader),
  SupplementaryDownloadUrl_SignedUrlHeader(SupplementaryDownloadUrl_SignedUrlHeader),
  SupplementaryUploadUrl_SignedUrlHeader(SupplementaryUploadUrl_SignedUrlHeader),
}
impl ISignedUrlHeader {
  fn value(&self) -> &String {
    match self {
      ISignedUrlHeader::SignedUrlHeader(m) => &m.value,
      ISignedUrlHeader::MetadataDownloadUrl_SignedUrlHeader(m) => &m.value,
      ISignedUrlHeader::MetadataUploadUrl_SignedUrlHeader(m) => &m.value,
      ISignedUrlHeader::SupplementaryDownloadUrl_SignedUrlHeader(m) => &m.value,
      ISignedUrlHeader::SupplementaryUploadUrl_SignedUrlHeader(m) => &m.value,
    }
  }
  fn name(&self) -> &String {
    match self {
      ISignedUrlHeader::SignedUrlHeader(m) => &m.name,
      ISignedUrlHeader::MetadataDownloadUrl_SignedUrlHeader(m) => &m.name,
      ISignedUrlHeader::MetadataUploadUrl_SignedUrlHeader(m) => &m.name,
      ISignedUrlHeader::SupplementaryDownloadUrl_SignedUrlHeader(m) => &m.name,
      ISignedUrlHeader::SupplementaryUploadUrl_SignedUrlHeader(m) => &m.name,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct GetCollection_Query {
  content: GetCollection_Content,
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowJobIdInput {
  WorkflowJobIdInput(WorkflowJobIdInput),
}
impl IWorkflowJobIdInput {
  fn index(&self) -> &i64 {
    match self {
      IWorkflowJobIdInput::WorkflowJobIdInput(m) => &m.index,
    }
  }
  fn id(&self) -> &i64 {
    match self {
      IWorkflowJobIdInput::WorkflowJobIdInput(m) => &m.id,
    }
  }
  fn queue(&self) -> &String {
    match self {
      IWorkflowJobIdInput::WorkflowJobIdInput(m) => &m.queue,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IQueuesMutation {
  QueuesMutation(QueuesMutation),
}
impl IQueuesMutation {
  fn retry(&self) -> &Option<Message> {
    match self {
      IQueuesMutation::QueuesMutation(m) => &m.retry,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct FindCollection_Collection {
  id: String,
  type_: CollectionType,
  name: String,
  labels: Vec<String>,
  attributes: Value,
  created: DateTime<Utc>,
  modified: DateTime<Utc>,
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowExecutionIdInput {
  queue: String,
  id: i64,
}
#[derive(Serialize, Deserialize)]
pub struct SetCollectionWorkflowStateComplete_Content {
  collection: Option<SetCollectionWorkflowStateComplete_Collection>,
}
#[derive(Serialize, Deserialize)]
pub struct AddMetadataBulk_Query {
  content: AddMetadataBulk_Content,
}
#[derive(Serialize, Deserialize)]
pub struct TraitById_Trait {
  id: String,
  workflow_ids: Vec<String>,
}
#[derive(Serialize, Deserialize)]
pub enum ISetExecutionPlanJobCheckin_Query {
  SetExecutionPlanJobCheckin_Query(SetExecutionPlanJobCheckin_Query),
}
impl ISetExecutionPlanJobCheckin_Query {
  fn workflows(&self) -> &SetExecutionPlanJobCheckin_Workflows {
    match self {
      ISetExecutionPlanJobCheckin_Query::SetExecutionPlanJobCheckin_Query(m) => &m.workflows,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IFindMetadata_MetadataSource {
  FindMetadata_MetadataSource(FindMetadata_MetadataSource),
}
impl IFindMetadata_MetadataSource {
  fn id(&self) -> &Option<String> {
    match self {
      IFindMetadata_MetadataSource::FindMetadata_MetadataSource(m) => &m.id,
    }
  }
  fn identifier(&self) -> &Option<String> {
    match self {
      IFindMetadata_MetadataSource::FindMetadata_MetadataSource(m) => &m.identifier,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IModelsMutation {
  ModelsMutation(ModelsMutation),
}
impl IModelsMutation {
  fn add(&self) -> &Option<Model> {
    match self {
      IModelsMutation::ModelsMutation(m) => &m.add,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct AddCollection_Collection {
}
#[derive(Serialize, Deserialize)]
pub enum ISetMetadataSystemAttributes_Query {
  SetMetadataSystemAttributes_Query(SetMetadataSystemAttributes_Query),
}
impl ISetMetadataSystemAttributes_Query {
  fn content(&self) -> &SetMetadataSystemAttributes_Content {
    match self {
      ISetMetadataSystemAttributes_Query::SetMetadataSystemAttributes_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SupplementaryUploadUrl_MetadataSupplementaryContent {
  type_: String,
  urls: SupplementaryUploadUrl_MetadataSupplementaryContentUrls,
}
#[derive(Serialize, Deserialize)]
pub enum ILogin_Security {
  Login_Security(Login_Security),
}
impl ILogin_Security {
  fn login(&self) -> &Login_Login {
    match self {
      ILogin_Security::Login_Security(m) => &m.login,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SetCollectionPublicList_Query {
  content: SetCollectionPublicList_Content,
}
#[derive(Serialize, Deserialize)]
pub struct SupplementaryDownloadUrl_SignedUrl {
  url: String,
  headers: Vec<SupplementaryDownloadUrl_SignedUrlHeader>,
}
#[derive(Serialize, Deserialize)]
pub struct Metadata {
  id: String,
  parent_id: Option<String>,
  version: i64,
  trait_ids: Vec<String>,
  type_: MetadataType,
  name: String,
  content: MetadataContent,
  language_tag: String,
  labels: Vec<String>,
  attributes: Value,
  item_attributes: Option<Value>,
  system_attributes: Option<Value>,
  created: DateTime<Utc>,
  modified: DateTime<Utc>,
  uploaded: Option<DateTime<Utc>>,
  ready: Option<DateTime<Utc>>,
  workflow: MetadataWorkflow,
  source: MetadataSource,
  public: bool,
  public_content: bool,
  public_supplementary: bool,
  permissions: Vec<Permission>,
  relationships: Vec<MetadataRelationship>,
  supplementary: Vec<MetadataSupplementary>,
  parent_collections: Vec<Collection>,
}
#[derive(Serialize, Deserialize)]
pub struct PromptInput {
  name: String,
  description: String,
  system_prompt: String,
  user_prompt: String,
  input_type: String,
  output_type: String,
}
#[derive(Serialize, Deserialize)]
pub struct Transition {
  from_state_id: String,
  to_state_id: String,
  description: String,
}
#[derive(Serialize, Deserialize)]
pub struct MetadataInput {
  parent_collection_id: Option<String>,
  parent_id: Option<String>,
  version: Option<i64>,
  metadata_type: Option<MetadataType>,
  name: String,
  content_type: String,
  content_length: Option<i64>,
  language_tag: String,
  labels: Vec<String>,
  trait_ids: Vec<String>,
  category_ids: Vec<String>,
  attributes: Option<Value>,
  state: Option<MetadataWorkflowInput>,
  source: Option<MetadataSourceInput>,
  index: Option<bool>,
  ready: Option<bool>,
}
#[derive(Serialize, Deserialize)]
pub enum IGetCollection_Query {
  GetCollection_Query(GetCollection_Query),
}
impl IGetCollection_Query {
  fn content(&self) -> &GetCollection_Content {
    match self {
      IGetCollection_Query::GetCollection_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowActivityStorageSystemInput {
  system_id: String,
  configuration: Value,
}
#[derive(Serialize, Deserialize)]
pub enum ISetWorkflowJobFailed_Query {
  SetWorkflowJobFailed_Query(SetWorkflowJobFailed_Query),
}
impl ISetWorkflowJobFailed_Query {
  fn workflows(&self) -> &SetWorkflowJobFailed_Workflows {
    match self {
      ISetWorkflowJobFailed_Query::SetWorkflowJobFailed_Query(m) => &m.workflows,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IGetCollection_Collection {
  GetCollection_Collection(GetCollection_Collection),
}
impl IGetCollection_Collection {
  fn labels(&self) -> &Vec<String> {
    match self {
      IGetCollection_Collection::GetCollection_Collection(m) => &m.labels,
    }
  }
  fn created(&self) -> &DateTime<Utc> {
    match self {
      IGetCollection_Collection::GetCollection_Collection(m) => &m.created,
    }
  }
  fn attributes(&self) -> &Value {
    match self {
      IGetCollection_Collection::GetCollection_Collection(m) => &m.attributes,
    }
  }
  fn modified(&self) -> &DateTime<Utc> {
    match self {
      IGetCollection_Collection::GetCollection_Collection(m) => &m.modified,
    }
  }
  fn id(&self) -> &String {
    match self {
      IGetCollection_Collection::GetCollection_Collection(m) => &m.id,
    }
  }
  fn name(&self) -> &String {
    match self {
      IGetCollection_Collection::GetCollection_Collection(m) => &m.name,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IGroup {
  Group(Group),
}
impl IGroup {
  fn name(&self) -> &String {
    match self {
      IGroup::Group(m) => &m.name,
    }
  }
  fn id(&self) -> &String {
    match self {
      IGroup::Group(m) => &m.id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISetCollectionPublicList_Query {
  SetCollectionPublicList_Query(SetCollectionPublicList_Query),
}
impl ISetCollectionPublicList_Query {
  fn content(&self) -> &SetCollectionPublicList_Content {
    match self {
      ISetCollectionPublicList_Query::SetCollectionPublicList_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISetCollectionWorkflowState_Content {
  SetCollectionWorkflowState_Content(SetCollectionWorkflowState_Content),
}
impl ISetCollectionWorkflowState_Content {
  fn collection(&self) -> &Option<SetCollectionWorkflowState_Collection> {
    match self {
      ISetCollectionWorkflowState_Content::SetCollectionWorkflowState_Content(m) => &m.collection,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct MetadataContent {
  type_: String,
  length: Option<i64>,
  urls: MetadataContentUrls,
  text: String,
  json: Value,
}
#[derive(Serialize, Deserialize)]
pub enum ISetWorkflowState_Query {
  SetWorkflowState_Query(SetWorkflowState_Query),
}
impl ISetWorkflowState_Query {
  fn content(&self) -> &SetWorkflowState_Content {
    match self {
      ISetWorkflowState_Query::SetWorkflowState_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowActivityInput {
  activity_id: String,
  queue: String,
  execution_group: i64,
  description: String,
  inputs: WorkflowActivityParameterInput,
  outputs: WorkflowActivityParameterInput,
  models: WorkflowActivityModelInput,
  storage_systems: WorkflowActivityStorageSystemInput,
  prompts: WorkflowActivityPromptInput,
  configuration: Value,
}
#[derive(Serialize, Deserialize)]
pub struct StorageSystemModel {
  model_id: String,
  model: Option<Model>,
  configuration: Value,
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowActivityStorageSystemInput {
  WorkflowActivityStorageSystemInput(WorkflowActivityStorageSystemInput),
}
impl IWorkflowActivityStorageSystemInput {
  fn system_id(&self) -> &String {
    match self {
      IWorkflowActivityStorageSystemInput::WorkflowActivityStorageSystemInput(m) => &m.system_id,
    }
  }
  fn configuration(&self) -> &Value {
    match self {
      IWorkflowActivityStorageSystemInput::WorkflowActivityStorageSystemInput(m) => &m.configuration,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SetMetadataAttributes_Query {
  content: SetMetadataAttributes_Content,
}
#[derive(Serialize, Deserialize)]
pub enum ICollectionWorkflowInput {
  CollectionWorkflowInput(CollectionWorkflowInput),
}
impl ICollectionWorkflowInput {
  fn state(&self) -> &String {
    match self {
      ICollectionWorkflowInput::CollectionWorkflowInput(m) => &m.state,
    }
  }
  fn delete_workflow_id(&self) -> &Option<String> {
    match self {
      ICollectionWorkflowInput::CollectionWorkflowInput(m) => &m.delete_workflow_id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SetMetadataAttributes_Metadata {
}
#[derive(Serialize, Deserialize)]
pub enum ILogin_Token {
  Login_Token(Login_Token),
}
impl ILogin_Token {
  fn token(&self) -> &String {
    match self {
      ILogin_Token::Login_Token(m) => &m.token,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct MetadataDownloadUrl_SignedUrl {
  url: String,
  headers: Vec<MetadataDownloadUrl_SignedUrlHeader>,
}
#[derive(Serialize, Deserialize)]
pub struct AddMetadata_Metadata {
}
#[derive(Serialize, Deserialize)]
pub enum IAddChildMetadata_Collection {
  AddChildMetadata_Collection(AddChildMetadata_Collection),
}
impl IAddChildMetadata_Collection {
}
#[derive(Serialize, Deserialize)]
pub struct MetadataMutation {
  add: Metadata,
  edit: Metadata,
  add_bulk: Vec<Metadata>,
  delete: bool,
  delete_content: bool,
  add_search_documents: bool,
  add_category: bool,
  delete_category: bool,
  add_trait: Vec<WorkflowExecutionPlan>,
  delete_trait: Option<WorkflowExecutionPlan>,
  set_public: Metadata,
  set_public_content: Metadata,
  set_public_supplementary: Metadata,
  add_permission: Permission,
  delete_permission: Permission,
  add_supplementary: MetadataSupplementary,
  delete_supplementary: bool,
  set_supplementary_uploaded: bool,
  add_relationship: MetadataRelationship,
  edit_relationship: bool,
  delete_relationship: bool,
  set_workflow_state: bool,
  set_workflow_state_complete: bool,
  set_metadata_attributes: bool,
  set_metadata_system_attributes: bool,
  set_metadata_contents: bool,
  set_metadata_uploaded: bool,
  set_metadata_ready: bool,
}
#[derive(Serialize, Deserialize)]
pub struct ActivityParameterInput {
  name: String,
  type_: ActivityParameterType,
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataChildInput {
  MetadataChildInput(MetadataChildInput),
}
impl IMetadataChildInput {
  fn metadata(&self) -> &MetadataInput {
    match self {
      IMetadataChildInput::MetadataChildInput(m) => &m.metadata,
    }
  }
  fn attributes(&self) -> &Option<Value> {
    match self {
      IMetadataChildInput::MetadataChildInput(m) => &m.attributes,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct Trait {
  id: String,
  name: String,
  description: String,
  workflow_ids: Vec<String>,
  workflows: Vec<Workflow>,
}
#[derive(Serialize, Deserialize)]
pub struct Permission {
  group_id: String,
  group: Group,
  action: PermissionAction,
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowInput {
  WorkflowInput(WorkflowInput),
}
impl IWorkflowInput {
  fn activities(&self) -> &WorkflowActivityInput {
    match self {
      IWorkflowInput::WorkflowInput(m) => &m.activities,
    }
  }
  fn queue(&self) -> &String {
    match self {
      IWorkflowInput::WorkflowInput(m) => &m.queue,
    }
  }
  fn id(&self) -> &String {
    match self {
      IWorkflowInput::WorkflowInput(m) => &m.id,
    }
  }
  fn description(&self) -> &String {
    match self {
      IWorkflowInput::WorkflowInput(m) => &m.description,
    }
  }
  fn configuration(&self) -> &Value {
    match self {
      IWorkflowInput::WorkflowInput(m) => &m.configuration,
    }
  }
  fn name(&self) -> &String {
    match self {
      IWorkflowInput::WorkflowInput(m) => &m.name,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IAddMetadata_Query {
  AddMetadata_Query(AddMetadata_Query),
}
impl IAddMetadata_Query {
  fn content(&self) -> &AddMetadata_Content {
    match self {
      IAddMetadata_Query::AddMetadata_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IGetMetadata_Query {
  GetMetadata_Query(GetMetadata_Query),
}
impl IGetMetadata_Query {
  fn content(&self) -> &GetMetadata_Content {
    match self {
      IGetMetadata_Query::GetMetadata_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct MetadataSupplementaryInput {
  metadata_id: String,
  key: String,
  name: String,
  content_type: String,
  content_length: Option<i64>,
  source_id: Option<String>,
  source_identifier: Option<String>,
  attributes: Option<Value>,
}
#[derive(Serialize, Deserialize)]
pub enum IEnqueueJob_Query {
  EnqueueJob_Query(EnqueueJob_Query),
}
impl IEnqueueJob_Query {
  fn workflows(&self) -> &EnqueueJob_Workflows {
    match self {
      IEnqueueJob_Query::EnqueueJob_Query(m) => &m.workflows,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IActivities {
  Activities(Activities),
}
impl IActivities {
  fn all(&self) -> &Vec<Activity> {
    match self {
      IActivities::Activities(m) => &m.all,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IModelInput {
  ModelInput(ModelInput),
}
impl IModelInput {
  fn type_(&self) -> &String {
    match self {
      IModelInput::ModelInput(m) => &m.type_,
    }
  }
  fn description(&self) -> &String {
    match self {
      IModelInput::ModelInput(m) => &m.description,
    }
  }
  fn name(&self) -> &String {
    match self {
      IModelInput::ModelInput(m) => &m.name,
    }
  }
  fn configuration(&self) -> &Value {
    match self {
      IModelInput::ModelInput(m) => &m.configuration,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct Group {
  id: String,
  name: String,
}
#[derive(Serialize, Deserialize)]
pub enum ISetCollectionPublicList_Content {
  SetCollectionPublicList_Content(SetCollectionPublicList_Content),
}
impl ISetCollectionPublicList_Content {
  fn collection(&self) -> &Option<SetCollectionPublicList_Collection> {
    match self {
      ISetCollectionPublicList_Content::SetCollectionPublicList_Content(m) => &m.collection,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataContentUrls {
  MetadataContentUrls(MetadataContentUrls),
  MetadataDownloadUrl_MetadataContentUrls(MetadataDownloadUrl_MetadataContentUrls),
  MetadataUploadUrl_MetadataContentUrls(MetadataUploadUrl_MetadataContentUrls),
}
impl IMetadataContentUrls {
}
#[derive(Serialize, Deserialize)]
pub struct SearchQuery {
  storage_system_id: String,
  query: String,
  filter: Option<String>,
  offset: Option<i64>,
  limit: Option<i64>,
}
#[derive(Serialize, Deserialize)]
pub enum IFindMetadata_MetadataContent {
  FindMetadata_MetadataContent(FindMetadata_MetadataContent),
}
impl IFindMetadata_MetadataContent {
  fn length(&self) -> &Option<i64> {
    match self {
      IFindMetadata_MetadataContent::FindMetadata_MetadataContent(m) => &m.length,
    }
  }
  fn type_(&self) -> &String {
    match self {
      IFindMetadata_MetadataContent::FindMetadata_MetadataContent(m) => &m.type_,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISearchQuery {
  SearchQuery(SearchQuery),
}
impl ISearchQuery {
  fn offset(&self) -> &Option<i64> {
    match self {
      ISearchQuery::SearchQuery(m) => &m.offset,
    }
  }
  fn storage_system_id(&self) -> &String {
    match self {
      ISearchQuery::SearchQuery(m) => &m.storage_system_id,
    }
  }
  fn limit(&self) -> &Option<i64> {
    match self {
      ISearchQuery::SearchQuery(m) => &m.limit,
    }
  }
  fn query(&self) -> &String {
    match self {
      ISearchQuery::SearchQuery(m) => &m.query,
    }
  }
  fn filter(&self) -> &Option<String> {
    match self {
      ISearchQuery::SearchQuery(m) => &m.filter,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISetWorkflowStateComplete_Content {
  SetWorkflowStateComplete_Content(SetWorkflowStateComplete_Content),
}
impl ISetWorkflowStateComplete_Content {
  fn metadata(&self) -> &Option<SetWorkflowStateComplete_Metadata> {
    match self {
      ISetWorkflowStateComplete_Content::SetWorkflowStateComplete_Content(m) => &m.metadata,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct QueuesMutation {
  retry: Option<Message>,
}
#[derive(Serialize, Deserialize)]
pub enum IGetMetadata_Content {
  GetMetadata_Content(GetMetadata_Content),
}
impl IGetMetadata_Content {
  fn metadata(&self) -> &Option<GetMetadata_Metadata> {
    match self {
      IGetMetadata_Content::GetMetadata_Content(m) => &m.metadata,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct BeginTransitionInput {
  collection_id: Option<String>,
  metadata_id: Option<String>,
  version: Option<i64>,
  state_id: String,
  status: String,
  supplementary_id: Option<String>,
  wait_for_completion: Option<bool>,
}
#[derive(Serialize, Deserialize)]
pub enum IStorageSystemModel {
  StorageSystemModel(StorageSystemModel),
}
impl IStorageSystemModel {
  fn model_id(&self) -> &String {
    match self {
      IStorageSystemModel::StorageSystemModel(m) => &m.model_id,
    }
  }
  fn model(&self) -> &Option<Model> {
    match self {
      IStorageSystemModel::StorageSystemModel(m) => &m.model,
    }
  }
  fn configuration(&self) -> &Value {
    match self {
      IStorageSystemModel::StorageSystemModel(m) => &m.configuration,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IActivityParameterInput {
  ActivityParameterInput(ActivityParameterInput),
}
impl IActivityParameterInput {
  fn name(&self) -> &String {
    match self {
      IActivityParameterInput::ActivityParameterInput(m) => &m.name,
    }
  }
  fn type_(&self) -> &ActivityParameterType {
    match self {
      IActivityParameterInput::ActivityParameterInput(m) => &m.type_,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SignedUrl {
  url: String,
  headers: Vec<SignedUrlHeader>,
}
#[derive(Serialize, Deserialize)]
pub enum ITraitById_Trait {
  TraitById_Trait(TraitById_Trait),
}
impl ITraitById_Trait {
  fn id(&self) -> &String {
    match self {
      ITraitById_Trait::TraitById_Trait(m) => &m.id,
    }
  }
  fn workflow_ids(&self) -> &Vec<String> {
    match self {
      ITraitById_Trait::TraitById_Trait(m) => &m.workflow_ids,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataInput {
  MetadataInput(MetadataInput),
}
impl IMetadataInput {
  fn labels(&self) -> &Vec<String> {
    match self {
      IMetadataInput::MetadataInput(m) => &m.labels,
    }
  }
  fn trait_ids(&self) -> &Vec<String> {
    match self {
      IMetadataInput::MetadataInput(m) => &m.trait_ids,
    }
  }
  fn source(&self) -> &Option<MetadataSourceInput> {
    match self {
      IMetadataInput::MetadataInput(m) => &m.source,
    }
  }
  fn parent_id(&self) -> &Option<String> {
    match self {
      IMetadataInput::MetadataInput(m) => &m.parent_id,
    }
  }
  fn state(&self) -> &Option<MetadataWorkflowInput> {
    match self {
      IMetadataInput::MetadataInput(m) => &m.state,
    }
  }
  fn attributes(&self) -> &Option<Value> {
    match self {
      IMetadataInput::MetadataInput(m) => &m.attributes,
    }
  }
  fn parent_collection_id(&self) -> &Option<String> {
    match self {
      IMetadataInput::MetadataInput(m) => &m.parent_collection_id,
    }
  }
  fn category_ids(&self) -> &Vec<String> {
    match self {
      IMetadataInput::MetadataInput(m) => &m.category_ids,
    }
  }
  fn version(&self) -> &Option<i64> {
    match self {
      IMetadataInput::MetadataInput(m) => &m.version,
    }
  }
  fn ready(&self) -> &Option<bool> {
    match self {
      IMetadataInput::MetadataInput(m) => &m.ready,
    }
  }
  fn name(&self) -> &String {
    match self {
      IMetadataInput::MetadataInput(m) => &m.name,
    }
  }
  fn metadata_type(&self) -> &Option<MetadataType> {
    match self {
      IMetadataInput::MetadataInput(m) => &m.metadata_type,
    }
  }
  fn content_type(&self) -> &String {
    match self {
      IMetadataInput::MetadataInput(m) => &m.content_type,
    }
  }
  fn index(&self) -> &Option<bool> {
    match self {
      IMetadataInput::MetadataInput(m) => &m.index,
    }
  }
  fn language_tag(&self) -> &String {
    match self {
      IMetadataInput::MetadataInput(m) => &m.language_tag,
    }
  }
  fn content_length(&self) -> &Option<i64> {
    match self {
      IMetadataInput::MetadataInput(m) => &m.content_length,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowActivityModel {
  WorkflowActivityModel(WorkflowActivityModel),
}
impl IWorkflowActivityModel {
  fn model(&self) -> &Model {
    match self {
      IWorkflowActivityModel::WorkflowActivityModel(m) => &m.model,
    }
  }
  fn configuration(&self) -> &Value {
    match self {
      IWorkflowActivityModel::WorkflowActivityModel(m) => &m.configuration,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowActivityPrompt {
  WorkflowActivityPrompt(WorkflowActivityPrompt),
}
impl IWorkflowActivityPrompt {
  fn prompt(&self) -> &Prompt {
    match self {
      IWorkflowActivityPrompt::WorkflowActivityPrompt(m) => &m.prompt,
    }
  }
  fn configuration(&self) -> &Value {
    match self {
      IWorkflowActivityPrompt::WorkflowActivityPrompt(m) => &m.configuration,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct StorageSystems {
  all: Vec<StorageSystem>,
  storage_system: Option<StorageSystem>,
}
#[derive(Serialize, Deserialize)]
pub enum ICollectionType {
  CollectionType(CollectionType),
}
impl ICollectionType {
}
#[derive(Serialize, Deserialize)]
pub struct AddMetadataBulk_Content {
  metadata: Option<AddMetadataBulk_Metadata>,
}
#[derive(Serialize, Deserialize)]
pub enum IPlan_Workflows {
  Plan_Workflows(Plan_Workflows),
}
impl IPlan_Workflows {
  fn next_workflow_execution(&self) -> &Option<Plan_WorkflowExecution> {
    match self {
      IPlan_Workflows::Plan_Workflows(m) => &m.next_workflow_execution,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISetCollectionReady_Content {
  SetCollectionReady_Content(SetCollectionReady_Content),
}
impl ISetCollectionReady_Content {
  fn collection(&self) -> &Option<SetCollectionReady_Collection> {
    match self {
      ISetCollectionReady_Content::SetCollectionReady_Content(m) => &m.collection,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowStatesMutation {
  add: Option<WorkflowState>,
}
#[derive(Serialize, Deserialize)]
pub struct EnqueueJob_Query {
  workflows: EnqueueJob_Workflows,
}
#[derive(Serialize, Deserialize)]
pub struct MetadataSupplementaryContent {
  type_: String,
  length: Option<i64>,
  urls: MetadataSupplementaryContentUrls,
  text: String,
  json: Value,
}
#[derive(Serialize, Deserialize)]
pub enum ICollectionChildInput {
  CollectionChildInput(CollectionChildInput),
}
impl ICollectionChildInput {
  fn collection(&self) -> &CollectionInput {
    match self {
      ICollectionChildInput::CollectionChildInput(m) => &m.collection,
    }
  }
  fn attributes(&self) -> &Option<Value> {
    match self {
      ICollectionChildInput::CollectionChildInput(m) => &m.attributes,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct MetadataDownloadUrl_SignedUrlHeader {
  name: String,
  value: String,
}
#[derive(Serialize, Deserialize)]
pub struct EnqueueChildWorkflows_Query {
  workflows: EnqueueChildWorkflows_Workflows,
}
#[derive(Serialize, Deserialize)]
pub enum IFindMetadata_MetadataSupplementary {
  FindMetadata_MetadataSupplementary(FindMetadata_MetadataSupplementary),
}
impl IFindMetadata_MetadataSupplementary {
  fn content(&self) -> &FindMetadata_MetadataSupplementaryContent {
    match self {
      IFindMetadata_MetadataSupplementary::FindMetadata_MetadataSupplementary(m) => &m.content,
    }
  }
  fn uploaded(&self) -> &Option<String> {
    match self {
      IFindMetadata_MetadataSupplementary::FindMetadata_MetadataSupplementary(m) => &m.uploaded,
    }
  }
  fn key(&self) -> &String {
    match self {
      IFindMetadata_MetadataSupplementary::FindMetadata_MetadataSupplementary(m) => &m.key,
    }
  }
  fn source(&self) -> &FindMetadata_MetadataSupplementarySource {
    match self {
      IFindMetadata_MetadataSupplementary::FindMetadata_MetadataSupplementary(m) => &m.source,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct Login_Query {
  security: Login_Security,
}
#[derive(Serialize, Deserialize)]
pub enum ICollectionWorkflowCompleteState {
  CollectionWorkflowCompleteState(CollectionWorkflowCompleteState),
}
impl ICollectionWorkflowCompleteState {
  fn status(&self) -> &String {
    match self {
      ICollectionWorkflowCompleteState::CollectionWorkflowCompleteState(m) => &m.status,
    }
  }
  fn collection_id(&self) -> &String {
    match self {
      ICollectionWorkflowCompleteState::CollectionWorkflowCompleteState(m) => &m.collection_id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct Login_Principal {
  id: String,
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowExecution {
  WorkflowExecutionPlan(WorkflowExecutionPlan),
  WorkflowJob(WorkflowJob),
}
impl IWorkflowExecution {
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowStateType {
  WorkflowStateType(WorkflowStateType),
}
impl IWorkflowStateType {
}
#[derive(Serialize, Deserialize)]
pub struct SetMetadataPublic_Metadata {
}
#[derive(Serialize, Deserialize)]
pub struct MetadataUploadUrl_Metadata {
  content: MetadataUploadUrl_MetadataContent,
}
#[derive(Serialize, Deserialize)]
pub enum ISetCollectionPublic_Collection {
  SetCollectionPublic_Collection(SetCollectionPublic_Collection),
}
impl ISetCollectionPublic_Collection {
}
#[derive(Serialize, Deserialize)]
pub struct TraitById_Content {
  trait_: Option<TraitById_Trait>,
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowActivityPromptInput {
  WorkflowActivityPromptInput(WorkflowActivityPromptInput),
}
impl IWorkflowActivityPromptInput {
  fn prompt_id(&self) -> &String {
    match self {
      IWorkflowActivityPromptInput::WorkflowActivityPromptInput(m) => &m.prompt_id,
    }
  }
  fn configuration(&self) -> &Value {
    match self {
      IWorkflowActivityPromptInput::WorkflowActivityPromptInput(m) => &m.configuration,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IAddChildCollection_Collection {
  AddChildCollection_Collection(AddChildCollection_Collection),
}
impl IAddChildCollection_Collection {
}
#[derive(Serialize, Deserialize)]
pub enum IEnqueueJob_Workflows {
  EnqueueJob_Workflows(EnqueueJob_Workflows),
}
impl IEnqueueJob_Workflows {
}
#[derive(Serialize, Deserialize)]
pub enum IEnqueueChildWorkflows_Query {
  EnqueueChildWorkflows_Query(EnqueueChildWorkflows_Query),
}
impl IEnqueueChildWorkflows_Query {
  fn workflows(&self) -> &EnqueueChildWorkflows_Workflows {
    match self {
      IEnqueueChildWorkflows_Query::EnqueueChildWorkflows_Query(m) => &m.workflows,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct MetadataUploadUrl_SignedUrl {
  url: String,
  headers: Vec<MetadataUploadUrl_SignedUrlHeader>,
}
#[derive(Serialize, Deserialize)]
pub struct SupplementaryUploadUrl_MetadataSupplementary {
  metadata_id: String,
  key: String,
  content: SupplementaryUploadUrl_MetadataSupplementaryContent,
}
#[derive(Serialize, Deserialize)]
pub struct MetadataContentUrls {
  download: SignedUrl,
  upload: SignedUrl,
}
#[derive(Serialize, Deserialize)]
pub struct SetWorkflowJobFailed_Workflows {
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataSupplementaryContentUrls {
  MetadataSupplementaryContentUrls(MetadataSupplementaryContentUrls),
  SupplementaryDownloadUrl_MetadataSupplementaryContentUrls(SupplementaryDownloadUrl_MetadataSupplementaryContentUrls),
  SupplementaryUploadUrl_MetadataSupplementaryContentUrls(SupplementaryUploadUrl_MetadataSupplementaryContentUrls),
}
impl IMetadataSupplementaryContentUrls {
}
#[derive(Serialize, Deserialize)]
pub enum ICollectionWorkflow {
  CollectionWorkflow(CollectionWorkflow),
}
impl ICollectionWorkflow {
  fn pending(&self) -> &Option<String> {
    match self {
      ICollectionWorkflow::CollectionWorkflow(m) => &m.pending,
    }
  }
  fn plans(&self) -> &Vec<WorkflowExecutionPlan> {
    match self {
      ICollectionWorkflow::CollectionWorkflow(m) => &m.plans,
    }
  }
  fn delete_workflow(&self) -> &Option<String> {
    match self {
      ICollectionWorkflow::CollectionWorkflow(m) => &m.delete_workflow,
    }
  }
  fn state(&self) -> &String {
    match self {
      ICollectionWorkflow::CollectionWorkflow(m) => &m.state,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IFindMetadata_MetadataSupplementaryContent {
  FindMetadata_MetadataSupplementaryContent(FindMetadata_MetadataSupplementaryContent),
}
impl IFindMetadata_MetadataSupplementaryContent {
  fn type_(&self) -> &String {
    match self {
      IFindMetadata_MetadataSupplementaryContent::FindMetadata_MetadataSupplementaryContent(m) => &m.type_,
    }
  }
  fn length(&self) -> &Option<i64> {
    match self {
      IFindMetadata_MetadataSupplementaryContent::FindMetadata_MetadataSupplementaryContent(m) => &m.length,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataSupplementaryContent {
  MetadataSupplementaryContent(MetadataSupplementaryContent),
  FindMetadata_MetadataSupplementaryContent(FindMetadata_MetadataSupplementaryContent),
  GetMetadata_MetadataSupplementaryContent(GetMetadata_MetadataSupplementaryContent),
  SupplementaryDownloadUrl_MetadataSupplementaryContent(SupplementaryDownloadUrl_MetadataSupplementaryContent),
  SupplementaryUploadUrl_MetadataSupplementaryContent(SupplementaryUploadUrl_MetadataSupplementaryContent),
}
impl IMetadataSupplementaryContent {
}
#[derive(Serialize, Deserialize)]
pub enum CollectionType {
  ROOT,
  STANDARD,
  FOLDER,
  QUEUE,
}
#[derive(Serialize, Deserialize)]
pub struct MetadataDownloadUrl_Query {
  content: MetadataDownloadUrl_Content,
}
#[derive(Serialize, Deserialize)]
pub struct SetCollectionReady_Content {
  collection: Option<SetCollectionReady_Collection>,
}
#[derive(Serialize, Deserialize)]
pub struct ModelInput {
  type_: String,
  name: String,
  description: String,
  configuration: Value,
}
#[derive(Serialize, Deserialize)]
pub struct SignedUrlHeader {
  name: String,
  value: String,
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowJobId {
  WorkflowJobId(WorkflowJobId),
}
impl IWorkflowJobId {
  fn queue(&self) -> &String {
    match self {
      IWorkflowJobId::WorkflowJobId(m) => &m.queue,
    }
  }
  fn id(&self) -> &i64 {
    match self {
      IWorkflowJobId::WorkflowJobId(m) => &m.id,
    }
  }
  fn index(&self) -> &i64 {
    match self {
      IWorkflowJobId::WorkflowJobId(m) => &m.index,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct CollectionWorkflowCompleteState {
  collection_id: String,
  status: String,
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataUploadUrl_MetadataContentUrls {
  MetadataUploadUrl_MetadataContentUrls(MetadataUploadUrl_MetadataContentUrls),
}
impl IMetadataUploadUrl_MetadataContentUrls {
  fn upload(&self) -> &MetadataUploadUrl_SignedUrl {
    match self {
      IMetadataUploadUrl_MetadataContentUrls::MetadataUploadUrl_MetadataContentUrls(m) => &m.upload,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum WorkflowStateType {
  PROCESSING,
  DRAFT,
  PENDING,
  APPROVAL,
  APPROVED,
  PUBLISHED,
  FAILURE,
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowStateInput {
  WorkflowStateInput(WorkflowStateInput),
}
impl IWorkflowStateInput {
  fn exit_workflow_id(&self) -> &Option<String> {
    match self {
      IWorkflowStateInput::WorkflowStateInput(m) => &m.exit_workflow_id,
    }
  }
  fn workflow_id(&self) -> &Option<String> {
    match self {
      IWorkflowStateInput::WorkflowStateInput(m) => &m.workflow_id,
    }
  }
  fn name(&self) -> &String {
    match self {
      IWorkflowStateInput::WorkflowStateInput(m) => &m.name,
    }
  }
  fn id(&self) -> &String {
    match self {
      IWorkflowStateInput::WorkflowStateInput(m) => &m.id,
    }
  }
  fn configuration(&self) -> &Value {
    match self {
      IWorkflowStateInput::WorkflowStateInput(m) => &m.configuration,
    }
  }
  fn type_(&self) -> &WorkflowStateType {
    match self {
      IWorkflowStateInput::WorkflowStateInput(m) => &m.type_,
    }
  }
  fn description(&self) -> &String {
    match self {
      IWorkflowStateInput::WorkflowStateInput(m) => &m.description,
    }
  }
  fn entry_workflow_id(&self) -> &Option<String> {
    match self {
      IWorkflowStateInput::WorkflowStateInput(m) => &m.entry_workflow_id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct AddCollection_Content {
  collection: Option<AddCollection_Collection>,
}
#[derive(Serialize, Deserialize)]
pub struct Message {
  id: i64,
  visible_timeout: DateTime<Utc>,
  value: Value,
}
#[derive(Serialize, Deserialize)]
pub struct FindMetadata_MetadataSupplementaryContent {
  type_: String,
  length: Option<i64>,
}
#[derive(Serialize, Deserialize)]
pub struct MetadataWorkflowInput {
  state: String,
  delete_workflow_id: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub enum ISupplementaryDownloadUrl_SignedUrl {
  SupplementaryDownloadUrl_SignedUrl(SupplementaryDownloadUrl_SignedUrl),
}
impl ISupplementaryDownloadUrl_SignedUrl {
  fn headers(&self) -> &Vec<SupplementaryDownloadUrl_SignedUrlHeader> {
    match self {
      ISupplementaryDownloadUrl_SignedUrl::SupplementaryDownloadUrl_SignedUrl(m) => &m.headers,
    }
  }
  fn url(&self) -> &String {
    match self {
      ISupplementaryDownloadUrl_SignedUrl::SupplementaryDownloadUrl_SignedUrl(m) => &m.url,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISupplementaryUploadUrl_MetadataSupplementary {
  SupplementaryUploadUrl_MetadataSupplementary(SupplementaryUploadUrl_MetadataSupplementary),
}
impl ISupplementaryUploadUrl_MetadataSupplementary {
  fn key(&self) -> &String {
    match self {
      ISupplementaryUploadUrl_MetadataSupplementary::SupplementaryUploadUrl_MetadataSupplementary(m) => &m.key,
    }
  }
  fn content(&self) -> &SupplementaryUploadUrl_MetadataSupplementaryContent {
    match self {
      ISupplementaryUploadUrl_MetadataSupplementary::SupplementaryUploadUrl_MetadataSupplementary(m) => &m.content,
    }
  }
  fn metadata_id(&self) -> &String {
    match self {
      ISupplementaryUploadUrl_MetadataSupplementary::SupplementaryUploadUrl_MetadataSupplementary(m) => &m.metadata_id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IAddCollection_Query {
  AddCollection_Query(AddCollection_Query),
}
impl IAddCollection_Query {
  fn content(&self) -> &AddCollection_Content {
    match self {
      IAddCollection_Query::AddCollection_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct MetadataRelationship {
  id: String,
  metadata: Metadata,
  relationship: String,
  attributes: Value,
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowActivityStorageSystem {
  WorkflowActivityStorageSystem(WorkflowActivityStorageSystem),
}
impl IWorkflowActivityStorageSystem {
  fn system(&self) -> &StorageSystem {
    match self {
      IWorkflowActivityStorageSystem::WorkflowActivityStorageSystem(m) => &m.system,
    }
  }
  fn configuration(&self) -> &Value {
    match self {
      IWorkflowActivityStorageSystem::WorkflowActivityStorageSystem(m) => &m.configuration,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct FindMetadata_Metadata {
  id: String,
  version: i64,
  trait_ids: Vec<String>,
  name: String,
  content: FindMetadata_MetadataContent,
  language_tag: String,
  labels: Vec<String>,
  attributes: Value,
  created: DateTime<Utc>,
  modified: DateTime<Utc>,
  source: FindMetadata_MetadataSource,
  supplementary: Vec<FindMetadata_MetadataSupplementary>,
}
#[derive(Serialize, Deserialize)]
pub enum IAttributesFilterInput {
  AttributesFilterInput(AttributesFilterInput),
}
impl IAttributesFilterInput {
  fn child_attributes(&self) -> &Option<AttributesFilterInput> {
    match self {
      IAttributesFilterInput::AttributesFilterInput(m) => &m.child_attributes,
    }
  }
  fn attributes(&self) -> &Vec<String> {
    match self {
      IAttributesFilterInput::AttributesFilterInput(m) => &m.attributes,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataRelationship {
  MetadataRelationship(MetadataRelationship),
}
impl IMetadataRelationship {
  fn metadata(&self) -> &Metadata {
    match self {
      IMetadataRelationship::MetadataRelationship(m) => &m.metadata,
    }
  }
  fn attributes(&self) -> &Value {
    match self {
      IMetadataRelationship::MetadataRelationship(m) => &m.attributes,
    }
  }
  fn relationship(&self) -> &String {
    match self {
      IMetadataRelationship::MetadataRelationship(m) => &m.relationship,
    }
  }
  fn id(&self) -> &String {
    match self {
      IMetadataRelationship::MetadataRelationship(m) => &m.id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SourceById_Query {
  content: SourceById_Content,
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowActivityParameter {
  WorkflowActivityParameter(WorkflowActivityParameter),
}
impl IWorkflowActivityParameter {
  fn name(&self) -> &String {
    match self {
      IWorkflowActivityParameter::WorkflowActivityParameter(m) => &m.name,
    }
  }
  fn value(&self) -> &String {
    match self {
      IWorkflowActivityParameter::WorkflowActivityParameter(m) => &m.value,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISetMetadataAttributes_Metadata {
  SetMetadataAttributes_Metadata(SetMetadataAttributes_Metadata),
}
impl ISetMetadataAttributes_Metadata {
}
#[derive(Serialize, Deserialize)]
pub struct FindCollection_Query {
  content: FindCollection_Content,
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowActivity {
  id: i64,
  queue: String,
  execution_group: i64,
  configuration: Value,
  inputs: Vec<WorkflowActivityParameter>,
  outputs: Vec<WorkflowActivityParameter>,
}
#[derive(Serialize, Deserialize)]
pub struct MessageQueueStats {
  size: i64,
  pending: i64,
  available: i64,
  min: Option<DateTime<Utc>>,
  max: Option<DateTime<Utc>>,
}
#[derive(Serialize, Deserialize)]
pub struct AddMetadata_Content {
  metadata: Option<AddMetadata_Metadata>,
}
#[derive(Serialize, Deserialize)]
pub struct CollectionWorkflow {
  state: String,
  pending: Option<String>,
  delete_workflow: Option<String>,
  plans: Vec<WorkflowExecutionPlan>,
}
#[derive(Serialize, Deserialize)]
pub enum PermissionAction {
  VIEW,
  EDIT,
  DELETE,
  MANAGE,
  LIST,
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflow {
  Workflow(Workflow),
}
impl IWorkflow {
  fn queue(&self) -> &String {
    match self {
      IWorkflow::Workflow(m) => &m.queue,
    }
  }
  fn id(&self) -> &String {
    match self {
      IWorkflow::Workflow(m) => &m.id,
    }
  }
  fn name(&self) -> &String {
    match self {
      IWorkflow::Workflow(m) => &m.name,
    }
  }
  fn description(&self) -> &String {
    match self {
      IWorkflow::Workflow(m) => &m.description,
    }
  }
  fn configuration(&self) -> &Value {
    match self {
      IWorkflow::Workflow(m) => &m.configuration,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IMessage {
  Message(Message),
}
impl IMessage {
  fn visible_timeout(&self) -> &DateTime<Utc> {
    match self {
      IMessage::Message(m) => &m.visible_timeout,
    }
  }
  fn id(&self) -> &i64 {
    match self {
      IMessage::Message(m) => &m.id,
    }
  }
  fn value(&self) -> &Value {
    match self {
      IMessage::Message(m) => &m.value,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct Prompt {
  id: String,
  name: String,
  description: String,
  system_prompt: String,
  user_prompt: String,
  input_type: String,
  output_type: String,
}
#[derive(Serialize, Deserialize)]
pub struct Source {
  id: String,
  name: String,
  description: String,
  configuration: Value,
}
#[derive(Serialize, Deserialize)]
pub struct GetMetadata_MetadataSupplementary {
  key: String,
  uploaded: Option<String>,
  content: GetMetadata_MetadataSupplementaryContent,
  source: GetMetadata_MetadataSupplementarySource,
}
#[derive(Serialize, Deserialize)]
pub enum ISetWorkflowState_Content {
  SetWorkflowState_Content(SetWorkflowState_Content),
}
impl ISetWorkflowState_Content {
  fn metadata(&self) -> &Option<SetWorkflowState_Metadata> {
    match self {
      ISetWorkflowState_Content::SetWorkflowState_Content(m) => &m.metadata,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISetCollectionWorkflowStateComplete_Collection {
  SetCollectionWorkflowStateComplete_Collection(SetCollectionWorkflowStateComplete_Collection),
}
impl ISetCollectionWorkflowStateComplete_Collection {
}
#[derive(Serialize, Deserialize)]
pub struct Login_LoginResponse {
  principal: Login_Principal,
  token: Login_Token,
}
#[derive(Serialize, Deserialize)]
pub enum IQuery {
  Query(Query),
  AddChildCollection_Query(AddChildCollection_Query),
  AddChildMetadata_Query(AddChildMetadata_Query),
  AddCollection_Query(AddCollection_Query),
  AddMetadata_Query(AddMetadata_Query),
  AddMetadataBulk_Query(AddMetadataBulk_Query),
  AddMetadataSupplementary_Query(AddMetadataSupplementary_Query),
  AddSearchDocuments_Query(AddSearchDocuments_Query),
  GetCollection_Query(GetCollection_Query),
  EnqueueChildWorkflow_Query(EnqueueChildWorkflow_Query),
  EnqueueChildWorkflows_Query(EnqueueChildWorkflows_Query),
  EnqueueJob_Query(EnqueueJob_Query),
  FindCollection_Query(FindCollection_Query),
  FindMetadata_Query(FindMetadata_Query),
  GetCollectionItems_Query(GetCollectionItems_Query),
  Login_Query(Login_Query),
  GetMetadata_Query(GetMetadata_Query),
  MetadataDownloadUrl_Query(MetadataDownloadUrl_Query),
  MetadataUploadUrl_Query(MetadataUploadUrl_Query),
  Plan_Query(Plan_Query),
  SetCollectionPublic_Query(SetCollectionPublic_Query),
  SetCollectionPublicList_Query(SetCollectionPublicList_Query),
  SetCollectionReady_Query(SetCollectionReady_Query),
  SetCollectionWorkflowState_Query(SetCollectionWorkflowState_Query),
  SetCollectionWorkflowStateComplete_Query(SetCollectionWorkflowStateComplete_Query),
  SetMetadataAttributes_Query(SetMetadataAttributes_Query),
  SetMetadataPublic_Query(SetMetadataPublic_Query),
  SetMetadataReady_Query(SetMetadataReady_Query),
  SetMetadataSystemAttributes_Query(SetMetadataSystemAttributes_Query),
  SetExecutionPlanJobCheckin_Query(SetExecutionPlanJobCheckin_Query),
  SetWorkflowJobComplete_Query(SetWorkflowJobComplete_Query),
  SetWorkflowJobContext_Query(SetWorkflowJobContext_Query),
  SetWorkflowJobFailed_Query(SetWorkflowJobFailed_Query),
  SetWorkflowPlanContext_Query(SetWorkflowPlanContext_Query),
  SetWorkflowState_Query(SetWorkflowState_Query),
  SetWorkflowStateComplete_Query(SetWorkflowStateComplete_Query),
  SourceById_Query(SourceById_Query),
  SupplementaryDownloadUrl_Query(SupplementaryDownloadUrl_Query),
  SupplementaryUploadUrl_Query(SupplementaryUploadUrl_Query),
  TraitById_Query(TraitById_Query),
}
impl IQuery {
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowStateInput {
  id: String,
  name: String,
  description: String,
  type_: WorkflowStateType,
  configuration: Value,
  workflow_id: Option<String>,
  entry_workflow_id: Option<String>,
  exit_workflow_id: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub enum ISetWorkflowPlanContext_Workflows {
  SetWorkflowPlanContext_Workflows(SetWorkflowPlanContext_Workflows),
}
impl ISetWorkflowPlanContext_Workflows {
}
#[derive(Serialize, Deserialize)]
pub enum ISupplementaryUploadUrl_Query {
  SupplementaryUploadUrl_Query(SupplementaryUploadUrl_Query),
}
impl ISupplementaryUploadUrl_Query {
  fn content(&self) -> &SupplementaryUploadUrl_Content {
    match self {
      ISupplementaryUploadUrl_Query::SupplementaryUploadUrl_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataSupplementarySource {
  MetadataSupplementarySource(MetadataSupplementarySource),
  FindMetadata_MetadataSupplementarySource(FindMetadata_MetadataSupplementarySource),
  GetMetadata_MetadataSupplementarySource(GetMetadata_MetadataSupplementarySource),
}
impl IMetadataSupplementarySource {
  fn identifier(&self) -> &Option<String> {
    match self {
      IMetadataSupplementarySource::MetadataSupplementarySource(m) => &m.identifier,
      IMetadataSupplementarySource::FindMetadata_MetadataSupplementarySource(m) => &m.identifier,
      IMetadataSupplementarySource::GetMetadata_MetadataSupplementarySource(m) => &m.identifier,
    }
  }
  fn id(&self) -> &String {
    match self {
      IMetadataSupplementarySource::MetadataSupplementarySource(m) => &m.id,
      IMetadataSupplementarySource::FindMetadata_MetadataSupplementarySource(m) => &m.id,
      IMetadataSupplementarySource::GetMetadata_MetadataSupplementarySource(m) => &m.id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct AddChildCollection_Collection {
}
#[derive(Serialize, Deserialize)]
pub struct SupplementaryDownloadUrl_MetadataSupplementary {
  metadata_id: String,
  key: String,
  content: SupplementaryDownloadUrl_MetadataSupplementaryContent,
}
#[derive(Serialize, Deserialize)]
pub struct SetCollectionReady_Collection {
}
#[derive(Serialize, Deserialize)]
pub enum IAddChildMetadata_Query {
  AddChildMetadata_Query(AddChildMetadata_Query),
}
impl IAddChildMetadata_Query {
  fn content(&self) -> &AddChildMetadata_Content {
    match self {
      IAddChildMetadata_Query::AddChildMetadata_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IAddSearchDocuments_Content {
  AddSearchDocuments_Content(AddSearchDocuments_Content),
}
impl IAddSearchDocuments_Content {
  fn metadata(&self) -> &Option<AddSearchDocuments_Metadata> {
    match self {
      IAddSearchDocuments_Content::AddSearchDocuments_Content(m) => &m.metadata,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IActivityInput {
  ActivityInput(ActivityInput),
}
impl IActivityInput {
  fn description(&self) -> &String {
    match self {
      IActivityInput::ActivityInput(m) => &m.description,
    }
  }
  fn name(&self) -> &String {
    match self {
      IActivityInput::ActivityInput(m) => &m.name,
    }
  }
  fn child_workflow_id(&self) -> &Option<String> {
    match self {
      IActivityInput::ActivityInput(m) => &m.child_workflow_id,
    }
  }
  fn configuration(&self) -> &Value {
    match self {
      IActivityInput::ActivityInput(m) => &m.configuration,
    }
  }
  fn outputs(&self) -> &ActivityParameterInput {
    match self {
      IActivityInput::ActivityInput(m) => &m.outputs,
    }
  }
  fn id(&self) -> &String {
    match self {
      IActivityInput::ActivityInput(m) => &m.id,
    }
  }
  fn inputs(&self) -> &ActivityParameterInput {
    match self {
      IActivityInput::ActivityInput(m) => &m.inputs,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISource {
  Source(Source),
  SourceById_Source(SourceById_Source),
}
impl ISource {
  fn id(&self) -> &String {
    match self {
      ISource::Source(m) => &m.id,
      ISource::SourceById_Source(m) => &m.id,
    }
  }
  fn name(&self) -> &String {
    match self {
      ISource::Source(m) => &m.name,
      ISource::SourceById_Source(m) => &m.name,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ICollectionWorkflowState {
  CollectionWorkflowState(CollectionWorkflowState),
}
impl ICollectionWorkflowState {
  fn state_id(&self) -> &String {
    match self {
      ICollectionWorkflowState::CollectionWorkflowState(m) => &m.state_id,
    }
  }
  fn status(&self) -> &String {
    match self {
      ICollectionWorkflowState::CollectionWorkflowState(m) => &m.status,
    }
  }
  fn immediate(&self) -> &bool {
    match self {
      ICollectionWorkflowState::CollectionWorkflowState(m) => &m.immediate,
    }
  }
  fn collection_id(&self) -> &String {
    match self {
      ICollectionWorkflowState::CollectionWorkflowState(m) => &m.collection_id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct AddMetadataBulk_Metadata {
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowExecutionId {
  queue: String,
  id: i64,
}
#[derive(Serialize, Deserialize)]
pub struct SetWorkflowState_Metadata {
}
#[derive(Serialize, Deserialize)]
pub struct SearchDocumentInput {
  metadata_id: Option<String>,
  collection_id: Option<String>,
  content: String,
}
#[derive(Serialize, Deserialize)]
pub struct FindCollection_Content {
  find_collection: Vec<FindCollection_Collection>,
}
#[derive(Serialize, Deserialize)]
pub struct MetadataDownloadUrl_MetadataContentUrls {
  download: MetadataDownloadUrl_SignedUrl,
}
#[derive(Serialize, Deserialize)]
pub enum IToken {
  Token(Token),
  Login_Token(Login_Token),
}
impl IToken {
  fn token(&self) -> &String {
    match self {
      IToken::Token(m) => &m.token,
      IToken::Login_Token(m) => &m.token,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowJobIdInput {
  queue: String,
  id: i64,
  index: i64,
}
#[derive(Serialize, Deserialize)]
pub enum ISetMetadataReady_Metadata {
  SetMetadataReady_Metadata(SetMetadataReady_Metadata),
}
impl ISetMetadataReady_Metadata {
}
#[derive(Serialize, Deserialize)]
pub enum IModel {
  Model(Model),
}
impl IModel {
  fn description(&self) -> &String {
    match self {
      IModel::Model(m) => &m.description,
    }
  }
  fn id(&self) -> &String {
    match self {
      IModel::Model(m) => &m.id,
    }
  }
  fn name(&self) -> &String {
    match self {
      IModel::Model(m) => &m.name,
    }
  }
  fn type_(&self) -> &String {
    match self {
      IModel::Model(m) => &m.type_,
    }
  }
  fn configuration(&self) -> &Value {
    match self {
      IModel::Model(m) => &m.configuration,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ITransition {
  Transition(Transition),
}
impl ITransition {
  fn to_state_id(&self) -> &String {
    match self {
      ITransition::Transition(m) => &m.to_state_id,
    }
  }
  fn description(&self) -> &String {
    match self {
      ITransition::Transition(m) => &m.description,
    }
  }
  fn from_state_id(&self) -> &String {
    match self {
      ITransition::Transition(m) => &m.from_state_id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SetWorkflowStateComplete_Metadata {
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataMutation {
  MetadataMutation(MetadataMutation),
}
impl IMetadataMutation {
  fn set_metadata_ready(&self) -> &bool {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.set_metadata_ready,
    }
  }
  fn set_metadata_system_attributes(&self) -> &bool {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.set_metadata_system_attributes,
    }
  }
  fn delete_content(&self) -> &bool {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.delete_content,
    }
  }
  fn delete(&self) -> &bool {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.delete,
    }
  }
  fn set_metadata_uploaded(&self) -> &bool {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.set_metadata_uploaded,
    }
  }
  fn add_bulk(&self) -> &Vec<Metadata> {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.add_bulk,
    }
  }
  fn add_relationship(&self) -> &MetadataRelationship {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.add_relationship,
    }
  }
  fn delete_permission(&self) -> &Permission {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.delete_permission,
    }
  }
  fn edit_relationship(&self) -> &bool {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.edit_relationship,
    }
  }
  fn add(&self) -> &Metadata {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.add,
    }
  }
  fn set_public_supplementary(&self) -> &Metadata {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.set_public_supplementary,
    }
  }
  fn set_workflow_state(&self) -> &bool {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.set_workflow_state,
    }
  }
  fn set_supplementary_uploaded(&self) -> &bool {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.set_supplementary_uploaded,
    }
  }
  fn add_search_documents(&self) -> &bool {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.add_search_documents,
    }
  }
  fn delete_supplementary(&self) -> &bool {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.delete_supplementary,
    }
  }
  fn delete_trait(&self) -> &Option<WorkflowExecutionPlan> {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.delete_trait,
    }
  }
  fn set_public(&self) -> &Metadata {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.set_public,
    }
  }
  fn add_category(&self) -> &bool {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.add_category,
    }
  }
  fn set_metadata_attributes(&self) -> &bool {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.set_metadata_attributes,
    }
  }
  fn delete_category(&self) -> &bool {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.delete_category,
    }
  }
  fn add_permission(&self) -> &Permission {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.add_permission,
    }
  }
  fn set_workflow_state_complete(&self) -> &bool {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.set_workflow_state_complete,
    }
  }
  fn set_metadata_contents(&self) -> &bool {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.set_metadata_contents,
    }
  }
  fn delete_relationship(&self) -> &bool {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.delete_relationship,
    }
  }
  fn edit(&self) -> &Metadata {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.edit,
    }
  }
  fn add_trait(&self) -> &Vec<WorkflowExecutionPlan> {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.add_trait,
    }
  }
  fn add_supplementary(&self) -> &MetadataSupplementary {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.add_supplementary,
    }
  }
  fn set_public_content(&self) -> &Metadata {
    match self {
      IMetadataMutation::MetadataMutation(m) => &m.set_public_content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum Plan_WorkflowExecution {
}
impl Plan_WorkflowExecution {
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataDownloadUrl_SignedUrlHeader {
  MetadataDownloadUrl_SignedUrlHeader(MetadataDownloadUrl_SignedUrlHeader),
}
impl IMetadataDownloadUrl_SignedUrlHeader {
  fn value(&self) -> &String {
    match self {
      IMetadataDownloadUrl_SignedUrlHeader::MetadataDownloadUrl_SignedUrlHeader(m) => &m.value,
    }
  }
  fn name(&self) -> &String {
    match self {
      IMetadataDownloadUrl_SignedUrlHeader::MetadataDownloadUrl_SignedUrlHeader(m) => &m.name,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ICollectionInput {
  CollectionInput(CollectionInput),
}
impl ICollectionInput {
  fn labels(&self) -> &Vec<String> {
    match self {
      ICollectionInput::CollectionInput(m) => &m.labels,
    }
  }
  fn parent_collection_id(&self) -> &Option<String> {
    match self {
      ICollectionInput::CollectionInput(m) => &m.parent_collection_id,
    }
  }
  fn name(&self) -> &String {
    match self {
      ICollectionInput::CollectionInput(m) => &m.name,
    }
  }
  fn metadata(&self) -> &MetadataChildInput {
    match self {
      ICollectionInput::CollectionInput(m) => &m.metadata,
    }
  }
  fn ready(&self) -> &Option<bool> {
    match self {
      ICollectionInput::CollectionInput(m) => &m.ready,
    }
  }
  fn collections(&self) -> &CollectionChildInput {
    match self {
      ICollectionInput::CollectionInput(m) => &m.collections,
    }
  }
  fn index(&self) -> &Option<bool> {
    match self {
      ICollectionInput::CollectionInput(m) => &m.index,
    }
  }
  fn description(&self) -> &Option<String> {
    match self {
      ICollectionInput::CollectionInput(m) => &m.description,
    }
  }
  fn collection_type(&self) -> &Option<CollectionType> {
    match self {
      ICollectionInput::CollectionInput(m) => &m.collection_type,
    }
  }
  fn ordering(&self) -> &Option<Value> {
    match self {
      ICollectionInput::CollectionInput(m) => &m.ordering,
    }
  }
  fn state(&self) -> &Option<CollectionWorkflowInput> {
    match self {
      ICollectionInput::CollectionInput(m) => &m.state,
    }
  }
  fn attributes(&self) -> &Option<Value> {
    match self {
      ICollectionInput::CollectionInput(m) => &m.attributes,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SetMetadataReady_Query {
  content: SetMetadataReady_Content,
}
#[derive(Serialize, Deserialize)]
pub enum IAddMetadata_Content {
  AddMetadata_Content(AddMetadata_Content),
}
impl IAddMetadata_Content {
  fn metadata(&self) -> &Option<AddMetadata_Metadata> {
    match self {
      IAddMetadata_Content::AddMetadata_Content(m) => &m.metadata,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISetWorkflowState_Metadata {
  SetWorkflowState_Metadata(SetWorkflowState_Metadata),
}
impl ISetWorkflowState_Metadata {
}
#[derive(Serialize, Deserialize)]
pub struct Principal {
  id: String,
  groups: Vec<Group>,
}
#[derive(Serialize, Deserialize)]
pub struct SupplementaryDownloadUrl_MetadataSupplementaryContentUrls {
  download: SupplementaryDownloadUrl_SignedUrl,
}
#[derive(Serialize, Deserialize)]
pub struct SetCollectionWorkflowState_Content {
  collection: Option<SetCollectionWorkflowState_Collection>,
}
#[derive(Serialize, Deserialize)]
pub enum IAddChildCollection_Query {
  AddChildCollection_Query(AddChildCollection_Query),
}
impl IAddChildCollection_Query {
  fn content(&self) -> &AddChildCollection_Content {
    match self {
      IAddChildCollection_Query::AddChildCollection_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct MetadataRelationshipInput {
  id1: String,
  id2: String,
  relationship: String,
  attributes: Value,
}
#[derive(Serialize, Deserialize)]
pub struct PromptsMutation {
  add: Option<Prompt>,
  edit: Option<Prompt>,
  delete: bool,
}
#[derive(Serialize, Deserialize)]
pub enum ISecurityMutation {
  SecurityMutation(SecurityMutation),
}
impl ISecurityMutation {
  fn add_principal_group(&self) -> &bool {
    match self {
      ISecurityMutation::SecurityMutation(m) => &m.add_principal_group,
    }
  }
  fn signup(&self) -> &Principal {
    match self {
      ISecurityMutation::SecurityMutation(m) => &m.signup,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISupplementaryDownloadUrl_Content {
  SupplementaryDownloadUrl_Content(SupplementaryDownloadUrl_Content),
}
impl ISupplementaryDownloadUrl_Content {
  fn metadata_supplementary(&self) -> &Option<SupplementaryDownloadUrl_MetadataSupplementary> {
    match self {
      ISupplementaryDownloadUrl_Content::SupplementaryDownloadUrl_Content(m) => &m.metadata_supplementary,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct GetMetadata_MetadataContent {
  type_: String,
  length: Option<i64>,
}
#[derive(Serialize, Deserialize)]
pub struct MetadataDownloadUrl_MetadataContent {
  type_: String,
  urls: MetadataDownloadUrl_MetadataContentUrls,
}
#[derive(Serialize, Deserialize)]
pub struct SetCollectionWorkflowStateComplete_Query {
  content: SetCollectionWorkflowStateComplete_Content,
}
#[derive(Serialize, Deserialize)]
pub struct Workflows {
  all: Vec<Workflow>,
  activities: Activities,
  models: Models,
  prompts: Prompts,
  states: WorkflowStates,
  storage_systems: StorageSystems,
  transitions: Vec<Transition>,
  next_workflow_execution: Option<WorkflowExecution>,
  execution_plan: Option<WorkflowExecutionPlan>,
  executions: Vec<WorkflowExecution>,
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataUploadUrl_SignedUrl {
  MetadataUploadUrl_SignedUrl(MetadataUploadUrl_SignedUrl),
}
impl IMetadataUploadUrl_SignedUrl {
  fn headers(&self) -> &Vec<MetadataUploadUrl_SignedUrlHeader> {
    match self {
      IMetadataUploadUrl_SignedUrl::MetadataUploadUrl_SignedUrl(m) => &m.headers,
    }
  }
  fn url(&self) -> &String {
    match self {
      IMetadataUploadUrl_SignedUrl::MetadataUploadUrl_SignedUrl(m) => &m.url,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataType {
  MetadataType(MetadataType),
}
impl IMetadataType {
}
#[derive(Serialize, Deserialize)]
pub struct AddSearchDocuments_Content {
  metadata: Option<AddSearchDocuments_Metadata>,
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowActivityParameterInput {
  WorkflowActivityParameterInput(WorkflowActivityParameterInput),
}
impl IWorkflowActivityParameterInput {
  fn name(&self) -> &String {
    match self {
      IWorkflowActivityParameterInput::WorkflowActivityParameterInput(m) => &m.name,
    }
  }
  fn value(&self) -> &String {
    match self {
      IWorkflowActivityParameterInput::WorkflowActivityParameterInput(m) => &m.value,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataSupplementaryInput {
  MetadataSupplementaryInput(MetadataSupplementaryInput),
}
impl IMetadataSupplementaryInput {
  fn name(&self) -> &String {
    match self {
      IMetadataSupplementaryInput::MetadataSupplementaryInput(m) => &m.name,
    }
  }
  fn source_id(&self) -> &Option<String> {
    match self {
      IMetadataSupplementaryInput::MetadataSupplementaryInput(m) => &m.source_id,
    }
  }
  fn metadata_id(&self) -> &String {
    match self {
      IMetadataSupplementaryInput::MetadataSupplementaryInput(m) => &m.metadata_id,
    }
  }
  fn attributes(&self) -> &Option<Value> {
    match self {
      IMetadataSupplementaryInput::MetadataSupplementaryInput(m) => &m.attributes,
    }
  }
  fn content_type(&self) -> &String {
    match self {
      IMetadataSupplementaryInput::MetadataSupplementaryInput(m) => &m.content_type,
    }
  }
  fn source_identifier(&self) -> &Option<String> {
    match self {
      IMetadataSupplementaryInput::MetadataSupplementaryInput(m) => &m.source_identifier,
    }
  }
  fn content_length(&self) -> &Option<i64> {
    match self {
      IMetadataSupplementaryInput::MetadataSupplementaryInput(m) => &m.content_length,
    }
  }
  fn key(&self) -> &String {
    match self {
      IMetadataSupplementaryInput::MetadataSupplementaryInput(m) => &m.key,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct AddMetadataSupplementary_Content {
  metadata: Option<AddMetadataSupplementary_Metadata>,
}
#[derive(Serialize, Deserialize)]
pub enum ISetCollectionWorkflowStateComplete_Content {
  SetCollectionWorkflowStateComplete_Content(SetCollectionWorkflowStateComplete_Content),
}
impl ISetCollectionWorkflowStateComplete_Content {
  fn collection(&self) -> &Option<SetCollectionWorkflowStateComplete_Collection> {
    match self {
      ISetCollectionWorkflowStateComplete_Content::SetCollectionWorkflowStateComplete_Content(m) => &m.collection,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IModels {
  Models(Models),
}
impl IModels {
  fn all(&self) -> &Vec<Model> {
    match self {
      IModels::Models(m) => &m.all,
    }
  }
  fn model(&self) -> &Option<Model> {
    match self {
      IModels::Models(m) => &m.model,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IPrompts {
  Prompts(Prompts),
}
impl IPrompts {
  fn prompt(&self) -> &Option<Prompt> {
    match self {
      IPrompts::Prompts(m) => &m.prompt,
    }
  }
  fn all(&self) -> &Vec<Prompt> {
    match self {
      IPrompts::Prompts(m) => &m.all,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISetCollectionPublic_Query {
  SetCollectionPublic_Query(SetCollectionPublic_Query),
}
impl ISetCollectionPublic_Query {
  fn content(&self) -> &SetCollectionPublic_Content {
    match self {
      ISetCollectionPublic_Query::SetCollectionPublic_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IAddMetadataSupplementary_Content {
  AddMetadataSupplementary_Content(AddMetadataSupplementary_Content),
}
impl IAddMetadataSupplementary_Content {
  fn metadata(&self) -> &Option<AddMetadataSupplementary_Metadata> {
    match self {
      IAddMetadataSupplementary_Content::AddMetadataSupplementary_Content(m) => &m.metadata,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IFindCollection_Collection {
  FindCollection_Collection(FindCollection_Collection),
}
impl IFindCollection_Collection {
  fn attributes(&self) -> &Value {
    match self {
      IFindCollection_Collection::FindCollection_Collection(m) => &m.attributes,
    }
  }
  fn id(&self) -> &String {
    match self {
      IFindCollection_Collection::FindCollection_Collection(m) => &m.id,
    }
  }
  fn created(&self) -> &DateTime<Utc> {
    match self {
      IFindCollection_Collection::FindCollection_Collection(m) => &m.created,
    }
  }
  fn modified(&self) -> &DateTime<Utc> {
    match self {
      IFindCollection_Collection::FindCollection_Collection(m) => &m.modified,
    }
  }
  fn type_(&self) -> &CollectionType {
    match self {
      IFindCollection_Collection::FindCollection_Collection(m) => &m.type_,
    }
  }
  fn name(&self) -> &String {
    match self {
      IFindCollection_Collection::FindCollection_Collection(m) => &m.name,
    }
  }
  fn labels(&self) -> &Vec<String> {
    match self {
      IFindCollection_Collection::FindCollection_Collection(m) => &m.labels,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct GetCollectionItems_Query {
  content: GetCollectionItems_Content,
}
#[derive(Serialize, Deserialize)]
pub enum ISupplementaryDownloadUrl_MetadataSupplementaryContentUrls {
  SupplementaryDownloadUrl_MetadataSupplementaryContentUrls(SupplementaryDownloadUrl_MetadataSupplementaryContentUrls),
}
impl ISupplementaryDownloadUrl_MetadataSupplementaryContentUrls {
  fn download(&self) -> &SupplementaryDownloadUrl_SignedUrl {
    match self {
      ISupplementaryDownloadUrl_MetadataSupplementaryContentUrls::SupplementaryDownloadUrl_MetadataSupplementaryContentUrls(m) => &m.download,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISearchDocument {
  SearchDocument(SearchDocument),
}
impl ISearchDocument {
  fn metadata(&self) -> &Option<Metadata> {
    match self {
      ISearchDocument::SearchDocument(m) => &m.metadata,
    }
  }
  fn collection(&self) -> &Option<Collection> {
    match self {
      ISearchDocument::SearchDocument(m) => &m.collection,
    }
  }
  fn content(&self) -> &String {
    match self {
      ISearchDocument::SearchDocument(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IAddMetadataBulk_Metadata {
  AddMetadataBulk_Metadata(AddMetadataBulk_Metadata),
}
impl IAddMetadataBulk_Metadata {
}
#[derive(Serialize, Deserialize)]
pub struct SetWorkflowJobComplete_Query {
  workflows: SetWorkflowJobComplete_Workflows,
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowActivityStorageSystem {
  configuration: Value,
  system: StorageSystem,
}
#[derive(Serialize, Deserialize)]
pub struct GetMetadata_MetadataSource {
  id: Option<String>,
  identifier: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataUploadUrl_SignedUrlHeader {
  MetadataUploadUrl_SignedUrlHeader(MetadataUploadUrl_SignedUrlHeader),
}
impl IMetadataUploadUrl_SignedUrlHeader {
  fn name(&self) -> &String {
    match self {
      IMetadataUploadUrl_SignedUrlHeader::MetadataUploadUrl_SignedUrlHeader(m) => &m.name,
    }
  }
  fn value(&self) -> &String {
    match self {
      IMetadataUploadUrl_SignedUrlHeader::MetadataUploadUrl_SignedUrlHeader(m) => &m.value,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SetExecutionPlanJobCheckin_Workflows {
}
#[derive(Serialize, Deserialize)]
pub enum ISetCollectionReady_Collection {
  SetCollectionReady_Collection(SetCollectionReady_Collection),
}
impl ISetCollectionReady_Collection {
}
#[derive(Serialize, Deserialize)]
pub enum ISetMetadataSystemAttributes_Content {
  SetMetadataSystemAttributes_Content(SetMetadataSystemAttributes_Content),
}
impl ISetMetadataSystemAttributes_Content {
  fn metadata(&self) -> &Option<SetMetadataSystemAttributes_Metadata> {
    match self {
      ISetMetadataSystemAttributes_Content::SetMetadataSystemAttributes_Content(m) => &m.metadata,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISupplementaryUploadUrl_SignedUrlHeader {
  SupplementaryUploadUrl_SignedUrlHeader(SupplementaryUploadUrl_SignedUrlHeader),
}
impl ISupplementaryUploadUrl_SignedUrlHeader {
  fn name(&self) -> &String {
    match self {
      ISupplementaryUploadUrl_SignedUrlHeader::SupplementaryUploadUrl_SignedUrlHeader(m) => &m.name,
    }
  }
  fn value(&self) -> &String {
    match self {
      ISupplementaryUploadUrl_SignedUrlHeader::SupplementaryUploadUrl_SignedUrlHeader(m) => &m.value,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct MetadataUploadUrl_SignedUrlHeader {
  name: String,
  value: String,
}
#[derive(Serialize, Deserialize)]
pub struct SetMetadataSystemAttributes_Metadata {
}
#[derive(Serialize, Deserialize)]
pub enum IFindCollection_Content {
  FindCollection_Content(FindCollection_Content),
}
impl IFindCollection_Content {
  fn find_collection(&self) -> &Vec<FindCollection_Collection> {
    match self {
      IFindCollection_Content::FindCollection_Content(m) => &m.find_collection,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct AddChildMetadata_Collection {
}
#[derive(Serialize, Deserialize)]
pub enum ISetWorkflowPlanContext_Query {
  SetWorkflowPlanContext_Query(SetWorkflowPlanContext_Query),
}
impl ISetWorkflowPlanContext_Query {
  fn workflows(&self) -> &SetWorkflowPlanContext_Workflows {
    match self {
      ISetWorkflowPlanContext_Query::SetWorkflowPlanContext_Query(m) => &m.workflows,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowActivityModelInput {
  model_id: String,
  configuration: Value,
}
#[derive(Serialize, Deserialize)]
pub struct SetExecutionPlanJobCheckin_Query {
  workflows: SetExecutionPlanJobCheckin_Workflows,
}
#[derive(Serialize, Deserialize)]
pub struct SetMetadataSystemAttributes_Query {
  content: SetMetadataSystemAttributes_Content,
}
#[derive(Serialize, Deserialize)]
pub struct SetWorkflowState_Query {
  content: SetWorkflowState_Content,
}
#[derive(Serialize, Deserialize)]
pub enum ISupplementaryDownloadUrl_Query {
  SupplementaryDownloadUrl_Query(SupplementaryDownloadUrl_Query),
}
impl ISupplementaryDownloadUrl_Query {
  fn content(&self) -> &SupplementaryDownloadUrl_Content {
    match self {
      ISupplementaryDownloadUrl_Query::SupplementaryDownloadUrl_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct MetadataWorkflowState {
  metadata_id: String,
  state_id: String,
  status: String,
  immediate: bool,
}
#[derive(Serialize, Deserialize)]
pub enum ISetMetadataAttributes_Content {
  SetMetadataAttributes_Content(SetMetadataAttributes_Content),
}
impl ISetMetadataAttributes_Content {
  fn metadata(&self) -> &Option<SetMetadataAttributes_Metadata> {
    match self {
      ISetMetadataAttributes_Content::SetMetadataAttributes_Content(m) => &m.metadata,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SetWorkflowJobContext_Workflows {
}
#[derive(Serialize, Deserialize)]
pub struct EnqueueChildWorkflow_Workflows {
}
#[derive(Serialize, Deserialize)]
pub struct EnqueueJob_Workflows {
}
#[derive(Serialize, Deserialize)]
pub enum ILogin_Query {
  Login_Query(Login_Query),
}
impl ILogin_Query {
  fn security(&self) -> &Login_Security {
    match self {
      ILogin_Query::Login_Query(m) => &m.security,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct AddCollection_Query {
  content: AddCollection_Content,
}
#[derive(Serialize, Deserialize)]
pub enum ISupplementaryUploadUrl_MetadataSupplementaryContent {
  SupplementaryUploadUrl_MetadataSupplementaryContent(SupplementaryUploadUrl_MetadataSupplementaryContent),
}
impl ISupplementaryUploadUrl_MetadataSupplementaryContent {
  fn type_(&self) -> &String {
    match self {
      ISupplementaryUploadUrl_MetadataSupplementaryContent::SupplementaryUploadUrl_MetadataSupplementaryContent(m) => &m.type_,
    }
  }
  fn urls(&self) -> &SupplementaryUploadUrl_MetadataSupplementaryContentUrls {
    match self {
      ISupplementaryUploadUrl_MetadataSupplementaryContent::SupplementaryUploadUrl_MetadataSupplementaryContent(m) => &m.urls,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SearchDocument {
  metadata: Option<Metadata>,
  collection: Option<Collection>,
  content: String,
}
#[derive(Serialize, Deserialize)]
pub enum IStorageSystems {
  StorageSystems(StorageSystems),
}
impl IStorageSystems {
  fn all(&self) -> &Vec<StorageSystem> {
    match self {
      IStorageSystems::StorageSystems(m) => &m.all,
    }
  }
  fn storage_system(&self) -> &Option<StorageSystem> {
    match self {
      IStorageSystems::StorageSystems(m) => &m.storage_system,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IFindMetadata_Query {
  FindMetadata_Query(FindMetadata_Query),
}
impl IFindMetadata_Query {
  fn content(&self) -> &FindMetadata_Content {
    match self {
      IFindMetadata_Query::FindMetadata_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ILoginResponse {
  LoginResponse(LoginResponse),
  Login_LoginResponse(Login_LoginResponse),
}
impl ILoginResponse {
}
#[derive(Serialize, Deserialize)]
pub struct CollectionChildInput {
  collection: CollectionInput,
  attributes: Option<Value>,
}
#[derive(Serialize, Deserialize)]
pub enum IPromptInput {
  PromptInput(PromptInput),
}
impl IPromptInput {
  fn user_prompt(&self) -> &String {
    match self {
      IPromptInput::PromptInput(m) => &m.user_prompt,
    }
  }
  fn output_type(&self) -> &String {
    match self {
      IPromptInput::PromptInput(m) => &m.output_type,
    }
  }
  fn input_type(&self) -> &String {
    match self {
      IPromptInput::PromptInput(m) => &m.input_type,
    }
  }
  fn name(&self) -> &String {
    match self {
      IPromptInput::PromptInput(m) => &m.name,
    }
  }
  fn description(&self) -> &String {
    match self {
      IPromptInput::PromptInput(m) => &m.description,
    }
  }
  fn system_prompt(&self) -> &String {
    match self {
      IPromptInput::PromptInput(m) => &m.system_prompt,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataDownloadUrl_MetadataContent {
  MetadataDownloadUrl_MetadataContent(MetadataDownloadUrl_MetadataContent),
}
impl IMetadataDownloadUrl_MetadataContent {
  fn type_(&self) -> &String {
    match self {
      IMetadataDownloadUrl_MetadataContent::MetadataDownloadUrl_MetadataContent(m) => &m.type_,
    }
  }
  fn urls(&self) -> &MetadataDownloadUrl_MetadataContentUrls {
    match self {
      IMetadataDownloadUrl_MetadataContent::MetadataDownloadUrl_MetadataContent(m) => &m.urls,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct CollectionWorkflowInput {
  state: String,
  delete_workflow_id: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub enum ILogin_LoginResponse {
  Login_LoginResponse(Login_LoginResponse),
}
impl ILogin_LoginResponse {
  fn token(&self) -> &Login_Token {
    match self {
      ILogin_LoginResponse::Login_LoginResponse(m) => &m.token,
    }
  }
  fn principal(&self) -> &Login_Principal {
    match self {
      ILogin_LoginResponse::Login_LoginResponse(m) => &m.principal,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISetWorkflowJobContext_Query {
  SetWorkflowJobContext_Query(SetWorkflowJobContext_Query),
}
impl ISetWorkflowJobContext_Query {
  fn workflows(&self) -> &SetWorkflowJobContext_Workflows {
    match self {
      ISetWorkflowJobContext_Query::SetWorkflowJobContext_Query(m) => &m.workflows,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SupplementaryUploadUrl_MetadataSupplementaryContentUrls {
  upload: SupplementaryUploadUrl_SignedUrl,
}
#[derive(Serialize, Deserialize)]
pub struct MetadataUploadUrl_Query {
  content: MetadataUploadUrl_Content,
}
#[derive(Serialize, Deserialize)]
pub enum IPromptsMutation {
  PromptsMutation(PromptsMutation),
}
impl IPromptsMutation {
  fn edit(&self) -> &Option<Prompt> {
    match self {
      IPromptsMutation::PromptsMutation(m) => &m.edit,
    }
  }
  fn delete(&self) -> &bool {
    match self {
      IPromptsMutation::PromptsMutation(m) => &m.delete,
    }
  }
  fn add(&self) -> &Option<Prompt> {
    match self {
      IPromptsMutation::PromptsMutation(m) => &m.add,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowExecutionIdInput {
  WorkflowExecutionIdInput(WorkflowExecutionIdInput),
}
impl IWorkflowExecutionIdInput {
  fn id(&self) -> &i64 {
    match self {
      IWorkflowExecutionIdInput::WorkflowExecutionIdInput(m) => &m.id,
    }
  }
  fn queue(&self) -> &String {
    match self {
      IWorkflowExecutionIdInput::WorkflowExecutionIdInput(m) => &m.queue,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISupplementaryUploadUrl_MetadataSupplementaryContentUrls {
  SupplementaryUploadUrl_MetadataSupplementaryContentUrls(SupplementaryUploadUrl_MetadataSupplementaryContentUrls),
}
impl ISupplementaryUploadUrl_MetadataSupplementaryContentUrls {
  fn upload(&self) -> &SupplementaryUploadUrl_SignedUrl {
    match self {
      ISupplementaryUploadUrl_MetadataSupplementaryContentUrls::SupplementaryUploadUrl_MetadataSupplementaryContentUrls(m) => &m.upload,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct ContentMutation {
  collection: CollectionMutation,
  metadata: MetadataMutation,
  reindex: bool,
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataRelationshipInput {
  MetadataRelationshipInput(MetadataRelationshipInput),
}
impl IMetadataRelationshipInput {
  fn attributes(&self) -> &Value {
    match self {
      IMetadataRelationshipInput::MetadataRelationshipInput(m) => &m.attributes,
    }
  }
  fn relationship(&self) -> &String {
    match self {
      IMetadataRelationshipInput::MetadataRelationshipInput(m) => &m.relationship,
    }
  }
  fn id2(&self) -> &String {
    match self {
      IMetadataRelationshipInput::MetadataRelationshipInput(m) => &m.id2,
    }
  }
  fn id1(&self) -> &String {
    match self {
      IMetadataRelationshipInput::MetadataRelationshipInput(m) => &m.id1,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IFindMetadata_Content {
  FindMetadata_Content(FindMetadata_Content),
}
impl IFindMetadata_Content {
  fn find_metadata(&self) -> &Vec<FindMetadata_Metadata> {
    match self {
      IFindMetadata_Content::FindMetadata_Content(m) => &m.find_metadata,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct GetCollectionItems_Content {
  collection: Option<GetCollectionItems_Collection>,
}
#[derive(Serialize, Deserialize)]
pub struct SupplementaryDownloadUrl_Query {
  content: SupplementaryDownloadUrl_Content,
}
#[derive(Serialize, Deserialize)]
pub enum IGetCollectionItems_Query {
  GetCollectionItems_Query(GetCollectionItems_Query),
}
impl IGetCollectionItems_Query {
  fn content(&self) -> &GetCollectionItems_Content {
    match self {
      IGetCollectionItems_Query::GetCollectionItems_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct StorageSystem {
  id: String,
  type_: StorageSystemType,
  name: String,
  description: String,
  configuration: Value,
  models: Vec<StorageSystemModel>,
}
#[derive(Serialize, Deserialize)]
pub struct GetMetadata_MetadataSupplementarySource {
  id: String,
  identifier: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub enum CollectionItem {
  Metadata(Metadata),
  Collection(Collection),
}
impl CollectionItem {
  fn item_attributes(&self) -> &Option<Value> {
    match self {
      CollectionItem::Metadata(m) => &m.item_attributes,
      CollectionItem::Collection(m) => &m.item_attributes,
    }
  }
  fn modified(&self) -> &DateTime<Utc> {
    match self {
      CollectionItem::Metadata(m) => &m.modified,
      CollectionItem::Collection(m) => &m.modified,
    }
  }
  fn name(&self) -> &String {
    match self {
      CollectionItem::Metadata(m) => &m.name,
      CollectionItem::Collection(m) => &m.name,
    }
  }
  fn parent_collections(&self) -> &Vec<Collection> {
    match self {
      CollectionItem::Metadata(m) => &m.parent_collections,
      CollectionItem::Collection(m) => &m.parent_collections,
    }
  }
  fn attributes(&self) -> &Value {
    match self {
      CollectionItem::Metadata(m) => &m.attributes,
      CollectionItem::Collection(m) => &m.attributes,
    }
  }
  fn trait_ids(&self) -> &Vec<String> {
    match self {
      CollectionItem::Metadata(m) => &m.trait_ids,
      CollectionItem::Collection(m) => &m.trait_ids,
    }
  }
  fn ready(&self) -> &Option<DateTime<Utc>> {
    match self {
      CollectionItem::Metadata(m) => &m.ready,
      CollectionItem::Collection(m) => &m.ready,
    }
  }
  fn created(&self) -> &DateTime<Utc> {
    match self {
      CollectionItem::Metadata(m) => &m.created,
      CollectionItem::Collection(m) => &m.created,
    }
  }
  fn id(&self) -> &String {
    match self {
      CollectionItem::Metadata(m) => &m.id,
      CollectionItem::Collection(m) => &m.id,
    }
  }
  fn system_attributes(&self) -> &Option<Value> {
    match self {
      CollectionItem::Metadata(m) => &m.system_attributes,
      CollectionItem::Collection(m) => &m.system_attributes,
    }
  }
  fn permissions(&self) -> &Vec<Permission> {
    match self {
      CollectionItem::Metadata(m) => &m.permissions,
      CollectionItem::Collection(m) => &m.permissions,
    }
  }
  fn labels(&self) -> &Vec<String> {
    match self {
      CollectionItem::Metadata(m) => &m.labels,
      CollectionItem::Collection(m) => &m.labels,
    }
  }
  fn public(&self) -> &bool {
    match self {
      CollectionItem::Metadata(m) => &m.public,
      CollectionItem::Collection(m) => &m.public,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISetMetadataReady_Content {
  SetMetadataReady_Content(SetMetadataReady_Content),
}
impl ISetMetadataReady_Content {
  fn metadata(&self) -> &Option<SetMetadataReady_Metadata> {
    match self {
      ISetMetadataReady_Content::SetMetadataReady_Content(m) => &m.metadata,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct Login {
  password: LoginResponse,
}
#[derive(Serialize, Deserialize)]
pub enum IContentMutation {
  ContentMutation(ContentMutation),
}
impl IContentMutation {
  fn metadata(&self) -> &MetadataMutation {
    match self {
      IContentMutation::ContentMutation(m) => &m.metadata,
    }
  }
  fn collection(&self) -> &CollectionMutation {
    match self {
      IContentMutation::ContentMutation(m) => &m.collection,
    }
  }
  fn reindex(&self) -> &bool {
    match self {
      IContentMutation::ContentMutation(m) => &m.reindex,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IPermissionAction {
  PermissionAction(PermissionAction),
}
impl IPermissionAction {
}
#[derive(Serialize, Deserialize)]
pub enum IFindMetadata_MetadataSupplementarySource {
  FindMetadata_MetadataSupplementarySource(FindMetadata_MetadataSupplementarySource),
}
impl IFindMetadata_MetadataSupplementarySource {
  fn identifier(&self) -> &Option<String> {
    match self {
      IFindMetadata_MetadataSupplementarySource::FindMetadata_MetadataSupplementarySource(m) => &m.identifier,
    }
  }
  fn id(&self) -> &String {
    match self {
      IFindMetadata_MetadataSupplementarySource::FindMetadata_MetadataSupplementarySource(m) => &m.id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SupplementaryDownloadUrl_Content {
  metadata_supplementary: Option<SupplementaryDownloadUrl_MetadataSupplementary>,
}
#[derive(Serialize, Deserialize)]
pub enum ICollectionMutation {
  CollectionMutation(CollectionMutation),
}
impl ICollectionMutation {
  fn set_public(&self) -> &Collection {
    match self {
      ICollectionMutation::CollectionMutation(m) => &m.set_public,
    }
  }
  fn remove_child_metadata(&self) -> &Collection {
    match self {
      ICollectionMutation::CollectionMutation(m) => &m.remove_child_metadata,
    }
  }
  fn set_collection_attributes(&self) -> &bool {
    match self {
      ICollectionMutation::CollectionMutation(m) => &m.set_collection_attributes,
    }
  }
  fn remove_child_collection(&self) -> &Collection {
    match self {
      ICollectionMutation::CollectionMutation(m) => &m.remove_child_collection,
    }
  }
  fn set_child_item_attributes(&self) -> &Collection {
    match self {
      ICollectionMutation::CollectionMutation(m) => &m.set_child_item_attributes,
    }
  }
  fn edit(&self) -> &Collection {
    match self {
      ICollectionMutation::CollectionMutation(m) => &m.edit,
    }
  }
  fn add_bulk(&self) -> &Vec<Collection> {
    match self {
      ICollectionMutation::CollectionMutation(m) => &m.add_bulk,
    }
  }
  fn add_child_metadata(&self) -> &Collection {
    match self {
      ICollectionMutation::CollectionMutation(m) => &m.add_child_metadata,
    }
  }
  fn set_public_list(&self) -> &Collection {
    match self {
      ICollectionMutation::CollectionMutation(m) => &m.set_public_list,
    }
  }
  fn add_permission(&self) -> &Permission {
    match self {
      ICollectionMutation::CollectionMutation(m) => &m.add_permission,
    }
  }
  fn delete(&self) -> &bool {
    match self {
      ICollectionMutation::CollectionMutation(m) => &m.delete,
    }
  }
  fn delete_permission(&self) -> &Permission {
    match self {
      ICollectionMutation::CollectionMutation(m) => &m.delete_permission,
    }
  }
  fn set_workflow_state(&self) -> &bool {
    match self {
      ICollectionMutation::CollectionMutation(m) => &m.set_workflow_state,
    }
  }
  fn add_child_collection(&self) -> &Collection {
    match self {
      ICollectionMutation::CollectionMutation(m) => &m.add_child_collection,
    }
  }
  fn set_collection_ordering(&self) -> &bool {
    match self {
      ICollectionMutation::CollectionMutation(m) => &m.set_collection_ordering,
    }
  }
  fn add(&self) -> &Collection {
    match self {
      ICollectionMutation::CollectionMutation(m) => &m.add,
    }
  }
  fn set_workflow_state_complete(&self) -> &bool {
    match self {
      ICollectionMutation::CollectionMutation(m) => &m.set_workflow_state_complete,
    }
  }
  fn set_ready(&self) -> &bool {
    match self {
      ICollectionMutation::CollectionMutation(m) => &m.set_ready,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IPrompt {
  Prompt(Prompt),
}
impl IPrompt {
  fn name(&self) -> &String {
    match self {
      IPrompt::Prompt(m) => &m.name,
    }
  }
  fn system_prompt(&self) -> &String {
    match self {
      IPrompt::Prompt(m) => &m.system_prompt,
    }
  }
  fn description(&self) -> &String {
    match self {
      IPrompt::Prompt(m) => &m.description,
    }
  }
  fn user_prompt(&self) -> &String {
    match self {
      IPrompt::Prompt(m) => &m.user_prompt,
    }
  }
  fn output_type(&self) -> &String {
    match self {
      IPrompt::Prompt(m) => &m.output_type,
    }
  }
  fn id(&self) -> &String {
    match self {
      IPrompt::Prompt(m) => &m.id,
    }
  }
  fn input_type(&self) -> &String {
    match self {
      IPrompt::Prompt(m) => &m.input_type,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SetCollectionPublic_Query {
  content: SetCollectionPublic_Content,
}
#[derive(Serialize, Deserialize)]
pub struct SetMetadataReady_Content {
  metadata: Option<SetMetadataReady_Metadata>,
}
#[derive(Serialize, Deserialize)]
pub enum IMetadata {
  Metadata(Metadata),
  AddMetadata_Metadata(AddMetadata_Metadata),
  AddMetadataBulk_Metadata(AddMetadataBulk_Metadata),
  AddMetadataSupplementary_Metadata(AddMetadataSupplementary_Metadata),
  AddSearchDocuments_Metadata(AddSearchDocuments_Metadata),
  FindMetadata_Metadata(FindMetadata_Metadata),
  GetMetadata_Metadata(GetMetadata_Metadata),
  MetadataDownloadUrl_Metadata(MetadataDownloadUrl_Metadata),
  MetadataUploadUrl_Metadata(MetadataUploadUrl_Metadata),
  SetMetadataAttributes_Metadata(SetMetadataAttributes_Metadata),
  SetMetadataPublic_Metadata(SetMetadataPublic_Metadata),
  SetMetadataReady_Metadata(SetMetadataReady_Metadata),
  SetMetadataSystemAttributes_Metadata(SetMetadataSystemAttributes_Metadata),
  SetWorkflowState_Metadata(SetWorkflowState_Metadata),
  SetWorkflowStateComplete_Metadata(SetWorkflowStateComplete_Metadata),
}
impl IMetadata {
}
#[derive(Serialize, Deserialize)]
pub enum IAddCollection_Collection {
  AddCollection_Collection(AddCollection_Collection),
}
impl IAddCollection_Collection {
}
#[derive(Serialize, Deserialize)]
pub enum ITraitById_Query {
  TraitById_Query(TraitById_Query),
}
impl ITraitById_Query {
  fn content(&self) -> &TraitById_Content {
    match self {
      ITraitById_Query::TraitById_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflows {
  Workflows(Workflows),
  EnqueueChildWorkflow_Workflows(EnqueueChildWorkflow_Workflows),
  EnqueueChildWorkflows_Workflows(EnqueueChildWorkflows_Workflows),
  EnqueueJob_Workflows(EnqueueJob_Workflows),
  Plan_Workflows(Plan_Workflows),
  SetExecutionPlanJobCheckin_Workflows(SetExecutionPlanJobCheckin_Workflows),
  SetWorkflowJobComplete_Workflows(SetWorkflowJobComplete_Workflows),
  SetWorkflowJobContext_Workflows(SetWorkflowJobContext_Workflows),
  SetWorkflowJobFailed_Workflows(SetWorkflowJobFailed_Workflows),
  SetWorkflowPlanContext_Workflows(SetWorkflowPlanContext_Workflows),
}
impl IWorkflows {
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowActivityInput {
  WorkflowActivityInput(WorkflowActivityInput),
}
impl IWorkflowActivityInput {
  fn configuration(&self) -> &Value {
    match self {
      IWorkflowActivityInput::WorkflowActivityInput(m) => &m.configuration,
    }
  }
  fn queue(&self) -> &String {
    match self {
      IWorkflowActivityInput::WorkflowActivityInput(m) => &m.queue,
    }
  }
  fn description(&self) -> &String {
    match self {
      IWorkflowActivityInput::WorkflowActivityInput(m) => &m.description,
    }
  }
  fn execution_group(&self) -> &i64 {
    match self {
      IWorkflowActivityInput::WorkflowActivityInput(m) => &m.execution_group,
    }
  }
  fn prompts(&self) -> &WorkflowActivityPromptInput {
    match self {
      IWorkflowActivityInput::WorkflowActivityInput(m) => &m.prompts,
    }
  }
  fn storage_systems(&self) -> &WorkflowActivityStorageSystemInput {
    match self {
      IWorkflowActivityInput::WorkflowActivityInput(m) => &m.storage_systems,
    }
  }
  fn models(&self) -> &WorkflowActivityModelInput {
    match self {
      IWorkflowActivityInput::WorkflowActivityInput(m) => &m.models,
    }
  }
  fn inputs(&self) -> &WorkflowActivityParameterInput {
    match self {
      IWorkflowActivityInput::WorkflowActivityInput(m) => &m.inputs,
    }
  }
  fn activity_id(&self) -> &String {
    match self {
      IWorkflowActivityInput::WorkflowActivityInput(m) => &m.activity_id,
    }
  }
  fn outputs(&self) -> &WorkflowActivityParameterInput {
    match self {
      IWorkflowActivityInput::WorkflowActivityInput(m) => &m.outputs,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct Workflow {
  id: String,
  name: String,
  queue: String,
  description: String,
  configuration: Value,
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataSource {
  MetadataSource(MetadataSource),
  FindMetadata_MetadataSource(FindMetadata_MetadataSource),
  GetMetadata_MetadataSource(GetMetadata_MetadataSource),
}
impl IMetadataSource {
  fn id(&self) -> &Option<String> {
    match self {
      IMetadataSource::MetadataSource(m) => &m.id,
      IMetadataSource::FindMetadata_MetadataSource(m) => &m.id,
      IMetadataSource::GetMetadata_MetadataSource(m) => &m.id,
    }
  }
  fn identifier(&self) -> &Option<String> {
    match self {
      IMetadataSource::MetadataSource(m) => &m.identifier,
      IMetadataSource::FindMetadata_MetadataSource(m) => &m.identifier,
      IMetadataSource::GetMetadata_MetadataSource(m) => &m.identifier,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct EnqueueChildWorkflows_Workflows {
}
#[derive(Serialize, Deserialize)]
pub struct FindMetadata_MetadataSupplementarySource {
  id: String,
  identifier: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub enum ISetWorkflowStateComplete_Metadata {
  SetWorkflowStateComplete_Metadata(SetWorkflowStateComplete_Metadata),
}
impl ISetWorkflowStateComplete_Metadata {
}
#[derive(Serialize, Deserialize)]
pub enum ISourceById_Content {
  SourceById_Content(SourceById_Content),
}
impl ISourceById_Content {
  fn source(&self) -> &Option<SourceById_Source> {
    match self {
      ISourceById_Content::SourceById_Content(m) => &m.source,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct FindMetadata_Query {
  content: FindMetadata_Content,
}
#[derive(Serialize, Deserialize)]
pub struct ActivitiesMutation {
  add: Option<Activity>,
  edit: Option<Activity>,
  delete: bool,
}
#[derive(Serialize, Deserialize)]
pub struct Content {
  find_collection: Vec<Collection>,
  collection: Option<Collection>,
  find_metadata: Vec<Metadata>,
  metadata: Option<Metadata>,
  metadata_supplementary: Option<MetadataSupplementary>,
  sources: Vec<Source>,
  source: Option<Source>,
  traits: Vec<Trait>,
  trait_: Option<Trait>,
  search: SearchResultObject,
}
#[derive(Serialize, Deserialize)]
pub struct AddSearchDocuments_Query {
  content: AddSearchDocuments_Content,
}
#[derive(Serialize, Deserialize)]
pub struct Models {
  all: Vec<Model>,
  model: Option<Model>,
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowExecutionPlan {
  WorkflowExecutionPlan(WorkflowExecutionPlan),
}
impl IWorkflowExecutionPlan {
  fn workflow(&self) -> &Workflow {
    match self {
      IWorkflowExecutionPlan::WorkflowExecutionPlan(m) => &m.workflow,
    }
  }
  fn failed(&self) -> &Vec<WorkflowJobId> {
    match self {
      IWorkflowExecutionPlan::WorkflowExecutionPlan(m) => &m.failed,
    }
  }
  fn parent(&self) -> &Option<WorkflowExecutionId> {
    match self {
      IWorkflowExecutionPlan::WorkflowExecutionPlan(m) => &m.parent,
    }
  }
  fn id(&self) -> &i64 {
    match self {
      IWorkflowExecutionPlan::WorkflowExecutionPlan(m) => &m.id,
    }
  }
  fn metadata(&self) -> &Option<Metadata> {
    match self {
      IWorkflowExecutionPlan::WorkflowExecutionPlan(m) => &m.metadata,
    }
  }
  fn current(&self) -> &Vec<WorkflowJobId> {
    match self {
      IWorkflowExecutionPlan::WorkflowExecutionPlan(m) => &m.current,
    }
  }
  fn next(&self) -> &Option<WorkflowJobId> {
    match self {
      IWorkflowExecutionPlan::WorkflowExecutionPlan(m) => &m.next,
    }
  }
  fn version(&self) -> &Option<i64> {
    match self {
      IWorkflowExecutionPlan::WorkflowExecutionPlan(m) => &m.version,
    }
  }
  fn complete(&self) -> &Vec<WorkflowJobId> {
    match self {
      IWorkflowExecutionPlan::WorkflowExecutionPlan(m) => &m.complete,
    }
  }
  fn pending(&self) -> &Vec<WorkflowJobId> {
    match self {
      IWorkflowExecutionPlan::WorkflowExecutionPlan(m) => &m.pending,
    }
  }
  fn metadata_id(&self) -> &Option<String> {
    match self {
      IWorkflowExecutionPlan::WorkflowExecutionPlan(m) => &m.metadata_id,
    }
  }
  fn context(&self) -> &Value {
    match self {
      IWorkflowExecutionPlan::WorkflowExecutionPlan(m) => &m.context,
    }
  }
  fn collection_id(&self) -> &Option<String> {
    match self {
      IWorkflowExecutionPlan::WorkflowExecutionPlan(m) => &m.collection_id,
    }
  }
  fn error(&self) -> &Option<String> {
    match self {
      IWorkflowExecutionPlan::WorkflowExecutionPlan(m) => &m.error,
    }
  }
  fn supplementary_id(&self) -> &Option<String> {
    match self {
      IWorkflowExecutionPlan::WorkflowExecutionPlan(m) => &m.supplementary_id,
    }
  }
  fn running(&self) -> &Vec<WorkflowJobId> {
    match self {
      IWorkflowExecutionPlan::WorkflowExecutionPlan(m) => &m.running,
    }
  }
  fn jobs(&self) -> &Vec<WorkflowJob> {
    match self {
      IWorkflowExecutionPlan::WorkflowExecutionPlan(m) => &m.jobs,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISupplementaryDownloadUrl_MetadataSupplementary {
  SupplementaryDownloadUrl_MetadataSupplementary(SupplementaryDownloadUrl_MetadataSupplementary),
}
impl ISupplementaryDownloadUrl_MetadataSupplementary {
  fn content(&self) -> &SupplementaryDownloadUrl_MetadataSupplementaryContent {
    match self {
      ISupplementaryDownloadUrl_MetadataSupplementary::SupplementaryDownloadUrl_MetadataSupplementary(m) => &m.content,
    }
  }
  fn key(&self) -> &String {
    match self {
      ISupplementaryDownloadUrl_MetadataSupplementary::SupplementaryDownloadUrl_MetadataSupplementary(m) => &m.key,
    }
  }
  fn metadata_id(&self) -> &String {
    match self {
      ISupplementaryDownloadUrl_MetadataSupplementary::SupplementaryDownloadUrl_MetadataSupplementary(m) => &m.metadata_id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IAddMetadataSupplementary_Query {
  AddMetadataSupplementary_Query(AddMetadataSupplementary_Query),
}
impl IAddMetadataSupplementary_Query {
  fn content(&self) -> &AddMetadataSupplementary_Content {
    match self {
      IAddMetadataSupplementary_Query::AddMetadataSupplementary_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataWorkflowInput {
  MetadataWorkflowInput(MetadataWorkflowInput),
}
impl IMetadataWorkflowInput {
  fn state(&self) -> &String {
    match self {
      IMetadataWorkflowInput::MetadataWorkflowInput(m) => &m.state,
    }
  }
  fn delete_workflow_id(&self) -> &Option<String> {
    match self {
      IMetadataWorkflowInput::MetadataWorkflowInput(m) => &m.delete_workflow_id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct AddMetadataSupplementary_Query {
  content: AddMetadataSupplementary_Content,
}
#[derive(Serialize, Deserialize)]
pub enum IFindAttributeInput {
  FindAttributeInput(FindAttributeInput),
}
impl IFindAttributeInput {
  fn value(&self) -> &String {
    match self {
      IFindAttributeInput::FindAttributeInput(m) => &m.value,
    }
  }
  fn key(&self) -> &String {
    match self {
      IFindAttributeInput::FindAttributeInput(m) => &m.key,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SetCollectionWorkflowState_Query {
  content: SetCollectionWorkflowState_Content,
}
#[derive(Serialize, Deserialize)]
pub enum ISourceById_Query {
  SourceById_Query(SourceById_Query),
}
impl ISourceById_Query {
  fn content(&self) -> &SourceById_Content {
    match self {
      ISourceById_Query::SourceById_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ITraitById_Content {
  TraitById_Content(TraitById_Content),
}
impl ITraitById_Content {
  fn trait_(&self) -> &Option<TraitById_Trait> {
    match self {
      ITraitById_Content::TraitById_Content(m) => &m.trait_,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum StorageSystemType {
  SEARCH,
  VECTOR,
  SUPPLEMENTARY,
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowActivityParameterInput {
  name: String,
  value: String,
}
#[derive(Serialize, Deserialize)]
pub struct AddChildMetadata_Content {
  collection: Option<AddChildMetadata_Collection>,
}
#[derive(Serialize, Deserialize)]
pub struct SupplementaryUploadUrl_Content {
  metadata_supplementary: Option<SupplementaryUploadUrl_MetadataSupplementary>,
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataDownloadUrl_Content {
  MetadataDownloadUrl_Content(MetadataDownloadUrl_Content),
}
impl IMetadataDownloadUrl_Content {
  fn metadata(&self) -> &Option<MetadataDownloadUrl_Metadata> {
    match self {
      IMetadataDownloadUrl_Content::MetadataDownloadUrl_Content(m) => &m.metadata,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataUploadUrl_Metadata {
  MetadataUploadUrl_Metadata(MetadataUploadUrl_Metadata),
}
impl IMetadataUploadUrl_Metadata {
  fn content(&self) -> &MetadataUploadUrl_MetadataContent {
    match self {
      IMetadataUploadUrl_Metadata::MetadataUploadUrl_Metadata(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct FindMetadata_MetadataSupplementary {
  key: String,
  uploaded: Option<String>,
  content: FindMetadata_MetadataSupplementaryContent,
  source: FindMetadata_MetadataSupplementarySource,
}
#[derive(Serialize, Deserialize)]
pub enum IGetCollectionItems_Content {
  GetCollectionItems_Content(GetCollectionItems_Content),
}
impl IGetCollectionItems_Content {
  fn collection(&self) -> &Option<GetCollectionItems_Collection> {
    match self {
      IGetCollectionItems_Content::GetCollectionItems_Content(m) => &m.collection,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct GetMetadata_Query {
  content: GetMetadata_Content,
}
#[derive(Serialize, Deserialize)]
pub enum IAddSearchDocuments_Query {
  AddSearchDocuments_Query(AddSearchDocuments_Query),
}
impl IAddSearchDocuments_Query {
  fn content(&self) -> &AddSearchDocuments_Content {
    match self {
      IAddSearchDocuments_Query::AddSearchDocuments_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SetWorkflowStateComplete_Content {
  metadata: Option<SetWorkflowStateComplete_Metadata>,
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataDownloadUrl_SignedUrl {
  MetadataDownloadUrl_SignedUrl(MetadataDownloadUrl_SignedUrl),
}
impl IMetadataDownloadUrl_SignedUrl {
  fn headers(&self) -> &Vec<MetadataDownloadUrl_SignedUrlHeader> {
    match self {
      IMetadataDownloadUrl_SignedUrl::MetadataDownloadUrl_SignedUrl(m) => &m.headers,
    }
  }
  fn url(&self) -> &String {
    match self {
      IMetadataDownloadUrl_SignedUrl::MetadataDownloadUrl_SignedUrl(m) => &m.url,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SecurityMutation {
  signup: Principal,
  add_principal_group: bool,
}
#[derive(Serialize, Deserialize)]
pub struct Prompts {
  all: Vec<Prompt>,
  prompt: Option<Prompt>,
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowJobId {
  queue: String,
  id: i64,
  index: i64,
}
#[derive(Serialize, Deserialize)]
pub enum ILogin_Principal {
  Login_Principal(Login_Principal),
}
impl ILogin_Principal {
  fn id(&self) -> &String {
    match self {
      ILogin_Principal::Login_Principal(m) => &m.id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IGetCollection_Content {
  GetCollection_Content(GetCollection_Content),
}
impl IGetCollection_Content {
  fn collection(&self) -> &Option<GetCollection_Collection> {
    match self {
      IGetCollection_Content::GetCollection_Content(m) => &m.collection,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IActivity {
  Activity(Activity),
}
impl IActivity {
  fn child_workflow_id(&self) -> &Option<String> {
    match self {
      IActivity::Activity(m) => &m.child_workflow_id,
    }
  }
  fn name(&self) -> &String {
    match self {
      IActivity::Activity(m) => &m.name,
    }
  }
  fn inputs(&self) -> &Vec<ActivityParameter> {
    match self {
      IActivity::Activity(m) => &m.inputs,
    }
  }
  fn configuration(&self) -> &Value {
    match self {
      IActivity::Activity(m) => &m.configuration,
    }
  }
  fn description(&self) -> &String {
    match self {
      IActivity::Activity(m) => &m.description,
    }
  }
  fn outputs(&self) -> &Vec<ActivityParameter> {
    match self {
      IActivity::Activity(m) => &m.outputs,
    }
  }
  fn id(&self) -> &String {
    match self {
      IActivity::Activity(m) => &m.id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct MetadataWorkflow {
  state: String,
  pending: Option<String>,
  delete_workflow: Option<String>,
  plans: Vec<WorkflowExecutionPlan>,
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowActivityParameter {
  name: String,
  value: String,
}
#[derive(Serialize, Deserialize)]
pub enum IGetMetadata_Metadata {
  GetMetadata_Metadata(GetMetadata_Metadata),
}
impl IGetMetadata_Metadata {
  fn source(&self) -> &GetMetadata_MetadataSource {
    match self {
      IGetMetadata_Metadata::GetMetadata_Metadata(m) => &m.source,
    }
  }
  fn id(&self) -> &String {
    match self {
      IGetMetadata_Metadata::GetMetadata_Metadata(m) => &m.id,
    }
  }
  fn trait_ids(&self) -> &Vec<String> {
    match self {
      IGetMetadata_Metadata::GetMetadata_Metadata(m) => &m.trait_ids,
    }
  }
  fn attributes(&self) -> &Value {
    match self {
      IGetMetadata_Metadata::GetMetadata_Metadata(m) => &m.attributes,
    }
  }
  fn language_tag(&self) -> &String {
    match self {
      IGetMetadata_Metadata::GetMetadata_Metadata(m) => &m.language_tag,
    }
  }
  fn name(&self) -> &String {
    match self {
      IGetMetadata_Metadata::GetMetadata_Metadata(m) => &m.name,
    }
  }
  fn labels(&self) -> &Vec<String> {
    match self {
      IGetMetadata_Metadata::GetMetadata_Metadata(m) => &m.labels,
    }
  }
  fn content(&self) -> &GetMetadata_MetadataContent {
    match self {
      IGetMetadata_Metadata::GetMetadata_Metadata(m) => &m.content,
    }
  }
  fn version(&self) -> &i64 {
    match self {
      IGetMetadata_Metadata::GetMetadata_Metadata(m) => &m.version,
    }
  }
  fn created(&self) -> &DateTime<Utc> {
    match self {
      IGetMetadata_Metadata::GetMetadata_Metadata(m) => &m.created,
    }
  }
  fn modified(&self) -> &DateTime<Utc> {
    match self {
      IGetMetadata_Metadata::GetMetadata_Metadata(m) => &m.modified,
    }
  }
  fn supplementary(&self) -> &Vec<GetMetadata_MetadataSupplementary> {
    match self {
      IGetMetadata_Metadata::GetMetadata_Metadata(m) => &m.supplementary,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SetMetadataAttributes_Content {
  metadata: Option<SetMetadataAttributes_Metadata>,
}
#[derive(Serialize, Deserialize)]
pub enum ISetCollectionReady_Query {
  SetCollectionReady_Query(SetCollectionReady_Query),
}
impl ISetCollectionReady_Query {
  fn content(&self) -> &SetCollectionReady_Content {
    match self {
      ISetCollectionReady_Query::SetCollectionReady_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SetWorkflowPlanContext_Workflows {
}
#[derive(Serialize, Deserialize)]
pub enum GetCollectionItems_CollectionItem {
}
impl GetCollectionItems_CollectionItem {
}
#[derive(Serialize, Deserialize)]
pub struct GetMetadata_Metadata {
  id: String,
  version: i64,
  trait_ids: Vec<String>,
  name: String,
  content: GetMetadata_MetadataContent,
  language_tag: String,
  labels: Vec<String>,
  attributes: Value,
  created: DateTime<Utc>,
  modified: DateTime<Utc>,
  source: GetMetadata_MetadataSource,
  supplementary: Vec<GetMetadata_MetadataSupplementary>,
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataDownloadUrl_MetadataContentUrls {
  MetadataDownloadUrl_MetadataContentUrls(MetadataDownloadUrl_MetadataContentUrls),
}
impl IMetadataDownloadUrl_MetadataContentUrls {
  fn download(&self) -> &MetadataDownloadUrl_SignedUrl {
    match self {
      IMetadataDownloadUrl_MetadataContentUrls::MetadataDownloadUrl_MetadataContentUrls(m) => &m.download,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowJob {
  id: WorkflowJobId,
  workflow: Workflow,
  error: Option<String>,
  collection_id: Option<String>,
  collection: Option<Collection>,
  metadata: Option<Metadata>,
  version: Option<i64>,
  supplementary_id: Option<String>,
  activity: Activity,
  children: Vec<WorkflowExecutionId>,
  completed_children: Vec<WorkflowExecutionId>,
  failed_children: Vec<WorkflowExecutionId>,
  workflow_activity: WorkflowActivity,
  prompts: Vec<WorkflowActivityPrompt>,
  storage_systems: Vec<WorkflowActivityStorageSystem>,
  models: Vec<WorkflowActivityModel>,
  context: Value,
}
#[derive(Serialize, Deserialize)]
pub struct AddMetadata_Query {
  content: AddMetadata_Content,
}
#[derive(Serialize, Deserialize)]
pub struct AddSearchDocuments_Metadata {
}
#[derive(Serialize, Deserialize)]
pub enum ISupplementaryDownloadUrl_MetadataSupplementaryContent {
  SupplementaryDownloadUrl_MetadataSupplementaryContent(SupplementaryDownloadUrl_MetadataSupplementaryContent),
}
impl ISupplementaryDownloadUrl_MetadataSupplementaryContent {
  fn urls(&self) -> &SupplementaryDownloadUrl_MetadataSupplementaryContentUrls {
    match self {
      ISupplementaryDownloadUrl_MetadataSupplementaryContent::SupplementaryDownloadUrl_MetadataSupplementaryContent(m) => &m.urls,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct PermissionInput {
  entity_id: String,
  group_id: String,
  action: PermissionAction,
}
#[derive(Serialize, Deserialize)]
pub enum MetadataType {
  STANDARD,
  VARIANT,
}
#[derive(Serialize, Deserialize)]
pub enum IGetMetadata_MetadataSource {
  GetMetadata_MetadataSource(GetMetadata_MetadataSource),
}
impl IGetMetadata_MetadataSource {
  fn identifier(&self) -> &Option<String> {
    match self {
      IGetMetadata_MetadataSource::GetMetadata_MetadataSource(m) => &m.identifier,
    }
  }
  fn id(&self) -> &Option<String> {
    match self {
      IGetMetadata_MetadataSource::GetMetadata_MetadataSource(m) => &m.id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SetMetadataSystemAttributes_Content {
  metadata: Option<SetMetadataSystemAttributes_Metadata>,
}
#[derive(Serialize, Deserialize)]
pub struct SupplementaryUploadUrl_SignedUrlHeader {
  name: String,
  value: String,
}
#[derive(Serialize, Deserialize)]
pub struct SetCollectionPublicList_Content {
  collection: Option<SetCollectionPublicList_Collection>,
}
#[derive(Serialize, Deserialize)]
pub enum ISetCollectionWorkflowStateComplete_Query {
  SetCollectionWorkflowStateComplete_Query(SetCollectionWorkflowStateComplete_Query),
}
impl ISetCollectionWorkflowStateComplete_Query {
  fn content(&self) -> &SetCollectionWorkflowStateComplete_Content {
    match self {
      ISetCollectionWorkflowStateComplete_Query::SetCollectionWorkflowStateComplete_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISetWorkflowJobFailed_Workflows {
  SetWorkflowJobFailed_Workflows(SetWorkflowJobFailed_Workflows),
}
impl ISetWorkflowJobFailed_Workflows {
}
#[derive(Serialize, Deserialize)]
pub struct MetadataWorkflowCompleteState {
  metadata_id: String,
  status: String,
}
#[derive(Serialize, Deserialize)]
pub struct TraitById_Query {
  content: TraitById_Content,
}
#[derive(Serialize, Deserialize)]
pub enum ISignedUrl {
  SignedUrl(SignedUrl),
  MetadataDownloadUrl_SignedUrl(MetadataDownloadUrl_SignedUrl),
  MetadataUploadUrl_SignedUrl(MetadataUploadUrl_SignedUrl),
  SupplementaryDownloadUrl_SignedUrl(SupplementaryDownloadUrl_SignedUrl),
  SupplementaryUploadUrl_SignedUrl(SupplementaryUploadUrl_SignedUrl),
}
impl ISignedUrl {
  fn url(&self) -> &String {
    match self {
      ISignedUrl::SignedUrl(m) => &m.url,
      ISignedUrl::MetadataDownloadUrl_SignedUrl(m) => &m.url,
      ISignedUrl::MetadataUploadUrl_SignedUrl(m) => &m.url,
      ISignedUrl::SupplementaryDownloadUrl_SignedUrl(m) => &m.url,
      ISignedUrl::SupplementaryUploadUrl_SignedUrl(m) => &m.url,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowConfigurationInput {
  WorkflowConfigurationInput(WorkflowConfigurationInput),
}
impl IWorkflowConfigurationInput {
  fn activity_id(&self) -> &String {
    match self {
      IWorkflowConfigurationInput::WorkflowConfigurationInput(m) => &m.activity_id,
    }
  }
  fn configuration(&self) -> &Value {
    match self {
      IWorkflowConfigurationInput::WorkflowConfigurationInput(m) => &m.configuration,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISupplementaryUploadUrl_Content {
  SupplementaryUploadUrl_Content(SupplementaryUploadUrl_Content),
}
impl ISupplementaryUploadUrl_Content {
  fn metadata_supplementary(&self) -> &Option<SupplementaryUploadUrl_MetadataSupplementary> {
    match self {
      ISupplementaryUploadUrl_Content::SupplementaryUploadUrl_Content(m) => &m.metadata_supplementary,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct MetadataDownloadUrl_Metadata {
  content: MetadataDownloadUrl_MetadataContent,
}
#[derive(Serialize, Deserialize)]
pub struct Queues {
  message_queues: Vec<MessageQueue>,
  get_messages: Vec<Message>,
  get_message: Option<Message>,
}
#[derive(Serialize, Deserialize)]
pub struct SourceById_Content {
  source: Option<SourceById_Source>,
}
#[derive(Serialize, Deserialize)]
pub struct MetadataSource {
  id: Option<String>,
  identifier: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataWorkflowCompleteState {
  MetadataWorkflowCompleteState(MetadataWorkflowCompleteState),
}
impl IMetadataWorkflowCompleteState {
  fn metadata_id(&self) -> &String {
    match self {
      IMetadataWorkflowCompleteState::MetadataWorkflowCompleteState(m) => &m.metadata_id,
    }
  }
  fn status(&self) -> &String {
    match self {
      IMetadataWorkflowCompleteState::MetadataWorkflowCompleteState(m) => &m.status,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SupplementaryUploadUrl_SignedUrl {
  url: String,
  headers: Vec<SupplementaryUploadUrl_SignedUrlHeader>,
}
#[derive(Serialize, Deserialize)]
pub enum IBeginTransitionInput {
  BeginTransitionInput(BeginTransitionInput),
}
impl IBeginTransitionInput {
  fn collection_id(&self) -> &Option<String> {
    match self {
      IBeginTransitionInput::BeginTransitionInput(m) => &m.collection_id,
    }
  }
  fn supplementary_id(&self) -> &Option<String> {
    match self {
      IBeginTransitionInput::BeginTransitionInput(m) => &m.supplementary_id,
    }
  }
  fn wait_for_completion(&self) -> &Option<bool> {
    match self {
      IBeginTransitionInput::BeginTransitionInput(m) => &m.wait_for_completion,
    }
  }
  fn status(&self) -> &String {
    match self {
      IBeginTransitionInput::BeginTransitionInput(m) => &m.status,
    }
  }
  fn metadata_id(&self) -> &Option<String> {
    match self {
      IBeginTransitionInput::BeginTransitionInput(m) => &m.metadata_id,
    }
  }
  fn version(&self) -> &Option<i64> {
    match self {
      IBeginTransitionInput::BeginTransitionInput(m) => &m.version,
    }
  }
  fn state_id(&self) -> &String {
    match self {
      IBeginTransitionInput::BeginTransitionInput(m) => &m.state_id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct MetadataSupplementaryContentUrls {
  download: SignedUrl,
  upload: SignedUrl,
}
#[derive(Serialize, Deserialize)]
pub enum IStorageSystem {
  StorageSystem(StorageSystem),
}
impl IStorageSystem {
  fn configuration(&self) -> &Value {
    match self {
      IStorageSystem::StorageSystem(m) => &m.configuration,
    }
  }
  fn name(&self) -> &String {
    match self {
      IStorageSystem::StorageSystem(m) => &m.name,
    }
  }
  fn id(&self) -> &String {
    match self {
      IStorageSystem::StorageSystem(m) => &m.id,
    }
  }
  fn type_(&self) -> &StorageSystemType {
    match self {
      IStorageSystem::StorageSystem(m) => &m.type_,
    }
  }
  fn description(&self) -> &String {
    match self {
      IStorageSystem::StorageSystem(m) => &m.description,
    }
  }
  fn models(&self) -> &Vec<StorageSystemModel> {
    match self {
      IStorageSystem::StorageSystem(m) => &m.models,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IActivityParameter {
  ActivityParameter(ActivityParameter),
}
impl IActivityParameter {
  fn name(&self) -> &String {
    match self {
      IActivityParameter::ActivityParameter(m) => &m.name,
    }
  }
  fn type_(&self) -> &ActivityParameterType {
    match self {
      IActivityParameter::ActivityParameter(m) => &m.type_,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct Token {
  token: String,
}
#[derive(Serialize, Deserialize)]
pub enum ICollection {
  Collection(Collection),
  AddChildCollection_Collection(AddChildCollection_Collection),
  AddChildMetadata_Collection(AddChildMetadata_Collection),
  AddCollection_Collection(AddCollection_Collection),
  GetCollection_Collection(GetCollection_Collection),
  FindCollection_Collection(FindCollection_Collection),
  GetCollectionItems_Collection(GetCollectionItems_Collection),
  SetCollectionPublic_Collection(SetCollectionPublic_Collection),
  SetCollectionPublicList_Collection(SetCollectionPublicList_Collection),
  SetCollectionReady_Collection(SetCollectionReady_Collection),
  SetCollectionWorkflowState_Collection(SetCollectionWorkflowState_Collection),
  SetCollectionWorkflowStateComplete_Collection(SetCollectionWorkflowStateComplete_Collection),
}
impl ICollection {
}
#[derive(Serialize, Deserialize)]
pub enum IGetMetadata_MetadataContent {
  GetMetadata_MetadataContent(GetMetadata_MetadataContent),
}
impl IGetMetadata_MetadataContent {
  fn type_(&self) -> &String {
    match self {
      IGetMetadata_MetadataContent::GetMetadata_MetadataContent(m) => &m.type_,
    }
  }
  fn length(&self) -> &Option<i64> {
    match self {
      IGetMetadata_MetadataContent::GetMetadata_MetadataContent(m) => &m.length,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IAddMetadata_Metadata {
  AddMetadata_Metadata(AddMetadata_Metadata),
}
impl IAddMetadata_Metadata {
}
#[derive(Serialize, Deserialize)]
pub enum IAddSearchDocuments_Metadata {
  AddSearchDocuments_Metadata(AddSearchDocuments_Metadata),
}
impl IAddSearchDocuments_Metadata {
}
#[derive(Serialize, Deserialize)]
pub struct GetMetadata_MetadataSupplementaryContent {
  type_: String,
  length: Option<i64>,
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataUploadUrl_Content {
  MetadataUploadUrl_Content(MetadataUploadUrl_Content),
}
impl IMetadataUploadUrl_Content {
  fn metadata(&self) -> &Option<MetadataUploadUrl_Metadata> {
    match self {
      IMetadataUploadUrl_Content::MetadataUploadUrl_Content(m) => &m.metadata,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct Activity {
  id: String,
  name: String,
  description: String,
  child_workflow_id: Option<String>,
  configuration: Value,
  inputs: Vec<ActivityParameter>,
  outputs: Vec<ActivityParameter>,
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowConfigurationInput {
  activity_id: String,
  configuration: Value,
}
#[derive(Serialize, Deserialize)]
pub struct SetCollectionWorkflowState_Collection {
}
#[derive(Serialize, Deserialize)]
pub struct SetCollectionReady_Query {
  content: SetCollectionReady_Content,
}
#[derive(Serialize, Deserialize)]
pub struct SourceById_Source {
  id: String,
  name: String,
}
#[derive(Serialize, Deserialize)]
pub struct Login_Token {
  token: String,
}
#[derive(Serialize, Deserialize)]
pub enum IMutation {
  Mutation(Mutation),
}
impl IMutation {
  fn queues(&self) -> &QueuesMutation {
    match self {
      IMutation::Mutation(m) => &m.queues,
    }
  }
  fn workflows(&self) -> &WorkflowsMutation {
    match self {
      IMutation::Mutation(m) => &m.workflows,
    }
  }
  fn content(&self) -> &ContentMutation {
    match self {
      IMutation::Mutation(m) => &m.content,
    }
  }
  fn security(&self) -> &SecurityMutation {
    match self {
      IMutation::Mutation(m) => &m.security,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISecurity {
  Security(Security),
  Login_Security(Login_Security),
}
impl ISecurity {
}
#[derive(Serialize, Deserialize)]
pub struct SetCollectionPublic_Collection {
}
#[derive(Serialize, Deserialize)]
pub enum IAddMetadataBulk_Content {
  AddMetadataBulk_Content(AddMetadataBulk_Content),
}
impl IAddMetadataBulk_Content {
  fn metadata(&self) -> &Option<AddMetadataBulk_Metadata> {
    match self {
      IAddMetadataBulk_Content::AddMetadataBulk_Content(m) => &m.metadata,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISetMetadataAttributes_Query {
  SetMetadataAttributes_Query(SetMetadataAttributes_Query),
}
impl ISetMetadataAttributes_Query {
  fn content(&self) -> &SetMetadataAttributes_Content {
    match self {
      ISetMetadataAttributes_Query::SetMetadataAttributes_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataDownloadUrl_Metadata {
  MetadataDownloadUrl_Metadata(MetadataDownloadUrl_Metadata),
}
impl IMetadataDownloadUrl_Metadata {
  fn content(&self) -> &MetadataDownloadUrl_MetadataContent {
    match self {
      IMetadataDownloadUrl_Metadata::MetadataDownloadUrl_Metadata(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowState {
  WorkflowState(WorkflowState),
}
impl IWorkflowState {
  fn name(&self) -> &String {
    match self {
      IWorkflowState::WorkflowState(m) => &m.name,
    }
  }
  fn exit_workflow_id(&self) -> &Option<String> {
    match self {
      IWorkflowState::WorkflowState(m) => &m.exit_workflow_id,
    }
  }
  fn workflow_id(&self) -> &Option<String> {
    match self {
      IWorkflowState::WorkflowState(m) => &m.workflow_id,
    }
  }
  fn type_(&self) -> &WorkflowStateType {
    match self {
      IWorkflowState::WorkflowState(m) => &m.type_,
    }
  }
  fn entry_workflow_id(&self) -> &Option<String> {
    match self {
      IWorkflowState::WorkflowState(m) => &m.entry_workflow_id,
    }
  }
  fn configuration(&self) -> &Value {
    match self {
      IWorkflowState::WorkflowState(m) => &m.configuration,
    }
  }
  fn description(&self) -> &String {
    match self {
      IWorkflowState::WorkflowState(m) => &m.description,
    }
  }
  fn id(&self) -> &String {
    match self {
      IWorkflowState::WorkflowState(m) => &m.id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct FindAttributeInput {
  key: String,
  value: String,
}
#[derive(Serialize, Deserialize)]
pub enum IPrincipal {
  Principal(Principal),
  Login_Principal(Login_Principal),
}
impl IPrincipal {
  fn id(&self) -> &String {
    match self {
      IPrincipal::Principal(m) => &m.id,
      IPrincipal::Login_Principal(m) => &m.id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IEnqueueChildWorkflows_Workflows {
  EnqueueChildWorkflows_Workflows(EnqueueChildWorkflows_Workflows),
}
impl IEnqueueChildWorkflows_Workflows {
}
#[derive(Serialize, Deserialize)]
pub struct SetMetadataPublic_Content {
  metadata: Option<SetMetadataPublic_Metadata>,
}
#[derive(Serialize, Deserialize)]
pub struct Plan_Query {
  workflows: Plan_Workflows,
}
#[derive(Serialize, Deserialize)]
pub struct Collection {
  id: String,
  type_: CollectionType,
  name: String,
  description: Option<String>,
  trait_ids: Vec<String>,
  labels: Vec<String>,
  attributes: Value,
  item_attributes: Option<Value>,
  system_attributes: Option<Value>,
  ordering: Option<Value>,
  created: DateTime<Utc>,
  modified: DateTime<Utc>,
  parent_collections: Vec<Collection>,
  items: Vec<CollectionItem>,
  collections: Vec<Collection>,
  metadata: Vec<Metadata>,
  workflow: CollectionWorkflow,
  ready: Option<DateTime<Utc>>,
  public: bool,
  public_list: bool,
  permissions: Vec<Permission>,
}
#[derive(Serialize, Deserialize)]
pub struct SetWorkflowJobContext_Query {
  workflows: SetWorkflowJobContext_Workflows,
}
#[derive(Serialize, Deserialize)]
pub enum ISetCollectionPublicList_Collection {
  SetCollectionPublicList_Collection(SetCollectionPublicList_Collection),
}
impl ISetCollectionPublicList_Collection {
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowActivity {
  WorkflowActivity(WorkflowActivity),
}
impl IWorkflowActivity {
  fn configuration(&self) -> &Value {
    match self {
      IWorkflowActivity::WorkflowActivity(m) => &m.configuration,
    }
  }
  fn inputs(&self) -> &Vec<WorkflowActivityParameter> {
    match self {
      IWorkflowActivity::WorkflowActivity(m) => &m.inputs,
    }
  }
  fn execution_group(&self) -> &i64 {
    match self {
      IWorkflowActivity::WorkflowActivity(m) => &m.execution_group,
    }
  }
  fn outputs(&self) -> &Vec<WorkflowActivityParameter> {
    match self {
      IWorkflowActivity::WorkflowActivity(m) => &m.outputs,
    }
  }
  fn id(&self) -> &i64 {
    match self {
      IWorkflowActivity::WorkflowActivity(m) => &m.id,
    }
  }
  fn queue(&self) -> &String {
    match self {
      IWorkflowActivity::WorkflowActivity(m) => &m.queue,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ITrait {
  Trait(Trait),
  TraitById_Trait(TraitById_Trait),
}
impl ITrait {
  fn workflow_ids(&self) -> &Vec<String> {
    match self {
      ITrait::Trait(m) => &m.workflow_ids,
      ITrait::TraitById_Trait(m) => &m.workflow_ids,
    }
  }
  fn id(&self) -> &String {
    match self {
      ITrait::Trait(m) => &m.id,
      ITrait::TraitById_Trait(m) => &m.id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataSourceInput {
  MetadataSourceInput(MetadataSourceInput),
}
impl IMetadataSourceInput {
  fn id(&self) -> &String {
    match self {
      IMetadataSourceInput::MetadataSourceInput(m) => &m.id,
    }
  }
  fn identifier(&self) -> &String {
    match self {
      IMetadataSourceInput::MetadataSourceInput(m) => &m.identifier,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataUploadUrl_Query {
  MetadataUploadUrl_Query(MetadataUploadUrl_Query),
}
impl IMetadataUploadUrl_Query {
  fn content(&self) -> &MetadataUploadUrl_Content {
    match self {
      IMetadataUploadUrl_Query::MetadataUploadUrl_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SetCollectionPublicList_Collection {
}
#[derive(Serialize, Deserialize)]
pub struct AddChildCollection_Query {
  content: AddChildCollection_Content,
}
#[derive(Serialize, Deserialize)]
pub enum IAddChildCollection_Content {
  AddChildCollection_Content(AddChildCollection_Content),
}
impl IAddChildCollection_Content {
  fn collection(&self) -> &Option<AddChildCollection_Collection> {
    match self {
      IAddChildCollection_Content::AddChildCollection_Content(m) => &m.collection,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum WorkflowExecution {
  WorkflowExecutionPlan(WorkflowExecutionPlan),
  WorkflowJob(WorkflowJob),
}
impl WorkflowExecution {
  fn collection_id(&self) -> &Option<String> {
    match self {
      WorkflowExecution::WorkflowExecutionPlan(m) => &m.collection_id,
      WorkflowExecution::WorkflowJob(m) => &m.collection_id,
    }
  }
  fn workflow(&self) -> &Workflow {
    match self {
      WorkflowExecution::WorkflowExecutionPlan(m) => &m.workflow,
      WorkflowExecution::WorkflowJob(m) => &m.workflow,
    }
  }
  fn metadata(&self) -> &Option<Metadata> {
    match self {
      WorkflowExecution::WorkflowExecutionPlan(m) => &m.metadata,
      WorkflowExecution::WorkflowJob(m) => &m.metadata,
    }
  }
  fn supplementary_id(&self) -> &Option<String> {
    match self {
      WorkflowExecution::WorkflowExecutionPlan(m) => &m.supplementary_id,
      WorkflowExecution::WorkflowJob(m) => &m.supplementary_id,
    }
  }
  fn context(&self) -> &Value {
    match self {
      WorkflowExecution::WorkflowExecutionPlan(m) => &m.context,
      WorkflowExecution::WorkflowJob(m) => &m.context,
    }
  }
  fn error(&self) -> &Option<String> {
    match self {
      WorkflowExecution::WorkflowExecutionPlan(m) => &m.error,
      WorkflowExecution::WorkflowJob(m) => &m.error,
    }
  }
  fn version(&self) -> &Option<i64> {
    match self {
      WorkflowExecution::WorkflowExecutionPlan(m) => &m.version,
      WorkflowExecution::WorkflowJob(m) => &m.version,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISetMetadataPublic_Query {
  SetMetadataPublic_Query(SetMetadataPublic_Query),
}
impl ISetMetadataPublic_Query {
  fn content(&self) -> &SetMetadataPublic_Content {
    match self {
      ISetMetadataPublic_Query::SetMetadataPublic_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct Login_Login {
  password: Login_LoginResponse,
}
#[derive(Serialize, Deserialize)]
pub enum ISearchDocumentInput {
  SearchDocumentInput(SearchDocumentInput),
}
impl ISearchDocumentInput {
  fn collection_id(&self) -> &Option<String> {
    match self {
      ISearchDocumentInput::SearchDocumentInput(m) => &m.collection_id,
    }
  }
  fn metadata_id(&self) -> &Option<String> {
    match self {
      ISearchDocumentInput::SearchDocumentInput(m) => &m.metadata_id,
    }
  }
  fn content(&self) -> &String {
    match self {
      ISearchDocumentInput::SearchDocumentInput(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISearchResultObject {
  SearchResultObject(SearchResultObject),
}
impl ISearchResultObject {
  fn documents(&self) -> &Vec<SearchDocument> {
    match self {
      ISearchResultObject::SearchResultObject(m) => &m.documents,
    }
  }
  fn estimated_hits(&self) -> &i64 {
    match self {
      ISearchResultObject::SearchResultObject(m) => &m.estimated_hits,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataDownloadUrl_Query {
  MetadataDownloadUrl_Query(MetadataDownloadUrl_Query),
}
impl IMetadataDownloadUrl_Query {
  fn content(&self) -> &MetadataDownloadUrl_Content {
    match self {
      IMetadataDownloadUrl_Query::MetadataDownloadUrl_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct MetadataSourceInput {
  id: String,
  identifier: String,
}
#[derive(Serialize, Deserialize)]
pub struct WorkflowsMutation {
  add: Workflow,
  edit: Workflow,
  delete: bool,
  models: ModelsMutation,
  states: WorkflowStatesMutation,
  activities: ActivitiesMutation,
  prompts: PromptsMutation,
  begin_transition: bool,
  enqueue_child_workflows: Vec<WorkflowExecutionId>,
  enqueue_child_workflow: WorkflowExecutionId,
  enqueue_job: Option<WorkflowExecutionId>,
  find_and_enqueue_workflow: Vec<WorkflowExecutionId>,
  enqueue_workflow: WorkflowExecutionId,
  set_execution_plan_context: bool,
  set_execution_job_context: bool,
  set_execution_plan_job_checkin: bool,
  set_execution_plan_job_complete: bool,
  set_execution_plan_job_failed: bool,
}
#[derive(Serialize, Deserialize)]
pub struct AddChildMetadata_Query {
  content: AddChildMetadata_Content,
}
#[derive(Serialize, Deserialize)]
pub struct GetMetadata_Content {
  metadata: Option<GetMetadata_Metadata>,
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataSupplementary {
  MetadataSupplementary(MetadataSupplementary),
  FindMetadata_MetadataSupplementary(FindMetadata_MetadataSupplementary),
  GetMetadata_MetadataSupplementary(GetMetadata_MetadataSupplementary),
  SupplementaryDownloadUrl_MetadataSupplementary(SupplementaryDownloadUrl_MetadataSupplementary),
  SupplementaryUploadUrl_MetadataSupplementary(SupplementaryUploadUrl_MetadataSupplementary),
}
impl IMetadataSupplementary {
  fn key(&self) -> &String {
    match self {
      IMetadataSupplementary::MetadataSupplementary(m) => &m.key,
      IMetadataSupplementary::FindMetadata_MetadataSupplementary(m) => &m.key,
      IMetadataSupplementary::GetMetadata_MetadataSupplementary(m) => &m.key,
      IMetadataSupplementary::SupplementaryDownloadUrl_MetadataSupplementary(m) => &m.key,
      IMetadataSupplementary::SupplementaryUploadUrl_MetadataSupplementary(m) => &m.key,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SetMetadataReady_Metadata {
}
#[derive(Serialize, Deserialize)]
pub struct GetCollection_Collection {
  id: String,
  name: String,
  labels: Vec<String>,
  attributes: Value,
  created: DateTime<Utc>,
  modified: DateTime<Utc>,
}
#[derive(Serialize, Deserialize)]
pub struct SupplementaryDownloadUrl_MetadataSupplementaryContent {
  urls: SupplementaryDownloadUrl_MetadataSupplementaryContentUrls,
}
#[derive(Serialize, Deserialize)]
pub struct MetadataChildInput {
  metadata: MetadataInput,
  attributes: Option<Value>,
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataContent {
  MetadataContent(MetadataContent),
  FindMetadata_MetadataContent(FindMetadata_MetadataContent),
  GetMetadata_MetadataContent(GetMetadata_MetadataContent),
  MetadataDownloadUrl_MetadataContent(MetadataDownloadUrl_MetadataContent),
  MetadataUploadUrl_MetadataContent(MetadataUploadUrl_MetadataContent),
}
impl IMetadataContent {
  fn type_(&self) -> &String {
    match self {
      IMetadataContent::MetadataContent(m) => &m.type_,
      IMetadataContent::FindMetadata_MetadataContent(m) => &m.type_,
      IMetadataContent::GetMetadata_MetadataContent(m) => &m.type_,
      IMetadataContent::MetadataDownloadUrl_MetadataContent(m) => &m.type_,
      IMetadataContent::MetadataUploadUrl_MetadataContent(m) => &m.type_,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IActivitiesMutation {
  ActivitiesMutation(ActivitiesMutation),
}
impl IActivitiesMutation {
  fn edit(&self) -> &Option<Activity> {
    match self {
      IActivitiesMutation::ActivitiesMutation(m) => &m.edit,
    }
  }
  fn add(&self) -> &Option<Activity> {
    match self {
      IActivitiesMutation::ActivitiesMutation(m) => &m.add,
    }
  }
  fn delete(&self) -> &bool {
    match self {
      IActivitiesMutation::ActivitiesMutation(m) => &m.delete,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISupplementaryDownloadUrl_SignedUrlHeader {
  SupplementaryDownloadUrl_SignedUrlHeader(SupplementaryDownloadUrl_SignedUrlHeader),
}
impl ISupplementaryDownloadUrl_SignedUrlHeader {
  fn name(&self) -> &String {
    match self {
      ISupplementaryDownloadUrl_SignedUrlHeader::SupplementaryDownloadUrl_SignedUrlHeader(m) => &m.name,
    }
  }
  fn value(&self) -> &String {
    match self {
      ISupplementaryDownloadUrl_SignedUrlHeader::SupplementaryDownloadUrl_SignedUrlHeader(m) => &m.value,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IActivityParameterType {
  ActivityParameterType(ActivityParameterType),
}
impl IActivityParameterType {
}
#[derive(Serialize, Deserialize)]
pub struct AttributesFilterInput {
  attributes: Vec<String>,
  child_attributes: Option<AttributesFilterInput>,
}
#[derive(Serialize, Deserialize)]
pub struct MetadataDownloadUrl_Content {
  metadata: Option<MetadataDownloadUrl_Metadata>,
}
#[derive(Serialize, Deserialize)]
pub struct SearchResultObject {
  documents: Vec<SearchDocument>,
  estimated_hits: i64,
}
#[derive(Serialize, Deserialize)]
pub struct FindMetadata_MetadataContent {
  type_: String,
  length: Option<i64>,
}
#[derive(Serialize, Deserialize)]
pub enum IMessageQueue {
  MessageQueue(MessageQueue),
}
impl IMessageQueue {
  fn archived_stats(&self) -> &MessageQueueStats {
    match self {
      IMessageQueue::MessageQueue(m) => &m.archived_stats,
    }
  }
  fn stats(&self) -> &MessageQueueStats {
    match self {
      IMessageQueue::MessageQueue(m) => &m.stats,
    }
  }
  fn name(&self) -> &String {
    match self {
      IMessageQueue::MessageQueue(m) => &m.name,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowExecutionId {
  WorkflowExecutionId(WorkflowExecutionId),
}
impl IWorkflowExecutionId {
  fn queue(&self) -> &String {
    match self {
      IWorkflowExecutionId::WorkflowExecutionId(m) => &m.queue,
    }
  }
  fn id(&self) -> &i64 {
    match self {
      IWorkflowExecutionId::WorkflowExecutionId(m) => &m.id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IGetMetadata_MetadataSupplementarySource {
  GetMetadata_MetadataSupplementarySource(GetMetadata_MetadataSupplementarySource),
}
impl IGetMetadata_MetadataSupplementarySource {
  fn identifier(&self) -> &Option<String> {
    match self {
      IGetMetadata_MetadataSupplementarySource::GetMetadata_MetadataSupplementarySource(m) => &m.identifier,
    }
  }
  fn id(&self) -> &String {
    match self {
      IGetMetadata_MetadataSupplementarySource::GetMetadata_MetadataSupplementarySource(m) => &m.id,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowStates {
  WorkflowStates(WorkflowStates),
}
impl IWorkflowStates {
  fn state(&self) -> &Option<WorkflowState> {
    match self {
      IWorkflowStates::WorkflowStates(m) => &m.state,
    }
  }
  fn all(&self) -> &Vec<WorkflowState> {
    match self {
      IWorkflowStates::WorkflowStates(m) => &m.all,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISourceById_Source {
  SourceById_Source(SourceById_Source),
}
impl ISourceById_Source {
  fn id(&self) -> &String {
    match self {
      ISourceById_Source::SourceById_Source(m) => &m.id,
    }
  }
  fn name(&self) -> &String {
    match self {
      ISourceById_Source::SourceById_Source(m) => &m.name,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ActivityParameterType {
  CONTEXT,
  SUPPLEMENTARY,
  SUPPLEMENTARY_ARRAY,
}
#[derive(Serialize, Deserialize)]
pub struct SetMetadataPublic_Query {
  content: SetMetadataPublic_Content,
}
#[derive(Serialize, Deserialize)]
pub struct EnqueueChildWorkflow_Query {
  workflows: EnqueueChildWorkflow_Workflows,
}
#[derive(Serialize, Deserialize)]
pub enum IFindCollection_Query {
  FindCollection_Query(FindCollection_Query),
}
impl IFindCollection_Query {
  fn content(&self) -> &FindCollection_Content {
    match self {
      IFindCollection_Query::FindCollection_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ISetWorkflowJobComplete_Workflows {
  SetWorkflowJobComplete_Workflows(SetWorkflowJobComplete_Workflows),
}
impl ISetWorkflowJobComplete_Workflows {
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataWorkflow {
  MetadataWorkflow(MetadataWorkflow),
}
impl IMetadataWorkflow {
  fn state(&self) -> &String {
    match self {
      IMetadataWorkflow::MetadataWorkflow(m) => &m.state,
    }
  }
  fn pending(&self) -> &Option<String> {
    match self {
      IMetadataWorkflow::MetadataWorkflow(m) => &m.pending,
    }
  }
  fn plans(&self) -> &Vec<WorkflowExecutionPlan> {
    match self {
      IMetadataWorkflow::MetadataWorkflow(m) => &m.plans,
    }
  }
  fn delete_workflow(&self) -> &Option<String> {
    match self {
      IMetadataWorkflow::MetadataWorkflow(m) => &m.delete_workflow,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct Mutation {
  content: ContentMutation,
  workflows: WorkflowsMutation,
  security: SecurityMutation,
  queues: QueuesMutation,
}
#[derive(Serialize, Deserialize)]
pub enum ISetMetadataPublic_Metadata {
  SetMetadataPublic_Metadata(SetMetadataPublic_Metadata),
}
impl ISetMetadataPublic_Metadata {
}
#[derive(Serialize, Deserialize)]
pub enum IWorkflowActivityModelInput {
  WorkflowActivityModelInput(WorkflowActivityModelInput),
}
impl IWorkflowActivityModelInput {
  fn model_id(&self) -> &String {
    match self {
      IWorkflowActivityModelInput::WorkflowActivityModelInput(m) => &m.model_id,
    }
  }
  fn configuration(&self) -> &Value {
    match self {
      IWorkflowActivityModelInput::WorkflowActivityModelInput(m) => &m.configuration,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IMetadataWorkflowState {
  MetadataWorkflowState(MetadataWorkflowState),
}
impl IMetadataWorkflowState {
  fn state_id(&self) -> &String {
    match self {
      IMetadataWorkflowState::MetadataWorkflowState(m) => &m.state_id,
    }
  }
  fn immediate(&self) -> &bool {
    match self {
      IMetadataWorkflowState::MetadataWorkflowState(m) => &m.immediate,
    }
  }
  fn metadata_id(&self) -> &String {
    match self {
      IMetadataWorkflowState::MetadataWorkflowState(m) => &m.metadata_id,
    }
  }
  fn status(&self) -> &String {
    match self {
      IMetadataWorkflowState::MetadataWorkflowState(m) => &m.status,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct SetWorkflowPlanContext_Query {
  workflows: SetWorkflowPlanContext_Workflows,
}
#[derive(Serialize, Deserialize)]
pub enum IPermissionInput {
  PermissionInput(PermissionInput),
}
impl IPermissionInput {
  fn group_id(&self) -> &String {
    match self {
      IPermissionInput::PermissionInput(m) => &m.group_id,
    }
  }
  fn entity_id(&self) -> &String {
    match self {
      IPermissionInput::PermissionInput(m) => &m.entity_id,
    }
  }
  fn action(&self) -> &PermissionAction {
    match self {
      IPermissionInput::PermissionInput(m) => &m.action,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IPlan_Query {
  Plan_Query(Plan_Query),
}
impl IPlan_Query {
  fn workflows(&self) -> &Plan_Workflows {
    match self {
      IPlan_Query::Plan_Query(m) => &m.workflows,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub struct Login_Security {
  login: Login_Login,
}
#[derive(Serialize, Deserialize)]
pub struct AddMetadataSupplementary_Metadata {
}
#[derive(Serialize, Deserialize)]
pub struct ModelsMutation {
  add: Option<Model>,
}
#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
  principal: Principal,
  token: Token,
}
#[derive(Serialize, Deserialize)]
pub enum IGetMetadata_MetadataSupplementary {
  GetMetadata_MetadataSupplementary(GetMetadata_MetadataSupplementary),
}
impl IGetMetadata_MetadataSupplementary {
  fn uploaded(&self) -> &Option<String> {
    match self {
      IGetMetadata_MetadataSupplementary::GetMetadata_MetadataSupplementary(m) => &m.uploaded,
    }
  }
  fn key(&self) -> &String {
    match self {
      IGetMetadata_MetadataSupplementary::GetMetadata_MetadataSupplementary(m) => &m.key,
    }
  }
  fn content(&self) -> &GetMetadata_MetadataSupplementaryContent {
    match self {
      IGetMetadata_MetadataSupplementary::GetMetadata_MetadataSupplementary(m) => &m.content,
    }
  }
  fn source(&self) -> &GetMetadata_MetadataSupplementarySource {
    match self {
      IGetMetadata_MetadataSupplementary::GetMetadata_MetadataSupplementary(m) => &m.source,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ILogin {
  Login(Login),
  Login_Login(Login_Login),
}
impl ILogin {
}
#[derive(Serialize, Deserialize)]
pub struct Query {
  content: Content,
  workflows: Workflows,
  security: Security,
  queues: Queues,
}
#[derive(Serialize, Deserialize)]
pub struct CollectionMutation {
  add: Collection,
  add_bulk: Vec<Collection>,
  edit: Collection,
  delete: bool,
  set_public: Collection,
  set_public_list: Collection,
  add_permission: Permission,
  delete_permission: Permission,
  set_child_item_attributes: Collection,
  add_child_collection: Collection,
  remove_child_collection: Collection,
  add_child_metadata: Collection,
  remove_child_metadata: Collection,
  set_workflow_state: bool,
  set_workflow_state_complete: bool,
  set_collection_attributes: bool,
  set_collection_ordering: bool,
  set_ready: bool,
}
#[derive(Serialize, Deserialize)]
pub enum ISetCollectionWorkflowState_Query {
  SetCollectionWorkflowState_Query(SetCollectionWorkflowState_Query),
}
impl ISetCollectionWorkflowState_Query {
  fn content(&self) -> &SetCollectionWorkflowState_Content {
    match self {
      ISetCollectionWorkflowState_Query::SetCollectionWorkflowState_Query(m) => &m.content,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum ILogin_Login {
  Login_Login(Login_Login),
}
impl ILogin_Login {
  fn password(&self) -> &Login_LoginResponse {
    match self {
      ILogin_Login::Login_Login(m) => &m.password,
    }
  }
}
#[derive(Serialize, Deserialize)]
pub enum IStorageSystemType {
  StorageSystemType(StorageSystemType),
}
impl IStorageSystemType {
}
#[derive(Serialize, Deserialize)]
pub enum IGetCollectionItems_Collection {
  GetCollectionItems_Collection(GetCollectionItems_Collection),
}
impl IGetCollectionItems_Collection {
  fn items(&self) -> &Vec<GetCollectionItems_CollectionItem> {
    match self {
      IGetCollectionItems_Collection::GetCollectionItems_Collection(m) => &m.items,
    }
  }
}
