// noinspection JSUnusedGlobalSymbols

export interface PlanInput extends IPlanInput, InputObject {
  queue: string
}
export interface AddChildCollectionInput extends IAddChildCollectionInput, InputObject {
  id: string
  collectionId: string
}
export interface Model extends IModel {
  __typename?: 'Model'
  id: string
  type: string
  name: string
  description: string
  configuration: any
}
export interface SupplementaryUploadUrl_Content extends IContent, ISupplementaryUploadUrl_Content {
  __typename?: 'Content'
  metadataSupplementary?: SupplementaryUploadUrl_MetadataSupplementary | null
}
export interface SetMetadataPublic_MetadataMutation extends IMetadataMutation, ISetMetadataPublic_MetadataMutation {
  __typename?: 'MetadataMutation'
  setPublic: SetMetadataPublic_Metadata
}
export interface SearchDocument extends ISearchDocument {
  __typename?: 'SearchDocument'
  metadata?: Metadata | null
  collection?: Collection | null
  content: string
}
export interface EnqueueChildWorkflowsInput extends IEnqueueChildWorkflowsInput, InputObject {
  jobId: WorkflowExecutionIdInput
  workflowIds: string
}
export interface ActivitiesMutation extends IActivitiesMutation {
  __typename?: 'ActivitiesMutation'
  add?: Activity | null
  edit?: Activity | null
  delete: boolean
}
export interface Security extends ISecurity {
  __typename?: 'Security'
  principal: Principal
  login: Login
}
export interface SetWorkflowJobFailed_Mutation extends IMutation, ISetWorkflowJobFailed_Mutation {
  __typename?: 'Mutation'
  workflows: SetWorkflowJobFailed_WorkflowsMutation
}
export interface MetadataDownloadUrlInput extends IMetadataDownloadUrlInput, InputObject {
  id: string
}
export interface ILogin_Principal {
  __typename?: string | null
  id: string
}
export interface ISetCollectionWorkflowStateCompleteInput {
}
export interface MetadataContent extends IMetadataContent {
  __typename?: 'MetadataContent'
  type: string
  length?: number | null
  urls: MetadataContentUrls
  text: string
  json: any
}
export interface Message extends IMessage {
  __typename?: 'Message'
  id: number
  visibleTimeout: Date
  value: any
}
export interface MetadataSupplementary extends IMetadataSupplementary {
  __typename?: 'MetadataSupplementary'
  metadataId: string
  key: string
  name: string
  created: string
  modified: string
  attributes?: any | null
  uploaded?: string | null
  content: MetadataSupplementaryContent
  source: MetadataSupplementarySource
}
export interface FindMetadata_MetadataContent extends IMetadataContent, IFindMetadata_MetadataContent {
  __typename?: 'MetadataContent'
  type: string
  length?: number | null
}
export interface SetCollectionWorkflowStateComplete_ContentMutation extends IContentMutation, ISetCollectionWorkflowStateComplete_ContentMutation {
  __typename?: 'ContentMutation'
  collection: SetCollectionWorkflowStateComplete_CollectionMutation
}
export interface ISetCollectionPublic_Collection {
  __typename?: string | null
  id: string
}
export interface MetadataChildInput extends IMetadataChildInput {
  metadata: MetadataInput
  attributes?: any | null
}
export interface AddMetadataBulk_MetadataMutation extends IMetadataMutation, IAddMetadataBulk_MetadataMutation {
  __typename?: 'MetadataMutation'
  addBulk: AddMetadataBulk_Metadata[]
}
export interface MetadataInput extends IMetadataInput {
  parentCollectionId?: string | null
  parentId?: string | null
  version?: number | null
  metadataType?: MetadataType | null
  name: string
  contentType: string
  contentLength?: number | null
  languageTag: string
  labels: string[]
  traitIds: string[]
  categoryIds: string[]
  attributes?: any | null
  state?: MetadataWorkflowInput | null
  source?: MetadataSourceInput | null
  index?: boolean | null
  ready?: boolean | null
}
export interface GetCollectionInput extends IGetCollectionInput, InputObject {
  id: string
}
export interface MetadataDownloadUrl_SignedUrl extends ISignedUrl, IMetadataDownloadUrl_SignedUrl {
  __typename?: 'SignedUrl'
  url: string
  headers: MetadataDownloadUrl_SignedUrlHeader[]
}
export interface SetWorkflowJobCompleteInput extends ISetWorkflowJobCompleteInput, InputObject {
  jobId: WorkflowJobIdInput
}
export interface GetCollectionItems_CollectionItem extends ICollectionItem, IGetCollectionItems_CollectionItem {
  __typename?: 'CollectionItem'
}
export interface IMetadataInput {
  version?: number | null
  name: string
  source?: MetadataSourceInput | null
  metadataType?: MetadataType | null
  state?: MetadataWorkflowInput | null
  languageTag: string
  parentCollectionId?: string | null
  attributes?: any | null
  ready?: boolean | null
  categoryIds: string[]
  parentId?: string | null
  labels: string[]
  index?: boolean | null
  traitIds: string[]
  contentLength?: number | null
  contentType: string
}
export interface AttributesFilterInput extends IAttributesFilterInput {
  attributes: string[]
  childAttributes?: AttributesFilterInput | null
}
export interface IGetMetadata_MetadataSupplementaryContent {
  __typename?: string | null
  length?: number | null
  type: string
}
export interface MetadataSupplementaryContent extends IMetadataSupplementaryContent {
  __typename?: 'MetadataSupplementaryContent'
  type: string
  length?: number | null
  urls: MetadataSupplementaryContentUrls
  text: string
  json: any
}
export interface SetWorkflowPlanContextInput extends ISetWorkflowPlanContextInput, InputObject {
  planId: WorkflowExecutionIdInput
  context: any
}
export interface GetCollectionItemsInput extends IGetCollectionItemsInput, InputObject {
  id: string
  offset: number
  limit: number
}
export interface IEnqueueChildWorkflowInput {
}
export interface WorkflowStateInput extends IWorkflowStateInput {
  id: string
  name: string
  description: string
  type: WorkflowStateType
  configuration: any
  workflowId?: string | null
  entryWorkflowId?: string | null
  exitWorkflowId?: string | null
}
export interface IWorkflowActivity {
  __typename?: string | null
  inputs: WorkflowActivityParameter[]
  configuration: any
  id: number
  outputs: WorkflowActivityParameter[]
  executionGroup: number
  queue: string
}
export interface MetadataDownloadUrl_Metadata extends IMetadata, IMetadataDownloadUrl_Metadata {
  __typename?: 'Metadata'
  content: MetadataDownloadUrl_MetadataContent
}
export interface Login_Security extends ISecurity, ILogin_Security {
  __typename?: 'Security'
  login: Login_Login
}
export interface AddMetadataBulkInput extends IAddMetadataBulkInput, InputObject {
  metadatas: MetadataChildInput
}
export interface SetMetadataAttributesInput extends ISetMetadataAttributesInput, InputObject {
  id: string
  attributes: any
}
export interface IFindMetadata_MetadataSupplementaryContent {
  __typename?: string | null
  length?: number | null
  type: string
}
export interface FindMetadata_Content extends IContent, IFindMetadata_Content {
  __typename?: 'Content'
  findMetadata: FindMetadata_Metadata[]
}
export interface TraitById_Content extends IContent, ITraitById_Content {
  __typename?: 'Content'
  trait?: TraitById_Trait | null
}
export interface EnqueueChildWorkflows_Mutation extends IMutation, IEnqueueChildWorkflows_Mutation {
  __typename?: 'Mutation'
  workflows: EnqueueChildWorkflows_WorkflowsMutation
}
export interface IGetMetadata_MetadataSupplementary {
  __typename?: string | null
  uploaded?: string | null
  source: GetMetadata_MetadataSupplementarySource
  key: string
  content: GetMetadata_MetadataSupplementaryContent
}
export interface ISetCollectionPublic_ContentMutation {
  __typename?: string | null
  collection: SetCollectionPublic_CollectionMutation
}
export interface IEnqueueChildWorkflows_WorkflowsMutation {
  __typename?: string | null
  enqueueChildWorkflows: EnqueueChildWorkflows_WorkflowExecutionId[]
}
export interface IInputObject {
}
export interface IAddMetadataBulkInput {
}
export interface IGetCollectionItems_Query {
  __typename?: string | null
  content: GetCollectionItems_Content
}
export interface IMetadataUploadUrl_MetadataContent {
  __typename?: string | null
  urls: MetadataUploadUrl_MetadataContentUrls
  type: string
}
export interface IAddSearchDocuments_Mutation {
  __typename?: string | null
  content: AddSearchDocuments_ContentMutation
}
export interface ISupplementaryDownloadUrl_MetadataSupplementaryContent {
  __typename?: string | null
  urls: SupplementaryDownloadUrl_MetadataSupplementaryContentUrls
}
export interface ISetCollectionPublicList_Collection {
  __typename?: string | null
  id: string
}
export interface ISupplementaryUploadUrl_SignedUrl {
  __typename?: string | null
  headers: SupplementaryUploadUrl_SignedUrlHeader[]
  url: string
}
export interface WorkflowState extends IWorkflowState {
  __typename?: 'WorkflowState'
  id: string
  type: WorkflowStateType
  name: string
  description: string
  configuration: any
  workflowId?: string | null
  entryWorkflowId?: string | null
  exitWorkflowId?: string | null
}
export interface WorkflowStates extends IWorkflowStates {
  __typename?: 'WorkflowStates'
  all: WorkflowState[]
  state?: WorkflowState | null
}
export interface IAddMetadataBulk_Mutation {
  __typename?: string | null
  content: AddMetadataBulk_ContentMutation
}
export enum WorkflowStateType {
  PROCESSING = 'PROCESSING',
  DRAFT = 'DRAFT',
  PENDING = 'PENDING',
  APPROVAL = 'APPROVAL',
  APPROVED = 'APPROVED',
  PUBLISHED = 'PUBLISHED',
  FAILURE = 'FAILURE',
}
export interface AddMetadata_Mutation extends IMutation, IAddMetadata_Mutation {
  __typename?: 'Mutation'
  content: AddMetadata_ContentMutation
}
export interface SetWorkflowJobFailed_WorkflowsMutation extends IWorkflowsMutation, ISetWorkflowJobFailed_WorkflowsMutation {
  __typename?: 'WorkflowsMutation'
  setExecutionPlanJobFailed: boolean
}
export interface IMetadataDownloadUrl_SignedUrl {
  __typename?: string | null
  headers: MetadataDownloadUrl_SignedUrlHeader[]
  url: string
}
export interface IAddMetadataInput {
}
export interface GetMetadata_Metadata extends IMetadata, IGetMetadata_Metadata {
  __typename?: 'Metadata'
  id: string
  version: number
  traitIds: string[]
  name: string
  content: GetMetadata_MetadataContent
  languageTag: string
  labels: string[]
  attributes: any
  created: Date
  modified: Date
  source: GetMetadata_MetadataSource
  supplementary: GetMetadata_MetadataSupplementary[]
}
export interface SetCollectionWorkflowStateInput extends ISetCollectionWorkflowStateInput, InputObject {
  state: CollectionWorkflowState
}
export interface SetWorkflowPlanContext_Mutation extends IMutation, ISetWorkflowPlanContext_Mutation {
  __typename?: 'Mutation'
  workflows: SetWorkflowPlanContext_WorkflowsMutation
}
export interface AddSearchDocumentsInput extends IAddSearchDocumentsInput, InputObject {
  storageSystemId: string
  documents: SearchDocumentInput
}
export interface SetCollectionPublic_Mutation extends IMutation, ISetCollectionPublic_Mutation {
  __typename?: 'Mutation'
  content: SetCollectionPublic_ContentMutation
}
export interface SetWorkflowState_Mutation extends IMutation, ISetWorkflowState_Mutation {
  __typename?: 'Mutation'
  content: SetWorkflowState_ContentMutation
}
export interface ISetWorkflowJobCompleteInput {
}
export interface WorkflowActivityStorageSystemInput extends IWorkflowActivityStorageSystemInput {
  systemId: string
  configuration: any
}
export interface SetCollectionPublicList_Collection extends ICollection, ISetCollectionPublicList_Collection {
  __typename?: 'Collection'
  id: string
}
export interface SetWorkflowStateInput extends ISetWorkflowStateInput, InputObject {
  state: MetadataWorkflowState
}
export interface IMetadataDownloadUrlInput {
}
export interface ISignedUrlHeader {
  __typename?: string | null
  name: string
  value: string
}
export interface IGetCollection_Query {
  __typename?: string | null
  content: GetCollection_Content
}
export interface AddMetadataInput extends IAddMetadataInput, InputObject {
  metadata: MetadataInput
}
export interface SupplementaryDownloadUrl_MetadataSupplementary extends IMetadataSupplementary, ISupplementaryDownloadUrl_MetadataSupplementary {
  __typename?: 'MetadataSupplementary'
  metadataId: string
  key: string
  content: SupplementaryDownloadUrl_MetadataSupplementaryContent
}
export interface ILoginResponse {
  __typename?: string | null
}
export interface IAddChildMetadata_CollectionMutation {
  __typename?: string | null
  addChildMetadata: AddChildMetadata_Collection
}
export interface ISourceById_Source {
  __typename?: string | null
  name: string
  id: string
}
export interface FindMetadata_Metadata extends IMetadata, IFindMetadata_Metadata {
  __typename?: 'Metadata'
  id: string
  version: number
  traitIds: string[]
  name: string
  content: FindMetadata_MetadataContent
  languageTag: string
  labels: string[]
  attributes: any
  created: Date
  modified: Date
  source: FindMetadata_MetadataSource
  supplementary: FindMetadata_MetadataSupplementary[]
}
export interface ISetExecutionPlanJobCheckin_Mutation {
  __typename?: string | null
  workflows: SetExecutionPlanJobCheckin_WorkflowsMutation
}
export interface ISetMetadataSystemAttributes_MetadataMutation {
  __typename?: string | null
  setMetadataSystemAttributes: boolean
}
export interface ISetCollectionWorkflowState_ContentMutation {
  __typename?: string | null
  collection: SetCollectionWorkflowState_CollectionMutation
}
export interface SecurityMutation extends ISecurityMutation {
  __typename?: 'SecurityMutation'
  signup: Principal
  addPrincipalGroup: boolean
}
export interface IGetCollectionItems_CollectionItem {
  __typename?: string | null
}
export interface ISourceByIdInput {
}
export interface ISetWorkflowPlanContext_Mutation {
  __typename?: string | null
  workflows: SetWorkflowPlanContext_WorkflowsMutation
}
export interface IWorkflows {
  __typename?: string | null
}
export interface SetMetadataReady_Mutation extends IMutation, ISetMetadataReady_Mutation {
  __typename?: 'Mutation'
  content: SetMetadataReady_ContentMutation
}
export interface IFindCollection_Content {
  __typename?: string | null
  findCollection: FindCollection_Collection[]
}
export interface IFindMetadata_Metadata {
  __typename?: string | null
  languageTag: string
  attributes: any
  content: FindMetadata_MetadataContent
  name: string
  id: string
  traitIds: string[]
  labels: string[]
  source: FindMetadata_MetadataSource
  modified: Date
  created: Date
  supplementary: FindMetadata_MetadataSupplementary[]
  version: number
}
export interface IEnqueueChildWorkflow_Mutation {
  __typename?: string | null
  workflows: EnqueueChildWorkflow_WorkflowsMutation
}
export interface PromptsMutation extends IPromptsMutation {
  __typename?: 'PromptsMutation'
  add?: Prompt | null
  edit?: Prompt | null
  delete: boolean
}
export interface IEnqueueChildWorkflow_WorkflowExecutionId {
  __typename?: string | null
  queue: string
  id: number
}
export interface IAddMetadata_MetadataMutation {
  __typename?: string | null
  add: AddMetadata_Metadata
}
export interface IActivityParameterType {
  __typename?: string | null
}
export interface SetWorkflowJobComplete_Mutation extends IMutation, ISetWorkflowJobComplete_Mutation {
  __typename?: 'Mutation'
  workflows: SetWorkflowJobComplete_WorkflowsMutation
}
export interface IFindMetadata_MetadataSource {
  __typename?: string | null
  identifier?: string | null
  id?: string | null
}
export interface SetCollectionReady_ContentMutation extends IContentMutation, ISetCollectionReady_ContentMutation {
  __typename?: 'ContentMutation'
  collection: SetCollectionReady_CollectionMutation
}
export interface IMetadataChildInput {
  metadata: MetadataInput
  attributes?: any | null
}
export interface SetMetadataSystemAttributes_ContentMutation extends IContentMutation, ISetMetadataSystemAttributes_ContentMutation {
  __typename?: 'ContentMutation'
  metadata: SetMetadataSystemAttributes_MetadataMutation
}
export interface GetMetadata_Content extends IContent, IGetMetadata_Content {
  __typename?: 'Content'
  metadata?: GetMetadata_Metadata | null
}
export interface WorkflowExecutionPlan extends WorkflowExecution, IWorkflowExecutionPlan {
  __typename?: 'WorkflowExecutionPlan'
  id: number
  parent?: WorkflowExecutionId | null
  workflow: Workflow
  next?: WorkflowJobId | null
  jobs: WorkflowJob[]
  metadataId?: string | null
  metadata?: Metadata | null
  version?: number | null
  collectionId?: string | null
  supplementaryId?: string | null
  context: any
  pending: WorkflowJobId[]
  current: WorkflowJobId[]
  running: WorkflowJobId[]
  complete: WorkflowJobId[]
  failed: WorkflowJobId[]
  error?: string | null
}
export interface ISetMetadataPublic_Metadata {
  __typename?: string | null
  id: string
}
export interface ICollectionWorkflowState {
  immediate: boolean
  collectionId: string
  stateId: string
  status: string
}
export interface ISetWorkflowState_ContentMutation {
  __typename?: string | null
  metadata: SetWorkflowState_MetadataMutation
}
export interface IGetCollectionItemsInput {
}
export interface AddMetadata_ContentMutation extends IContentMutation, IAddMetadata_ContentMutation {
  __typename?: 'ContentMutation'
  metadata: AddMetadata_MetadataMutation
}
export interface CollectionItem extends ICollectionItem {
  __typename?: string | null
  ready?: Date | null
  systemAttributes?: any | null
  name: string
  traitIds: string[]
  attributes: any
  parentCollections: Collection[]
  created: Date
  id: string
  permissions: Permission[]
  public: boolean
  modified: Date
  itemAttributes?: any | null
  labels: string[]
}
export interface IPrompt {
  __typename?: string | null
  systemPrompt: string
  outputType: string
  description: string
  name: string
  inputType: string
  id: string
  userPrompt: string
}
export interface SetExecutionPlanJobCheckin_Mutation extends IMutation, ISetExecutionPlanJobCheckin_Mutation {
  __typename?: 'Mutation'
  workflows: SetExecutionPlanJobCheckin_WorkflowsMutation
}
export interface AddCollection_Mutation extends IMutation, IAddCollection_Mutation {
  __typename?: 'Mutation'
  content: AddCollection_ContentMutation
}
export interface GetMetadata_MetadataSource extends IMetadataSource, IGetMetadata_MetadataSource {
  __typename?: 'MetadataSource'
  id?: string | null
  identifier?: string | null
}
export interface MessageQueue extends IMessageQueue {
  __typename?: 'MessageQueue'
  name: string
  stats: MessageQueueStats
  archivedStats: MessageQueueStats
}
export interface IGetMetadata_Content {
  __typename?: string | null
  metadata?: GetMetadata_Metadata | null
}
export interface SupplementaryUploadUrlInput extends ISupplementaryUploadUrlInput, InputObject {
  id: string
  key: string
}
export interface ISetCollectionReady_ContentMutation {
  __typename?: string | null
  collection: SetCollectionReady_CollectionMutation
}
export interface IGetMetadataInput {
}
export interface ICollectionWorkflow {
  __typename?: string | null
  deleteWorkflow?: string | null
  plans: WorkflowExecutionPlan[]
  pending?: string | null
  state: string
}
export interface SetWorkflowState_ContentMutation extends IContentMutation, ISetWorkflowState_ContentMutation {
  __typename?: 'ContentMutation'
  metadata: SetWorkflowState_MetadataMutation
}
export interface ISetCollectionPublicInput {
}
export interface MetadataUploadUrl_SignedUrlHeader extends ISignedUrlHeader, IMetadataUploadUrl_SignedUrlHeader {
  __typename?: 'SignedUrlHeader'
  name: string
  value: string
}
export interface WorkflowActivity extends IWorkflowActivity {
  __typename?: 'WorkflowActivity'
  id: number
  queue: string
  executionGroup: number
  configuration: any
  inputs: WorkflowActivityParameter[]
  outputs: WorkflowActivityParameter[]
}
export interface AddMetadataSupplementary_MetadataMutation extends IMetadataMutation, IAddMetadataSupplementary_MetadataMutation {
  __typename?: 'MetadataMutation'
  addSupplementary: AddMetadataSupplementary_MetadataSupplementary
}
export interface IWorkflowsMutation {
  __typename?: string | null
}
export interface SupplementaryUploadUrl_MetadataSupplementaryContentUrls extends IMetadataSupplementaryContentUrls, ISupplementaryUploadUrl_MetadataSupplementaryContentUrls {
  __typename?: 'MetadataSupplementaryContentUrls'
  upload: SupplementaryUploadUrl_SignedUrl
}
export interface CollectionWorkflowCompleteState extends ICollectionWorkflowCompleteState {
  collectionId: string
  status: string
}
export interface MetadataRelationshipInput extends IMetadataRelationshipInput {
  id1: string
  id2: string
  relationship: string
  attributes: any
}
export interface SupplementaryDownloadUrl_MetadataSupplementaryContent extends IMetadataSupplementaryContent, ISupplementaryDownloadUrl_MetadataSupplementaryContent {
  __typename?: 'MetadataSupplementaryContent'
  urls: SupplementaryDownloadUrl_MetadataSupplementaryContentUrls
}
export interface IAddChildCollection_Mutation {
  __typename?: string | null
  content: AddChildCollection_ContentMutation
}
export interface IFindCollection_Collection {
  __typename?: string | null
  labels: string[]
  attributes: any
  created: Date
  modified: Date
  type: CollectionType
  name: string
  id: string
}
export interface SetCollectionPublicList_Mutation extends IMutation, ISetCollectionPublicList_Mutation {
  __typename?: 'Mutation'
  content: SetCollectionPublicList_ContentMutation
}
export interface IAddChildCollectionInput {
}
export interface ISetCollectionPublicListInput {
}
export interface AddSearchDocuments_Mutation extends IMutation, IAddSearchDocuments_Mutation {
  __typename?: 'Mutation'
  content: AddSearchDocuments_ContentMutation
}
export interface SetCollectionPublicListInput extends ISetCollectionPublicListInput, InputObject {
  id: string
  public: boolean
}
export interface Mutation extends IMutation {
  __typename?: 'Mutation'
  content: ContentMutation
  workflows: WorkflowsMutation
  security: SecurityMutation
  queues: QueuesMutation
}
export interface Login_Principal extends IPrincipal, ILogin_Principal {
  __typename?: 'Principal'
  id: string
}
export interface IStorageSystemType {
  __typename?: string | null
}
export interface AddMetadata_Metadata extends IMetadata, IAddMetadata_Metadata {
  __typename?: 'Metadata'
  id: string
}
export interface IMetadataWorkflowInput {
  state: string
  deleteWorkflowId?: string | null
}
export interface IAddChildMetadata_ContentMutation {
  __typename?: string | null
  collection: AddChildMetadata_CollectionMutation
}
export interface SetMetadataPublic_Mutation extends IMutation, ISetMetadataPublic_Mutation {
  __typename?: 'Mutation'
  content: SetMetadataPublic_ContentMutation
}
export interface EnqueueJob_Mutation extends IMutation, IEnqueueJob_Mutation {
  __typename?: 'Mutation'
  workflows: EnqueueJob_WorkflowsMutation
}
export interface IEnqueueJob_WorkflowExecutionId {
  __typename?: string | null
  id: number
  queue: string
}
export interface IAddMetadataBulk_MetadataMutation {
  __typename?: string | null
  addBulk: AddMetadataBulk_Metadata[]
}
export interface ModelInput extends IModelInput {
  type: string
  name: string
  description: string
  configuration: any
}
export interface PermissionInput extends IPermissionInput {
  entityId: string
  groupId: string
  action: PermissionAction
}
export interface IModel {
  __typename?: string | null
  id: string
  type: string
  configuration: any
  name: string
  description: string
}
export interface IPrompts {
  __typename?: string | null
  prompt?: Prompt | null
  all: Prompt[]
}
export interface AddChildCollection_Collection extends ICollection, IAddChildCollection_Collection {
  __typename?: 'Collection'
  id: string
}
export interface SetMetadataAttributes_Mutation extends IMutation, ISetMetadataAttributes_Mutation {
  __typename?: 'Mutation'
  content: SetMetadataAttributes_ContentMutation
}
export interface FindMetadata_MetadataSupplementarySource extends IMetadataSupplementarySource, IFindMetadata_MetadataSupplementarySource {
  __typename?: 'MetadataSupplementarySource'
  id: string
  identifier?: string | null
}
export interface SetMetadataReady_MetadataMutation extends IMetadataMutation, ISetMetadataReady_MetadataMutation {
  __typename?: 'MetadataMutation'
  setMetadataReady: boolean
}
export interface IMetadataDownloadUrl_MetadataContent {
  __typename?: string | null
  type: string
  urls: MetadataDownloadUrl_MetadataContentUrls
}
export interface AddCollection_CollectionMutation extends ICollectionMutation, IAddCollection_CollectionMutation {
  __typename?: 'CollectionMutation'
  add: AddCollection_Collection
}
export interface IMetadataUploadUrl_Metadata {
  __typename?: string | null
  content: MetadataUploadUrl_MetadataContent
}
export interface ISetCollectionPublicList_Mutation {
  __typename?: string | null
  content: SetCollectionPublicList_ContentMutation
}
export interface TraitById_Query extends IQuery, ITraitById_Query {
  __typename?: 'Query'
  content: TraitById_Content
}
export interface IWorkflowActivityParameter {
  __typename?: string | null
  value: string
  name: string
}
export interface SetCollectionPublicList_CollectionMutation extends ICollectionMutation, ISetCollectionPublicList_CollectionMutation {
  __typename?: 'CollectionMutation'
  setPublicList: SetCollectionPublicList_Collection
}
export interface SetWorkflowJobFailedInput extends ISetWorkflowJobFailedInput, InputObject {
  jobId: WorkflowJobIdInput
  error: string
}
export interface ISetCollectionWorkflowState_Mutation {
  __typename?: string | null
  content: SetCollectionWorkflowState_ContentMutation
}
export interface WorkflowActivityModelInput extends IWorkflowActivityModelInput {
  modelId: string
  configuration: any
}
export interface WorkflowActivityInput extends IWorkflowActivityInput {
  activityId: string
  queue: string
  executionGroup: number
  description: string
  inputs: WorkflowActivityParameterInput
  outputs: WorkflowActivityParameterInput
  models: WorkflowActivityModelInput
  storageSystems: WorkflowActivityStorageSystemInput
  prompts: WorkflowActivityPromptInput
  configuration: any
}
export interface FindMetadata_MetadataSupplementary extends IMetadataSupplementary, IFindMetadata_MetadataSupplementary {
  __typename?: 'MetadataSupplementary'
  key: string
  uploaded?: string | null
  content: FindMetadata_MetadataSupplementaryContent
  source: FindMetadata_MetadataSupplementarySource
}
export interface Workflows extends IWorkflows {
  __typename?: 'Workflows'
  all: Workflow[]
  activities: Activities
  models: Models
  prompts: Prompts
  states: WorkflowStates
  storageSystems: StorageSystems
  transitions: Transition[]
  nextWorkflowExecution?: WorkflowExecution | null
  executionPlan?: WorkflowExecutionPlan | null
  executions: WorkflowExecution[]
}
export interface Login_LoginResponse extends ILoginResponse, ILogin_LoginResponse {
  __typename?: 'LoginResponse'
  principal: Login_Principal
  token: Login_Token
}
export interface IFindMetadata_MetadataSupplementary {
  __typename?: string | null
  key: string
  source: FindMetadata_MetadataSupplementarySource
  uploaded?: string | null
  content: FindMetadata_MetadataSupplementaryContent
}
export interface ISetMetadataPublic_ContentMutation {
  __typename?: string | null
  metadata: SetMetadataPublic_MetadataMutation
}
export enum ActivityParameterType {
  CONTEXT = 'CONTEXT',
  SUPPLEMENTARY = 'SUPPLEMENTARY',
  SUPPLEMENTARY_ARRAY = 'SUPPLEMENTARY_ARRAY',
}
export interface IGetMetadata_MetadataSupplementarySource {
  __typename?: string | null
  identifier?: string | null
  id: string
}
export interface IPlan_Query {
  __typename?: string | null
  workflows: Plan_Workflows
}
export interface IGetMetadata_Metadata {
  __typename?: string | null
  attributes: any
  content: GetMetadata_MetadataContent
  version: number
  languageTag: string
  labels: string[]
  source: GetMetadata_MetadataSource
  modified: Date
  created: Date
  id: string
  name: string
  traitIds: string[]
  supplementary: GetMetadata_MetadataSupplementary[]
}
export interface ISupplementaryUploadUrlInput {
}
export interface SupplementaryDownloadUrlInput extends ISupplementaryDownloadUrlInput, InputObject {
  id: string
  key: string
}
export interface ISetCollectionPublic_Mutation {
  __typename?: string | null
  content: SetCollectionPublic_ContentMutation
}
export interface EnqueueJobInput extends IEnqueueJobInput, InputObject {
  planId: WorkflowExecutionIdInput
  jobIndex: number
}
export interface WorkflowActivityModel extends IWorkflowActivityModel {
  __typename?: 'WorkflowActivityModel'
  configuration: any
  model: Model
}
export interface ISetWorkflowJobContextInput {
}
export interface IWorkflowExecutionIdInput {
  queue: string
  id: number
}
export interface MetadataUploadUrl_Content extends IContent, IMetadataUploadUrl_Content {
  __typename?: 'Content'
  metadata?: MetadataUploadUrl_Metadata | null
}
export interface PromptInput extends IPromptInput {
  name: string
  description: string
  systemPrompt: string
  userPrompt: string
  inputType: string
  outputType: string
}
export interface Queues extends IQueues {
  __typename?: 'Queues'
  messageQueues: MessageQueue[]
  getMessages: Message[]
  getMessage?: Message | null
}
export interface SetMetadataSystemAttributesInput extends ISetMetadataSystemAttributesInput, InputObject {
  id: string
  attributes: any
}
export interface IEnqueueJob_WorkflowsMutation {
  __typename?: string | null
  enqueueJob?: EnqueueJob_WorkflowExecutionId | null
}
export interface MetadataUploadUrl_MetadataContent extends IMetadataContent, IMetadataUploadUrl_MetadataContent {
  __typename?: 'MetadataContent'
  type: string
  urls: MetadataUploadUrl_MetadataContentUrls
}
export interface ISetWorkflowPlanContextInput {
}
export interface IContent {
  __typename?: string | null
}
export interface ICollectionItem {
  __typename?: string | null
}
export interface EnqueueChildWorkflow_Mutation extends IMutation, IEnqueueChildWorkflow_Mutation {
  __typename?: 'Mutation'
  workflows: EnqueueChildWorkflow_WorkflowsMutation
}
export interface ISetWorkflowStateComplete_Mutation {
  __typename?: string | null
  content: SetWorkflowStateComplete_ContentMutation
}
export interface IMetadataSource {
  __typename?: string | null
  identifier?: string | null
  id?: string | null
}
export interface ISetCollectionPublicList_CollectionMutation {
  __typename?: string | null
  setPublicList: SetCollectionPublicList_Collection
}
export interface WorkflowActivityPrompt extends IWorkflowActivityPrompt {
  __typename?: 'WorkflowActivityPrompt'
  configuration: any
  prompt: Prompt
}
export interface IWorkflowState {
  __typename?: string | null
  workflowId?: string | null
  description: string
  name: string
  exitWorkflowId?: string | null
  id: string
  type: WorkflowStateType
  entryWorkflowId?: string | null
  configuration: any
}
export interface SetWorkflowJobContext_Mutation extends IMutation, ISetWorkflowJobContext_Mutation {
  __typename?: 'Mutation'
  workflows: SetWorkflowJobContext_WorkflowsMutation
}
export interface ISetWorkflowPlanContext_WorkflowsMutation {
  __typename?: string | null
  setExecutionPlanContext: boolean
}
export interface IFindCollection_Query {
  __typename?: string | null
  content: FindCollection_Content
}
export interface IModelsMutation {
  __typename?: string | null
  add?: Model | null
}
export interface ISetMetadataPublicInput {
}
export interface ICollectionInput {
  ordering?: any | null
  collectionType?: CollectionType | null
  parentCollectionId?: string | null
  labels: string[]
  metadata: MetadataChildInput
  state?: CollectionWorkflowInput | null
  description?: string | null
  ready?: boolean | null
  attributes?: any | null
  index?: boolean | null
  name: string
  collections: CollectionChildInput
}
export interface FindCollection_Collection extends ICollection, IFindCollection_Collection {
  __typename?: 'Collection'
  id: string
  type: CollectionType
  name: string
  labels: string[]
  attributes: any
  created: Date
  modified: Date
}
export interface Activities extends IActivities {
  __typename?: 'Activities'
  all: Activity[]
}
export interface IGetCollectionItems_Collection {
  __typename?: string | null
  items: GetCollectionItems_CollectionItem[]
}
export interface SetCollectionWorkflowStateComplete_Mutation extends IMutation, ISetCollectionWorkflowStateComplete_Mutation {
  __typename?: 'Mutation'
  content: SetCollectionWorkflowStateComplete_ContentMutation
}
export interface MetadataContentUrls extends IMetadataContentUrls {
  __typename?: 'MetadataContentUrls'
  download: SignedUrl
  upload: SignedUrl
}
export interface AddChildCollection_Mutation extends IMutation, IAddChildCollection_Mutation {
  __typename?: 'Mutation'
  content: AddChildCollection_ContentMutation
}
export interface IToken {
  __typename?: string | null
  token: string
}
export interface ISetWorkflowState_MetadataMutation {
  __typename?: string | null
  setWorkflowState: boolean
}
export interface GetCollection_Query extends IQuery, IGetCollection_Query {
  __typename?: 'Query'
  content: GetCollection_Content
}
export interface ISecurityMutation {
  __typename?: string | null
  addPrincipalGroup: boolean
  signup: Principal
}
export interface ISignedUrl {
  __typename?: string | null
  url: string
}
export interface SetCollectionWorkflowStateCompleteInput extends ISetCollectionWorkflowStateCompleteInput, InputObject {
  state: CollectionWorkflowCompleteState
}
export interface ISetWorkflowJobFailedInput {
}
export interface ContentMutation extends IContentMutation {
  __typename?: 'ContentMutation'
  collection: CollectionMutation
  metadata: MetadataMutation
  reindex: boolean
}
export interface IAddMetadataSupplementary_Mutation {
  __typename?: string | null
  content: AddMetadataSupplementary_ContentMutation
}
export interface IPermissionAction {
  __typename?: string | null
}
export interface ILogin_Token {
  __typename?: string | null
  token: string
}
export interface Transition extends ITransition {
  __typename?: 'Transition'
  fromStateId: string
  toStateId: string
  description: string
}
export interface AddChildCollection_ContentMutation extends IContentMutation, IAddChildCollection_ContentMutation {
  __typename?: 'ContentMutation'
  collection: AddChildCollection_CollectionMutation
}
export interface SetMetadataReady_ContentMutation extends IContentMutation, ISetMetadataReady_ContentMutation {
  __typename?: 'ContentMutation'
  metadata: SetMetadataReady_MetadataMutation
}
export interface IAttributesFilterInput {
  childAttributes?: AttributesFilterInput | null
  attributes: string[]
}
export interface SetCollectionPublic_CollectionMutation extends ICollectionMutation, ISetCollectionPublic_CollectionMutation {
  __typename?: 'CollectionMutation'
  setPublic: SetCollectionPublic_Collection
}
export interface SetWorkflowPlanContext_WorkflowsMutation extends IWorkflowsMutation, ISetWorkflowPlanContext_WorkflowsMutation {
  __typename?: 'WorkflowsMutation'
  setExecutionPlanContext: boolean
}
export interface EnqueueChildWorkflow_WorkflowsMutation extends IWorkflowsMutation, IEnqueueChildWorkflow_WorkflowsMutation {
  __typename?: 'WorkflowsMutation'
  enqueueChildWorkflow: EnqueueChildWorkflow_WorkflowExecutionId
}
export interface ISetCollectionWorkflowStateComplete_CollectionMutation {
  __typename?: string | null
  setWorkflowStateComplete: boolean
}
export interface ISupplementaryDownloadUrl_SignedUrlHeader {
  __typename?: string | null
  value: string
  name: string
}
export interface IAddCollection_ContentMutation {
  __typename?: string | null
  collection: AddCollection_CollectionMutation
}
export interface SearchResultObject extends ISearchResultObject {
  __typename?: 'SearchResultObject'
  documents: SearchDocument[]
  estimatedHits: number
}
export interface ISetMetadataAttributesInput {
}
export interface CollectionInput extends ICollectionInput {
  parentCollectionId?: string | null
  collectionType?: CollectionType | null
  name: string
  description?: string | null
  labels: string[]
  attributes?: any | null
  ordering?: any | null
  state?: CollectionWorkflowInput | null
  index?: boolean | null
  collections: CollectionChildInput
  metadata: MetadataChildInput
  ready?: boolean | null
}
export interface SetMetadataAttributes_MetadataMutation extends IMetadataMutation, ISetMetadataAttributes_MetadataMutation {
  __typename?: 'MetadataMutation'
  setMetadataAttributes: boolean
}
export interface MetadataSupplementaryContentUrls extends IMetadataSupplementaryContentUrls {
  __typename?: 'MetadataSupplementaryContentUrls'
  download: SignedUrl
  upload: SignedUrl
}
export interface IMetadataContent {
  __typename?: string | null
  type: string
}
export interface IAddChildMetadata_Collection {
  __typename?: string | null
  id: string
}
export interface SourceById_Content extends IContent, ISourceById_Content {
  __typename?: 'Content'
  source?: SourceById_Source | null
}
export interface TraitById_Trait extends ITrait, ITraitById_Trait {
  __typename?: 'Trait'
  id: string
  workflowIds: string[]
}
export interface IMetadataRelationshipInput {
  id1: string
  relationship: string
  attributes: any
  id2: string
}
export interface ITraitById_Content {
  __typename?: string | null
  trait?: TraitById_Trait | null
}
export interface ISourceById_Content {
  __typename?: string | null
  source?: SourceById_Source | null
}
export interface IMetadataUploadUrl_MetadataContentUrls {
  __typename?: string | null
  upload: MetadataUploadUrl_SignedUrl
}
export interface LoginInput extends ILoginInput, InputObject {
  identifier: string
  password: string
}
export interface CollectionWorkflowInput extends ICollectionWorkflowInput {
  state: string
  deleteWorkflowId?: string | null
}
export interface IMetadataUploadUrl_Content {
  __typename?: string | null
  metadata?: MetadataUploadUrl_Metadata | null
}
export enum StorageSystemType {
  SEARCH = 'SEARCH',
  VECTOR = 'VECTOR',
  SUPPLEMENTARY = 'SUPPLEMENTARY',
}
export interface ISetCollectionReady_Mutation {
  __typename?: string | null
  content: SetCollectionReady_ContentMutation
}
export interface SetMetadataSystemAttributes_Mutation extends IMutation, ISetMetadataSystemAttributes_Mutation {
  __typename?: 'Mutation'
  content: SetMetadataSystemAttributes_ContentMutation
}
export interface ISupplementaryDownloadUrl_Query {
  __typename?: string | null
  content: SupplementaryDownloadUrl_Content
}
export interface ILogin_LoginResponse {
  __typename?: string | null
  token: Login_Token
  principal: Login_Principal
}
export interface AddMetadataSupplementary_Mutation extends IMutation, IAddMetadataSupplementary_Mutation {
  __typename?: 'Mutation'
  content: AddMetadataSupplementary_ContentMutation
}
export interface IWorkflowActivityParameterInput {
  name: string
  value: string
}
export interface Login_Token extends IToken, ILogin_Token {
  __typename?: 'Token'
  token: string
}
export interface AddChildMetadata_ContentMutation extends IContentMutation, IAddChildMetadata_ContentMutation {
  __typename?: 'ContentMutation'
  collection: AddChildMetadata_CollectionMutation
}
export interface Metadata extends CollectionItem, IMetadata {
  __typename?: 'Metadata'
  id: string
  parentId?: string | null
  version: number
  traitIds: string[]
  type: MetadataType
  name: string
  content: MetadataContent
  languageTag: string
  labels: string[]
  attributes: any
  itemAttributes?: any | null
  systemAttributes?: any | null
  created: Date
  modified: Date
  uploaded?: Date | null
  ready?: Date | null
  workflow: MetadataWorkflow
  source: MetadataSource
  public: boolean
  publicContent: boolean
  publicSupplementary: boolean
  permissions: Permission[]
  relationships: MetadataRelationship[]
  supplementary: MetadataSupplementary[]
  parentCollections: Collection[]
}
export interface Login extends ILogin {
  __typename?: 'Login'
  password: LoginResponse
}
export interface ISupplementaryDownloadUrl_MetadataSupplementaryContentUrls {
  __typename?: string | null
  download: SupplementaryDownloadUrl_SignedUrl
}
export interface SetWorkflowJobContext_WorkflowsMutation extends IWorkflowsMutation, ISetWorkflowJobContext_WorkflowsMutation {
  __typename?: 'WorkflowsMutation'
  setExecutionJobContext: boolean
}
export interface IMetadataUploadUrl_Query {
  __typename?: string | null
  content: MetadataUploadUrl_Content
}
export interface ISetMetadataSystemAttributes_Mutation {
  __typename?: string | null
  content: SetMetadataSystemAttributes_ContentMutation
}
export interface IAddChildMetadata_Mutation {
  __typename?: string | null
  content: AddChildMetadata_ContentMutation
}
export interface IAddMetadataSupplementary_MetadataSupplementary {
  __typename?: string | null
  key: string
}
export interface CollectionWorkflow extends ICollectionWorkflow {
  __typename?: 'CollectionWorkflow'
  state: string
  pending?: string | null
  deleteWorkflow?: string | null
  plans: WorkflowExecutionPlan[]
}
export interface AddChildMetadata_Mutation extends IMutation, IAddChildMetadata_Mutation {
  __typename?: 'Mutation'
  content: AddChildMetadata_ContentMutation
}
export interface SetWorkflowJobContextInput extends ISetWorkflowJobContextInput, InputObject {
  jobId: WorkflowExecutionIdInput
  context: any
}
export interface Content extends IContent {
  __typename?: 'Content'
  findCollection: Collection[]
  collection?: Collection | null
  findMetadata: Metadata[]
  metadata?: Metadata | null
  metadataSupplementary?: MetadataSupplementary | null
  sources: Source[]
  source?: Source | null
  traits: Trait[]
  trait?: Trait | null
  search: SearchResultObject
}
export interface IWorkflowActivityPrompt {
  __typename?: string | null
  prompt: Prompt
  configuration: any
}
export interface WorkflowActivityParameter extends IWorkflowActivityParameter {
  __typename?: 'WorkflowActivityParameter'
  name: string
  value: string
}
export interface WorkflowExecution extends IWorkflowExecution {
  __typename?: string | null
  supplementaryId?: string | null
  error?: string | null
  workflow: Workflow
  collectionId?: string | null
  version?: number | null
  context: any
  metadata?: Metadata | null
}
export interface ISetMetadataAttributes_MetadataMutation {
  __typename?: string | null
  setMetadataAttributes: boolean
}
export interface GetCollection_Collection extends ICollection, IGetCollection_Collection {
  __typename?: 'Collection'
  id: string
  name: string
  labels: string[]
  attributes: any
  created: Date
  modified: Date
}
export interface IMessageQueue {
  __typename?: string | null
  name: string
  archivedStats: MessageQueueStats
  stats: MessageQueueStats
}
export interface IMetadataWorkflow {
  __typename?: string | null
  plans: WorkflowExecutionPlan[]
  pending?: string | null
  deleteWorkflow?: string | null
  state: string
}
export interface FindMetadata_Query extends IQuery, IFindMetadata_Query {
  __typename?: 'Query'
  content: FindMetadata_Content
}
export interface FindMetadata_MetadataSupplementaryContent extends IMetadataSupplementaryContent, IFindMetadata_MetadataSupplementaryContent {
  __typename?: 'MetadataSupplementaryContent'
  type: string
  length?: number | null
}
export interface Workflow extends IWorkflow {
  __typename?: 'Workflow'
  id: string
  name: string
  queue: string
  description: string
  configuration: any
}
export interface SetCollectionPublic_Collection extends ICollection, ISetCollectionPublic_Collection {
  __typename?: 'Collection'
  id: string
}
export interface SetExecutionPlanJobCheckin_WorkflowsMutation extends IWorkflowsMutation, ISetExecutionPlanJobCheckin_WorkflowsMutation {
  __typename?: 'WorkflowsMutation'
  setExecutionPlanJobCheckin: boolean
}
export interface ISetExecutionPlanJobCheckinInput {
}
export interface FindCollectionInput extends IFindCollectionInput, InputObject {
  attributes: FindAttributeInput
}
export interface ISupplementaryDownloadUrl_SignedUrl {
  __typename?: string | null
  headers: SupplementaryDownloadUrl_SignedUrlHeader[]
  url: string
}
export interface WorkflowsMutation extends IWorkflowsMutation {
  __typename?: 'WorkflowsMutation'
  add: Workflow
  edit: Workflow
  delete: boolean
  models: ModelsMutation
  states: WorkflowStatesMutation
  activities: ActivitiesMutation
  prompts: PromptsMutation
  beginTransition: boolean
  enqueueChildWorkflows: WorkflowExecutionId[]
  enqueueChildWorkflow: WorkflowExecutionId
  enqueueJob?: WorkflowExecutionId | null
  findAndEnqueueWorkflow: WorkflowExecutionId[]
  enqueueWorkflow: WorkflowExecutionId
  setExecutionPlanContext: boolean
  setExecutionJobContext: boolean
  setExecutionPlanJobCheckin: boolean
  setExecutionPlanJobComplete: boolean
  setExecutionPlanJobFailed: boolean
}
export interface MetadataUploadUrl_Query extends IQuery, IMetadataUploadUrl_Query {
  __typename?: 'Query'
  content: MetadataUploadUrl_Content
}
export interface SourceById_Source extends ISource, ISourceById_Source {
  __typename?: 'Source'
  id: string
  name: string
}
export interface FindCollection_Content extends IContent, IFindCollection_Content {
  __typename?: 'Content'
  findCollection: FindCollection_Collection[]
}
export interface IMetadataSourceInput {
  id: string
  identifier: string
}
export interface AddChildCollection_CollectionMutation extends ICollectionMutation, IAddChildCollection_CollectionMutation {
  __typename?: 'CollectionMutation'
  addChildCollection: AddChildCollection_Collection
}
export interface EnqueueJob_WorkflowExecutionId extends IWorkflowExecutionId, IEnqueueJob_WorkflowExecutionId {
  __typename?: 'WorkflowExecutionId'
  queue: string
  id: number
}
export interface Login_Login extends ILogin, ILogin_Login {
  __typename?: 'Login'
  password: Login_LoginResponse
}
export interface IAddMetadataSupplementary_MetadataMutation {
  __typename?: string | null
  addSupplementary: AddMetadataSupplementary_MetadataSupplementary
}
export interface AddChildMetadata_Collection extends ICollection, IAddChildMetadata_Collection {
  __typename?: 'Collection'
  id: string
}
export interface AddCollection_ContentMutation extends IContentMutation, IAddCollection_ContentMutation {
  __typename?: 'ContentMutation'
  collection: AddCollection_CollectionMutation
}
export interface CollectionWorkflowState extends ICollectionWorkflowState {
  collectionId: string
  stateId: string
  status: string
  immediate: boolean
}
export interface ISetWorkflowJobFailed_WorkflowsMutation {
  __typename?: string | null
  setExecutionPlanJobFailed: boolean
}
export interface IMetadataDownloadUrl_Metadata {
  __typename?: string | null
  content: MetadataDownloadUrl_MetadataContent
}
export interface AddCollectionInput extends IAddCollectionInput, InputObject {
  collection: CollectionInput
}
export interface IWorkflowActivityStorageSystemInput {
  configuration: any
  systemId: string
}
export interface SetCollectionWorkflowStateComplete_CollectionMutation extends ICollectionMutation, ISetCollectionWorkflowStateComplete_CollectionMutation {
  __typename?: 'CollectionMutation'
  setWorkflowStateComplete: boolean
}
export interface Source extends ISource {
  __typename?: 'Source'
  id: string
  name: string
  description: string
  configuration: any
}
export interface IAddMetadata_Mutation {
  __typename?: string | null
  content: AddMetadata_ContentMutation
}
export interface IActivityParameter {
  __typename?: string | null
  name: string
  type: ActivityParameterType
}
export interface ISetWorkflowStateComplete_ContentMutation {
  __typename?: string | null
  metadata: SetWorkflowStateComplete_MetadataMutation
}
export interface ICollection {
  __typename?: string | null
}
export interface ISetWorkflowJobComplete_WorkflowsMutation {
  __typename?: string | null
  setExecutionPlanJobComplete: boolean
}
export interface IFindMetadata_Content {
  __typename?: string | null
  findMetadata: FindMetadata_Metadata[]
}
export interface GetMetadata_MetadataSupplementary extends IMetadataSupplementary, IGetMetadata_MetadataSupplementary {
  __typename?: 'MetadataSupplementary'
  key: string
  uploaded?: string | null
  content: GetMetadata_MetadataSupplementaryContent
  source: GetMetadata_MetadataSupplementarySource
}
export interface IActivities {
  __typename?: string | null
  all: Activity[]
}
export interface AddChildMetadata_CollectionMutation extends ICollectionMutation, IAddChildMetadata_CollectionMutation {
  __typename?: 'CollectionMutation'
  addChildMetadata: AddChildMetadata_Collection
}
export interface ITraitById_Query {
  __typename?: string | null
  content: TraitById_Content
}
export interface IGetMetadata_MetadataContent {
  __typename?: string | null
  length?: number | null
  type: string
}
export interface IContentMutation {
  __typename?: string | null
}
export interface SetCollectionPublicList_ContentMutation extends IContentMutation, ISetCollectionPublicList_ContentMutation {
  __typename?: 'ContentMutation'
  collection: SetCollectionPublicList_CollectionMutation
}
export interface IWorkflowConfigurationInput {
  configuration: any
  activityId: string
}
export interface ISetMetadataReady_Mutation {
  __typename?: string | null
  content: SetMetadataReady_ContentMutation
}
export interface IWorkflowExecutionId {
  __typename?: string | null
  queue: string
  id: number
}
export interface SetMetadataPublicInput extends ISetMetadataPublicInput, InputObject {
  id: string
  public: boolean
}
export interface Permission extends IPermission {
  __typename?: 'Permission'
  groupId: string
  group: Group
  action: PermissionAction
}
export interface SearchQuery extends ISearchQuery {
  storageSystemId: string
  query: string
  filter?: string | null
  offset?: number | null
  limit?: number | null
}
export interface AddMetadataBulk_ContentMutation extends IContentMutation, IAddMetadataBulk_ContentMutation {
  __typename?: 'ContentMutation'
  metadata: AddMetadataBulk_MetadataMutation
}
export interface WorkflowActivityParameterInput extends IWorkflowActivityParameterInput {
  name: string
  value: string
}
export interface SupplementaryUploadUrl_MetadataSupplementaryContent extends IMetadataSupplementaryContent, ISupplementaryUploadUrl_MetadataSupplementaryContent {
  __typename?: 'MetadataSupplementaryContent'
  type: string
  urls: SupplementaryUploadUrl_MetadataSupplementaryContentUrls
}
export interface ILogin {
  __typename?: string | null
}
export interface SetCollectionPublicInput extends ISetCollectionPublicInput, InputObject {
  id: string
  public: boolean
}
export interface IMetadataRelationship {
  __typename?: string | null
  relationship: string
  id: string
  metadata: Metadata
  attributes: any
}
export interface ISetMetadataPublic_Mutation {
  __typename?: string | null
  content: SetMetadataPublic_ContentMutation
}
export interface WorkflowActivityStorageSystem extends IWorkflowActivityStorageSystem {
  __typename?: 'WorkflowActivityStorageSystem'
  configuration: any
  system: StorageSystem
}
export interface ISupplementaryUploadUrl_MetadataSupplementaryContentUrls {
  __typename?: string | null
  upload: SupplementaryUploadUrl_SignedUrl
}
export interface MetadataSourceInput extends IMetadataSourceInput {
  id: string
  identifier: string
}
export interface ICollectionType {
  __typename?: string | null
}
export interface ActivityParameter extends IActivityParameter {
  __typename?: 'ActivityParameter'
  name: string
  type: ActivityParameterType
}
export interface ISetMetadataReady_MetadataMutation {
  __typename?: string | null
  setMetadataReady: boolean
}
export interface ISetCollectionReadyInput {
}
export interface StorageSystemModel extends IStorageSystemModel {
  __typename?: 'StorageSystemModel'
  modelId: string
  model?: Model | null
  configuration: any
}
export interface IAddMetadataSupplementary_ContentMutation {
  __typename?: string | null
  metadata: AddMetadataSupplementary_MetadataMutation
}
export interface IActivity {
  __typename?: string | null
  id: string
  inputs: ActivityParameter[]
  outputs: ActivityParameter[]
  name: string
  childWorkflowId?: string | null
  description: string
  configuration: any
}
export interface IPrincipal {
  __typename?: string | null
  id: string
}
export enum PermissionAction {
  VIEW = 'VIEW',
  EDIT = 'EDIT',
  DELETE = 'DELETE',
  MANAGE = 'MANAGE',
  LIST = 'LIST',
}
export interface ITrait {
  __typename?: string | null
  id: string
  workflowIds: string[]
}
export interface IWorkflowStatesMutation {
  __typename?: string | null
  add?: WorkflowState | null
}
export interface IWorkflowActivityPromptInput {
  promptId: string
  configuration: any
}
export interface ISecurity {
  __typename?: string | null
}
export interface ILogin_Security {
  __typename?: string | null
  login: Login_Login
}
export interface IFindMetadata_MetadataContent {
  __typename?: string | null
  type: string
  length?: number | null
}
export interface MetadataMutation extends IMetadataMutation {
  __typename?: 'MetadataMutation'
  add: Metadata
  edit: Metadata
  addBulk: Metadata[]
  delete: boolean
  deleteContent: boolean
  addSearchDocuments: boolean
  addCategory: boolean
  deleteCategory: boolean
  addTrait: WorkflowExecutionPlan[]
  deleteTrait?: WorkflowExecutionPlan | null
  setPublic: Metadata
  setPublicContent: Metadata
  setPublicSupplementary: Metadata
  addPermission: Permission
  deletePermission: Permission
  addSupplementary: MetadataSupplementary
  deleteSupplementary: boolean
  setSupplementaryUploaded: boolean
  addRelationship: MetadataRelationship
  editRelationship: boolean
  deleteRelationship: boolean
  setWorkflowState: boolean
  setWorkflowStateComplete: boolean
  setMetadataAttributes: boolean
  setMetadataSystemAttributes: boolean
  setMetadataContents: boolean
  setMetadataUploaded: boolean
  setMetadataReady: boolean
}
export interface IWorkflow {
  __typename?: string | null
  queue: string
  configuration: any
  id: string
  name: string
  description: string
}
export interface IBeginTransitionInput {
  status: string
  version?: number | null
  metadataId?: string | null
  stateId: string
  supplementaryId?: string | null
  collectionId?: string | null
  waitForCompletion?: boolean | null
}
export interface ICollectionWorkflowCompleteState {
  collectionId: string
  status: string
}
export interface IAddChildCollection_ContentMutation {
  __typename?: string | null
  collection: AddChildCollection_CollectionMutation
}
export interface ISupplementaryDownloadUrl_MetadataSupplementary {
  __typename?: string | null
  key: string
  content: SupplementaryDownloadUrl_MetadataSupplementaryContent
  metadataId: string
}
export interface IGetCollectionInput {
}
export interface SourceById_Query extends IQuery, ISourceById_Query {
  __typename?: 'Query'
  content: SourceById_Content
}
export interface IMetadata {
  __typename?: string | null
}
export interface MetadataDownloadUrl_SignedUrlHeader extends ISignedUrlHeader, IMetadataDownloadUrl_SignedUrlHeader {
  __typename?: 'SignedUrlHeader'
  name: string
  value: string
}
export interface IPlanInput {
}
export interface IGetCollection_Collection {
  __typename?: string | null
  attributes: any
  modified: Date
  created: Date
  labels: string[]
  name: string
  id: string
}
export interface SetCollectionReady_CollectionMutation extends ICollectionMutation, ISetCollectionReady_CollectionMutation {
  __typename?: 'CollectionMutation'
  setReady: boolean
}
export interface ISetCollectionWorkflowStateComplete_ContentMutation {
  __typename?: string | null
  collection: SetCollectionWorkflowStateComplete_CollectionMutation
}
export interface IEnqueueChildWorkflowsInput {
}
export interface ISupplementaryUploadUrl_MetadataSupplementary {
  __typename?: string | null
  key: string
  metadataId: string
  content: SupplementaryUploadUrl_MetadataSupplementaryContent
}
export interface Query extends IQuery {
  __typename?: 'Query'
  content: Content
  workflows: Workflows
  security: Security
  queues: Queues
}
export interface Prompt extends IPrompt {
  __typename?: 'Prompt'
  id: string
  name: string
  description: string
  systemPrompt: string
  userPrompt: string
  inputType: string
  outputType: string
}
export interface Token extends IToken {
  __typename?: 'Token'
  token: string
}
export interface MetadataWorkflowCompleteState extends IMetadataWorkflowCompleteState {
  metadataId: string
  status: string
}
export interface SupplementaryDownloadUrl_Query extends IQuery, ISupplementaryDownloadUrl_Query {
  __typename?: 'Query'
  content: SupplementaryDownloadUrl_Content
}
export interface MetadataUploadUrl_SignedUrl extends ISignedUrl, IMetadataUploadUrl_SignedUrl {
  __typename?: 'SignedUrl'
  url: string
  headers: MetadataUploadUrl_SignedUrlHeader[]
}
export interface IFindMetadataInput {
}
export interface IMetadataUploadUrl_SignedUrl {
  __typename?: string | null
  headers: MetadataUploadUrl_SignedUrlHeader[]
  url: string
}
export interface IStorageSystem {
  __typename?: string | null
  id: string
  name: string
  models: StorageSystemModel[]
  description: string
  type: StorageSystemType
  configuration: any
}
export interface IWorkflowJob {
  __typename?: string | null
  metadata?: Metadata | null
  workflow: Workflow
  id: WorkflowJobId
  models: WorkflowActivityModel[]
  collection?: Collection | null
  failedChildren: WorkflowExecutionId[]
  version?: number | null
  prompts: WorkflowActivityPrompt[]
  storageSystems: WorkflowActivityStorageSystem[]
  completedChildren: WorkflowExecutionId[]
  children: WorkflowExecutionId[]
  collectionId?: string | null
  context: any
  workflowActivity: WorkflowActivity
  error?: string | null
  activity: Activity
  supplementaryId?: string | null
}
export interface SupplementaryDownloadUrl_MetadataSupplementaryContentUrls extends IMetadataSupplementaryContentUrls, ISupplementaryDownloadUrl_MetadataSupplementaryContentUrls {
  __typename?: 'MetadataSupplementaryContentUrls'
  download: SupplementaryDownloadUrl_SignedUrl
}
export interface IPermissionInput {
  action: PermissionAction
  entityId: string
  groupId: string
}
export interface IMetadataSupplementaryInput {
  attributes?: any | null
  name: string
  contentLength?: number | null
  key: string
  contentType: string
  metadataId: string
  sourceIdentifier?: string | null
  sourceId?: string | null
}
export interface IGetMetadata_MetadataSource {
  __typename?: string | null
  identifier?: string | null
  id?: string | null
}
export interface IWorkflowStates {
  __typename?: string | null
  state?: WorkflowState | null
  all: WorkflowState[]
}
export interface SetMetadataSystemAttributes_MetadataMutation extends IMetadataMutation, ISetMetadataSystemAttributes_MetadataMutation {
  __typename?: 'MetadataMutation'
  setMetadataSystemAttributes: boolean
}
export interface ISetMetadataAttributes_Mutation {
  __typename?: string | null
  content: SetMetadataAttributes_ContentMutation
}
export interface MetadataDownloadUrl_MetadataContentUrls extends IMetadataContentUrls, IMetadataDownloadUrl_MetadataContentUrls {
  __typename?: 'MetadataContentUrls'
  download: MetadataDownloadUrl_SignedUrl
}
export interface AddChildMetadataInput extends IAddChildMetadataInput, InputObject {
  id: string
  metadataId: string
}
export interface ISourceById_Query {
  __typename?: string | null
  content: SourceById_Content
}
export interface ISetCollectionWorkflowStateInput {
}
export interface ICollectionWorkflowInput {
  state: string
  deleteWorkflowId?: string | null
}
export interface ISupplementaryDownloadUrl_Content {
  __typename?: string | null
  metadataSupplementary?: SupplementaryDownloadUrl_MetadataSupplementary | null
}
export interface SetMetadataAttributes_ContentMutation extends IContentMutation, ISetMetadataAttributes_ContentMutation {
  __typename?: 'ContentMutation'
  metadata: SetMetadataAttributes_MetadataMutation
}
export interface IGetCollection_Content {
  __typename?: string | null
  collection?: GetCollection_Collection | null
}
export interface WorkflowActivityPromptInput extends IWorkflowActivityPromptInput {
  promptId: string
  configuration: any
}
export interface IMetadataUploadUrl_SignedUrlHeader {
  __typename?: string | null
  value: string
  name: string
}
export interface IEnqueueChildWorkflows_Mutation {
  __typename?: string | null
  workflows: EnqueueChildWorkflows_WorkflowsMutation
}
export interface MetadataDownloadUrl_MetadataContent extends IMetadataContent, IMetadataDownloadUrl_MetadataContent {
  __typename?: 'MetadataContent'
  type: string
  urls: MetadataDownloadUrl_MetadataContentUrls
}
export interface FindAttributeInput extends IFindAttributeInput {
  key: string
  value: string
}
export interface SetWorkflowState_MetadataMutation extends IMetadataMutation, ISetWorkflowState_MetadataMutation {
  __typename?: 'MetadataMutation'
  setWorkflowState: boolean
}
export interface SourceByIdInput extends ISourceByIdInput, InputObject {
  id: string
}
export interface InputObject extends IInputObject {
}
export interface IPromptInput {
  userPrompt: string
  name: string
  outputType: string
  inputType: string
  systemPrompt: string
  description: string
}
export interface MetadataDownloadUrl_Content extends IContent, IMetadataDownloadUrl_Content {
  __typename?: 'Content'
  metadata?: MetadataDownloadUrl_Metadata | null
}
export interface ISetMetadataPublic_MetadataMutation {
  __typename?: string | null
  setPublic: SetMetadataPublic_Metadata
}
export interface ITraitByIdInput {
}
export interface SetCollectionWorkflowState_ContentMutation extends IContentMutation, ISetCollectionWorkflowState_ContentMutation {
  __typename?: 'ContentMutation'
  collection: SetCollectionWorkflowState_CollectionMutation
}
export interface WorkflowExecutionId extends IWorkflowExecutionId {
  __typename?: 'WorkflowExecutionId'
  queue: string
  id: number
}
export interface CollectionChildInput extends ICollectionChildInput {
  collection: CollectionInput
  attributes?: any | null
}
export interface IGroup {
  __typename?: string | null
  id: string
  name: string
}
export interface WorkflowConfigurationInput extends IWorkflowConfigurationInput {
  activityId: string
  configuration: any
}
export interface ISetCollectionReady_CollectionMutation {
  __typename?: string | null
  setReady: boolean
}
export interface IFindMetadata_Query {
  __typename?: string | null
  content: FindMetadata_Content
}
export interface IWorkflowExecution {
  __typename?: string | null
}
export interface QueuesMutation extends IQueuesMutation {
  __typename?: 'QueuesMutation'
  retry?: Message | null
}
export interface SearchDocumentInput extends ISearchDocumentInput {
  metadataId?: string | null
  collectionId?: string | null
  content: string
}
export interface IEnqueueJob_Mutation {
  __typename?: string | null
  workflows: EnqueueJob_WorkflowsMutation
}
export interface ActivityParameterInput extends IActivityParameterInput {
  name: string
  type: ActivityParameterType
}
export interface ISetMetadataReady_ContentMutation {
  __typename?: string | null
  metadata: SetMetadataReady_MetadataMutation
}
export interface WorkflowStatesMutation extends IWorkflowStatesMutation {
  __typename?: 'WorkflowStatesMutation'
  add?: WorkflowState | null
}
export interface ILogin_Query {
  __typename?: string | null
  security: Login_Security
}
export interface SetWorkflowStateComplete_MetadataMutation extends IMetadataMutation, ISetWorkflowStateComplete_MetadataMutation {
  __typename?: 'MetadataMutation'
  setWorkflowStateComplete: boolean
}
export interface ISupplementaryUploadUrl_MetadataSupplementaryContent {
  __typename?: string | null
  type: string
  urls: SupplementaryUploadUrl_MetadataSupplementaryContentUrls
}
export interface IMessageQueueStats {
  __typename?: string | null
  available: number
  pending: number
  max?: Date | null
  min?: Date | null
  size: number
}
export interface SetCollectionReady_Mutation extends IMutation, ISetCollectionReady_Mutation {
  __typename?: 'Mutation'
  content: SetCollectionReady_ContentMutation
}
export interface IAddMetadata_Metadata {
  __typename?: string | null
  id: string
}
export interface AddSearchDocuments_MetadataMutation extends IMetadataMutation, IAddSearchDocuments_MetadataMutation {
  __typename?: 'MetadataMutation'
  addSearchDocuments: boolean
}
export interface IMetadataDownloadUrl_Query {
  __typename?: string | null
  content: MetadataDownloadUrl_Content
}
export interface SupplementaryUploadUrl_Query extends IQuery, ISupplementaryUploadUrl_Query {
  __typename?: 'Query'
  content: SupplementaryUploadUrl_Content
}
export interface StorageSystem extends IStorageSystem {
  __typename?: 'StorageSystem'
  id: string
  type: StorageSystemType
  name: string
  description: string
  configuration: any
  models: StorageSystemModel[]
}
export interface WorkflowJobIdInput extends IWorkflowJobIdInput {
  queue: string
  id: number
  index: number
}
export interface AddCollection_Collection extends ICollection, IAddCollection_Collection {
  __typename?: 'Collection'
  id: string
}
export interface ISetWorkflowJobComplete_Mutation {
  __typename?: string | null
  workflows: SetWorkflowJobComplete_WorkflowsMutation
}
export interface GetMetadata_Query extends IQuery, IGetMetadata_Query {
  __typename?: 'Query'
  content: GetMetadata_Content
}
export interface FindCollection_Query extends IQuery, IFindCollection_Query {
  __typename?: 'Query'
  content: FindCollection_Content
}
export interface SignedUrl extends ISignedUrl {
  __typename?: 'SignedUrl'
  url: string
  headers: SignedUrlHeader[]
}
export interface ISearchQuery {
  offset?: number | null
  storageSystemId: string
  filter?: string | null
  query: string
  limit?: number | null
}
export interface IAddCollection_Mutation {
  __typename?: string | null
  content: AddCollection_ContentMutation
}
export interface ModelsMutation extends IModelsMutation {
  __typename?: 'ModelsMutation'
  add?: Model | null
}
export interface Trait extends ITrait {
  __typename?: 'Trait'
  id: string
  name: string
  description: string
  workflowIds: string[]
  workflows: Workflow[]
}
export interface Group extends IGroup {
  __typename?: 'Group'
  id: string
  name: string
}
export interface IQueues {
  __typename?: string | null
  getMessages: Message[]
  messageQueues: MessageQueue[]
  getMessage?: Message | null
}
export interface ISearchDocumentInput {
  metadataId?: string | null
  content: string
  collectionId?: string | null
}
export interface SetCollectionPublic_ContentMutation extends IContentMutation, ISetCollectionPublic_ContentMutation {
  __typename?: 'ContentMutation'
  collection: SetCollectionPublic_CollectionMutation
}
export interface Login_Query extends IQuery, ILogin_Query {
  __typename?: 'Query'
  security: Login_Security
}
export interface IMetadataDownloadUrl_MetadataContentUrls {
  __typename?: string | null
  download: MetadataDownloadUrl_SignedUrl
}
export interface SetWorkflowStateComplete_ContentMutation extends IContentMutation, ISetWorkflowStateComplete_ContentMutation {
  __typename?: 'ContentMutation'
  metadata: SetWorkflowStateComplete_MetadataMutation
}
export interface FindMetadataInput extends IFindMetadataInput, InputObject {
  attributes: FindAttributeInput
}
export interface IAddSearchDocumentsInput {
}
export interface IMetadataSupplementaryContentUrls {
  __typename?: string | null
}
export interface ISetWorkflowState_Mutation {
  __typename?: string | null
  content: SetWorkflowState_ContentMutation
}
export interface ISupplementaryUploadUrl_Content {
  __typename?: string | null
  metadataSupplementary?: SupplementaryUploadUrl_MetadataSupplementary | null
}
export interface IMetadataWorkflowCompleteState {
  status: string
  metadataId: string
}
export interface IMetadataUploadUrlInput {
}
export interface IFindCollectionInput {
}
export interface IModels {
  __typename?: string | null
  all: Model[]
  model?: Model | null
}
export interface StorageSystems extends IStorageSystems {
  __typename?: 'StorageSystems'
  all: StorageSystem[]
  storageSystem?: StorageSystem | null
}
export interface Principal extends IPrincipal {
  __typename?: 'Principal'
  id: string
  groups: Group[]
}
export interface ISetCollectionWorkflowState_CollectionMutation {
  __typename?: string | null
  setWorkflowState: boolean
}
export interface EnqueueChildWorkflowInput extends IEnqueueChildWorkflowInput, InputObject {
  jobId: WorkflowExecutionIdInput
  workflowId: string
  configurations: WorkflowConfigurationInput
}
export interface ISetExecutionPlanJobCheckin_WorkflowsMutation {
  __typename?: string | null
  setExecutionPlanJobCheckin: boolean
}
export interface AddSearchDocuments_ContentMutation extends IContentMutation, IAddSearchDocuments_ContentMutation {
  __typename?: 'ContentMutation'
  metadata: AddSearchDocuments_MetadataMutation
}
export interface GetMetadata_MetadataSupplementaryContent extends IMetadataSupplementaryContent, IGetMetadata_MetadataSupplementaryContent {
  __typename?: 'MetadataSupplementaryContent'
  type: string
  length?: number | null
}
export interface IWorkflowJobId {
  __typename?: string | null
  queue: string
  id: number
  index: number
}
export interface ISetMetadataReadyInput {
}
export interface IAddChildMetadataInput {
}
export interface IMetadataType {
  __typename?: string | null
}
export interface IGetMetadata_Query {
  __typename?: string | null
  content: GetMetadata_Content
}
export interface IMetadataContentUrls {
  __typename?: string | null
}
export interface Collection extends ICollection, CollectionItem {
  __typename?: 'Collection'
  id: string
  type: CollectionType
  name: string
  description?: string | null
  traitIds: string[]
  labels: string[]
  attributes: any
  itemAttributes?: any | null
  systemAttributes?: any | null
  ordering?: any | null
  created: Date
  modified: Date
  parentCollections: Collection[]
  items: CollectionItem[]
  collections: Collection[]
  metadata: Metadata[]
  workflow: CollectionWorkflow
  ready?: Date | null
  public: boolean
  publicList: boolean
  permissions: Permission[]
}
export interface ICollectionChildInput {
  attributes?: any | null
  collection: CollectionInput
}
export interface IWorkflowActivityModel {
  __typename?: string | null
  configuration: any
  model: Model
}
export interface IMetadataSupplementarySource {
  __typename?: string | null
  id: string
  identifier?: string | null
}
export interface IAddSearchDocuments_MetadataMutation {
  __typename?: string | null
  addSearchDocuments: boolean
}
export interface EnqueueChildWorkflows_WorkflowExecutionId extends IWorkflowExecutionId, IEnqueueChildWorkflows_WorkflowExecutionId {
  __typename?: 'WorkflowExecutionId'
  queue: string
  id: number
}
export interface AddMetadataBulk_Mutation extends IMutation, IAddMetadataBulk_Mutation {
  __typename?: 'Mutation'
  content: AddMetadataBulk_ContentMutation
}
export interface ISearchDocument {
  __typename?: string | null
  content: string
  metadata?: Metadata | null
  collection?: Collection | null
}
export interface ITransition {
  __typename?: string | null
  description: string
  toStateId: string
  fromStateId: string
}
export interface MetadataUploadUrl_MetadataContentUrls extends IMetadataContentUrls, IMetadataUploadUrl_MetadataContentUrls {
  __typename?: 'MetadataContentUrls'
  upload: MetadataUploadUrl_SignedUrl
}
export interface IAddSearchDocuments_ContentMutation {
  __typename?: string | null
  metadata: AddSearchDocuments_MetadataMutation
}
export interface ISearchResultObject {
  __typename?: string | null
  estimatedHits: number
  documents: SearchDocument[]
}
export interface MetadataDownloadUrl_Query extends IQuery, IMetadataDownloadUrl_Query {
  __typename?: 'Query'
  content: MetadataDownloadUrl_Content
}
export interface Models extends IModels {
  __typename?: 'Models'
  all: Model[]
  model?: Model | null
}
export interface IActivityParameterInput {
  name: string
  type: ActivityParameterType
}
export interface ILoginInput {
}
export interface IWorkflowActivityStorageSystem {
  __typename?: string | null
  configuration: any
  system: StorageSystem
}
export enum CollectionType {
  ROOT = 'ROOT',
  STANDARD = 'STANDARD',
  FOLDER = 'FOLDER',
  QUEUE = 'QUEUE',
}
export interface IMetadataWorkflowState {
  stateId: string
  immediate: boolean
  metadataId: string
  status: string
}
export interface IPromptsMutation {
  __typename?: string | null
  edit?: Prompt | null
  delete: boolean
  add?: Prompt | null
}
export interface IAddCollection_Collection {
  __typename?: string | null
  id: string
}
export interface IFindMetadata_MetadataSupplementarySource {
  __typename?: string | null
  identifier?: string | null
  id: string
}
export interface SupplementaryDownloadUrl_SignedUrl extends ISignedUrl, ISupplementaryDownloadUrl_SignedUrl {
  __typename?: 'SignedUrl'
  url: string
  headers: SupplementaryDownloadUrl_SignedUrlHeader[]
}
export interface MetadataWorkflow extends IMetadataWorkflow {
  __typename?: 'MetadataWorkflow'
  state: string
  pending?: string | null
  deleteWorkflow?: string | null
  plans: WorkflowExecutionPlan[]
}
export interface MetadataRelationship extends IMetadataRelationship {
  __typename?: 'MetadataRelationship'
  id: string
  metadata: Metadata
  relationship: string
  attributes: any
}
export interface AddMetadataSupplementaryInput extends IAddMetadataSupplementaryInput, InputObject {
  supplementary: MetadataSupplementaryInput
}
export interface MetadataSupplementarySource extends IMetadataSupplementarySource {
  __typename?: 'MetadataSupplementarySource'
  id: string
  identifier?: string | null
}
export interface IWorkflowActivityInput {
  prompts: WorkflowActivityPromptInput
  storageSystems: WorkflowActivityStorageSystemInput
  activityId: string
  configuration: any
  description: string
  outputs: WorkflowActivityParameterInput
  queue: string
  inputs: WorkflowActivityParameterInput
  models: WorkflowActivityModelInput
  executionGroup: number
}
export interface GetCollectionItems_Collection extends ICollection, IGetCollectionItems_Collection {
  __typename?: 'Collection'
  items: GetCollectionItems_CollectionItem[]
}
export interface IMetadataDownloadUrl_Content {
  __typename?: string | null
  metadata?: MetadataDownloadUrl_Metadata | null
}
export interface WorkflowJobId extends IWorkflowJobId {
  __typename?: 'WorkflowJobId'
  queue: string
  id: number
  index: number
}
export interface GetCollectionItems_Query extends IQuery, IGetCollectionItems_Query {
  __typename?: 'Query'
  content: GetCollectionItems_Content
}
export interface ISetWorkflowStateInput {
}
export interface ISupplementaryUploadUrl_SignedUrlHeader {
  __typename?: string | null
  name: string
  value: string
}
export interface MetadataWorkflowState extends IMetadataWorkflowState {
  metadataId: string
  stateId: string
  status: string
  immediate: boolean
}
export interface SetCollectionWorkflowState_Mutation extends IMutation, ISetCollectionWorkflowState_Mutation {
  __typename?: 'Mutation'
  content: SetCollectionWorkflowState_ContentMutation
}
export interface ISetWorkflowStateCompleteInput {
}
export interface ISetMetadataSystemAttributes_ContentMutation {
  __typename?: string | null
  metadata: SetMetadataSystemAttributes_MetadataMutation
}
export interface IStorageSystemModel {
  __typename?: string | null
  model?: Model | null
  modelId: string
  configuration: any
}
export interface IWorkflowStateType {
  __typename?: string | null
}
export enum MetadataType {
  STANDARD = 'STANDARD',
  VARIANT = 'VARIANT',
}
export interface SignedUrlHeader extends ISignedUrlHeader {
  __typename?: 'SignedUrlHeader'
  name: string
  value: string
}
export interface IAddMetadataBulk_ContentMutation {
  __typename?: string | null
  metadata: AddMetadataBulk_MetadataMutation
}
export interface ISetWorkflowJobContext_WorkflowsMutation {
  __typename?: string | null
  setExecutionJobContext: boolean
}
export interface GetMetadata_MetadataContent extends IMetadataContent, IGetMetadata_MetadataContent {
  __typename?: 'MetadataContent'
  type: string
  length?: number | null
}
export interface AddMetadataBulk_Metadata extends IMetadata, IAddMetadataBulk_Metadata {
  __typename?: 'Metadata'
  id: string
}
export interface MetadataSource extends IMetadataSource {
  __typename?: 'MetadataSource'
  id?: string | null
  identifier?: string | null
}
export interface BeginTransitionInput extends IBeginTransitionInput {
  collectionId?: string | null
  metadataId?: string | null
  version?: number | null
  stateId: string
  status: string
  supplementaryId?: string | null
  waitForCompletion?: boolean | null
}
export interface CollectionMutation extends ICollectionMutation {
  __typename?: 'CollectionMutation'
  add: Collection
  addBulk: Collection[]
  edit: Collection
  delete: boolean
  setPublic: Collection
  setPublicList: Collection
  addPermission: Permission
  deletePermission: Permission
  setChildItemAttributes: Collection
  addChildCollection: Collection
  removeChildCollection: Collection
  addChildMetadata: Collection
  removeChildMetadata: Collection
  setWorkflowState: boolean
  setWorkflowStateComplete: boolean
  setCollectionAttributes: boolean
  setCollectionOrdering: boolean
  setReady: boolean
}
export interface ILogin_Login {
  __typename?: string | null
  password: Login_LoginResponse
}
export interface EnqueueJob_WorkflowsMutation extends IWorkflowsMutation, IEnqueueJob_WorkflowsMutation {
  __typename?: 'WorkflowsMutation'
  enqueueJob?: EnqueueJob_WorkflowExecutionId | null
}
export interface ISetCollectionWorkflowStateComplete_Mutation {
  __typename?: string | null
  content: SetCollectionWorkflowStateComplete_ContentMutation
}
export interface SetCollectionWorkflowState_CollectionMutation extends ICollectionMutation, ISetCollectionWorkflowState_CollectionMutation {
  __typename?: 'CollectionMutation'
  setWorkflowState: boolean
}
export interface SetCollectionReadyInput extends ISetCollectionReadyInput, InputObject {
  id: string
}
export interface SetMetadataReadyInput extends ISetMetadataReadyInput, InputObject {
  id: string
}
export interface ICollectionMutation {
  __typename?: string | null
}
export interface Plan_WorkflowExecution extends IWorkflowExecution, IPlan_WorkflowExecution {
  __typename?: 'WorkflowExecution'
}
export interface ActivityInput extends IActivityInput {
  id: string
  name: string
  description: string
  childWorkflowId?: string | null
  configuration: any
  inputs: ActivityParameterInput
  outputs: ActivityParameterInput
}
export interface SetWorkflowJobComplete_WorkflowsMutation extends IWorkflowsMutation, ISetWorkflowJobComplete_WorkflowsMutation {
  __typename?: 'WorkflowsMutation'
  setExecutionPlanJobComplete: boolean
}
export interface SetExecutionPlanJobCheckinInput extends ISetExecutionPlanJobCheckinInput, InputObject {
  jobId: WorkflowJobIdInput
}
export interface IMetadataMutation {
  __typename?: string | null
}
export interface IAddChildCollection_Collection {
  __typename?: string | null
  id: string
}
export interface AddMetadataSupplementary_ContentMutation extends IContentMutation, IAddMetadataSupplementary_ContentMutation {
  __typename?: 'ContentMutation'
  metadata: AddMetadataSupplementary_MetadataMutation
}
export interface IAddCollectionInput {
}
export interface AddMetadata_MetadataMutation extends IMetadataMutation, IAddMetadata_MetadataMutation {
  __typename?: 'MetadataMutation'
  add: AddMetadata_Metadata
}
export interface SupplementaryUploadUrl_SignedUrlHeader extends ISignedUrlHeader, ISupplementaryUploadUrl_SignedUrlHeader {
  __typename?: 'SignedUrlHeader'
  name: string
  value: string
}
export interface SetMetadataPublic_Metadata extends IMetadata, ISetMetadataPublic_Metadata {
  __typename?: 'Metadata'
  id: string
}
export interface MetadataSupplementaryInput extends IMetadataSupplementaryInput {
  metadataId: string
  key: string
  name: string
  contentType: string
  contentLength?: number | null
  sourceId?: string | null
  sourceIdentifier?: string | null
  attributes?: any | null
}
export interface IFindAttributeInput {
  value: string
  key: string
}
export interface Activity extends IActivity {
  __typename?: 'Activity'
  id: string
  name: string
  description: string
  childWorkflowId?: string | null
  configuration: any
  inputs: ActivityParameter[]
  outputs: ActivityParameter[]
}
export interface IMutation {
  __typename?: string | null
}
export interface IWorkflowInput {
  queue: string
  name: string
  id: string
  description: string
  configuration: any
  activities: WorkflowActivityInput
}
export interface IQueuesMutation {
  __typename?: string | null
  retry?: Message | null
}
export interface SupplementaryUploadUrl_MetadataSupplementary extends IMetadataSupplementary, ISupplementaryUploadUrl_MetadataSupplementary {
  __typename?: 'MetadataSupplementary'
  metadataId: string
  key: string
  content: SupplementaryUploadUrl_MetadataSupplementaryContent
}
export interface GetCollectionItems_Content extends IContent, IGetCollectionItems_Content {
  __typename?: 'Content'
  collection?: GetCollectionItems_Collection | null
}
export interface MessageQueueStats extends IMessageQueueStats {
  __typename?: 'MessageQueueStats'
  size: number
  pending: number
  available: number
  min?: Date | null
  max?: Date | null
}
export interface GetCollection_Content extends IContent, IGetCollection_Content {
  __typename?: 'Content'
  collection?: GetCollection_Collection | null
}
export interface IMetadataSupplementaryContent {
  __typename?: string | null
}
export interface EnqueueChildWorkflow_WorkflowExecutionId extends IWorkflowExecutionId, IEnqueueChildWorkflow_WorkflowExecutionId {
  __typename?: 'WorkflowExecutionId'
  queue: string
  id: number
}
export interface IWorkflowJobIdInput {
  index: number
  id: number
  queue: string
}
export interface IActivityInput {
  description: string
  id: string
  outputs: ActivityParameterInput
  inputs: ActivityParameterInput
  childWorkflowId?: string | null
  configuration: any
  name: string
}
export interface WorkflowJob extends WorkflowExecution, IWorkflowJob {
  __typename?: 'WorkflowJob'
  id: WorkflowJobId
  workflow: Workflow
  error?: string | null
  collectionId?: string | null
  collection?: Collection | null
  metadata?: Metadata | null
  version?: number | null
  supplementaryId?: string | null
  activity: Activity
  children: WorkflowExecutionId[]
  completedChildren: WorkflowExecutionId[]
  failedChildren: WorkflowExecutionId[]
  workflowActivity: WorkflowActivity
  prompts: WorkflowActivityPrompt[]
  storageSystems: WorkflowActivityStorageSystem[]
  models: WorkflowActivityModel[]
  context: any
}
export interface WorkflowExecutionIdInput extends IWorkflowExecutionIdInput {
  queue: string
  id: number
}
export interface IModelInput {
  type: string
  configuration: any
  name: string
  description: string
}
export interface IWorkflowActivityModelInput {
  configuration: any
  modelId: string
}
export interface IAddMetadataBulk_Metadata {
  __typename?: string | null
  id: string
}
export interface SupplementaryDownloadUrl_SignedUrlHeader extends ISignedUrlHeader, ISupplementaryDownloadUrl_SignedUrlHeader {
  __typename?: 'SignedUrlHeader'
  name: string
  value: string
}
export interface ITraitById_Trait {
  __typename?: string | null
  workflowIds: string[]
  id: string
}
export interface MetadataWorkflowInput extends IMetadataWorkflowInput {
  state: string
  deleteWorkflowId?: string | null
}
export interface IAddMetadata_ContentMutation {
  __typename?: string | null
  metadata: AddMetadata_MetadataMutation
}
export interface ISetCollectionPublic_CollectionMutation {
  __typename?: string | null
  setPublic: SetCollectionPublic_Collection
}
export interface ISetWorkflowJobContext_Mutation {
  __typename?: string | null
  workflows: SetWorkflowJobContext_WorkflowsMutation
}
export interface IEnqueueChildWorkflow_WorkflowsMutation {
  __typename?: string | null
  enqueueChildWorkflow: EnqueueChildWorkflow_WorkflowExecutionId
}
export interface IMetadataSupplementary {
  __typename?: string | null
  key: string
}
export interface Plan_Query extends IQuery, IPlan_Query {
  __typename?: 'Query'
  workflows: Plan_Workflows
}
export interface IActivitiesMutation {
  __typename?: string | null
  delete: boolean
  add?: Activity | null
  edit?: Activity | null
}
export interface IPermission {
  __typename?: string | null
  group: Group
  groupId: string
  action: PermissionAction
}
export interface AddMetadataSupplementary_MetadataSupplementary extends IMetadataSupplementary, IAddMetadataSupplementary_MetadataSupplementary {
  __typename?: 'MetadataSupplementary'
  key: string
}
export interface IPlan_WorkflowExecution {
  __typename?: string | null
}
export interface ISetCollectionPublicList_ContentMutation {
  __typename?: string | null
  collection: SetCollectionPublicList_CollectionMutation
}
export interface ISource {
  __typename?: string | null
  id: string
  name: string
}
export interface IMetadataDownloadUrl_SignedUrlHeader {
  __typename?: string | null
  name: string
  value: string
}
export interface IAddCollection_CollectionMutation {
  __typename?: string | null
  add: AddCollection_Collection
}
export interface IEnqueueJobInput {
}
export interface ISetWorkflowJobFailed_Mutation {
  __typename?: string | null
  workflows: SetWorkflowJobFailed_WorkflowsMutation
}
export interface IAddMetadataSupplementaryInput {
}
export interface LoginResponse extends ILoginResponse {
  __typename?: 'LoginResponse'
  principal: Principal
  token: Token
}
export interface IPlan_Workflows {
  __typename?: string | null
  nextWorkflowExecution?: Plan_WorkflowExecution | null
}
export interface IQuery {
  __typename?: string | null
}
export interface IEnqueueChildWorkflows_WorkflowExecutionId {
  __typename?: string | null
  queue: string
  id: number
}
export interface ISetMetadataSystemAttributesInput {
}
export interface EnqueueChildWorkflows_WorkflowsMutation extends IWorkflowsMutation, IEnqueueChildWorkflows_WorkflowsMutation {
  __typename?: 'WorkflowsMutation'
  enqueueChildWorkflows: EnqueueChildWorkflows_WorkflowExecutionId[]
}
export interface ISetMetadataAttributes_ContentMutation {
  __typename?: string | null
  metadata: SetMetadataAttributes_MetadataMutation
}
export interface SetMetadataPublic_ContentMutation extends IContentMutation, ISetMetadataPublic_ContentMutation {
  __typename?: 'ContentMutation'
  metadata: SetMetadataPublic_MetadataMutation
}
export interface Plan_Workflows extends IWorkflows, IPlan_Workflows {
  __typename?: 'Workflows'
  nextWorkflowExecution?: Plan_WorkflowExecution | null
}
export interface SupplementaryUploadUrl_SignedUrl extends ISignedUrl, ISupplementaryUploadUrl_SignedUrl {
  __typename?: 'SignedUrl'
  url: string
  headers: SupplementaryUploadUrl_SignedUrlHeader[]
}
export interface SetWorkflowStateComplete_Mutation extends IMutation, ISetWorkflowStateComplete_Mutation {
  __typename?: 'Mutation'
  content: SetWorkflowStateComplete_ContentMutation
}
export interface IMessage {
  __typename?: string | null
  value: any
  visibleTimeout: Date
  id: number
}
export interface IAddChildCollection_CollectionMutation {
  __typename?: string | null
  addChildCollection: AddChildCollection_Collection
}
export interface IStorageSystems {
  __typename?: string | null
  all: StorageSystem[]
  storageSystem?: StorageSystem | null
}
export interface GetMetadataInput extends IGetMetadataInput, InputObject {
  id: string
}
export interface IWorkflowExecutionPlan {
  __typename?: string | null
  collectionId?: string | null
  jobs: WorkflowJob[]
  id: number
  version?: number | null
  complete: WorkflowJobId[]
  context: any
  workflow: Workflow
  parent?: WorkflowExecutionId | null
  supplementaryId?: string | null
  metadataId?: string | null
  error?: string | null
  next?: WorkflowJobId | null
  running: WorkflowJobId[]
  current: WorkflowJobId[]
  failed: WorkflowJobId[]
  metadata?: Metadata | null
  pending: WorkflowJobId[]
}
export interface FindMetadata_MetadataSource extends IMetadataSource, IFindMetadata_MetadataSource {
  __typename?: 'MetadataSource'
  id?: string | null
  identifier?: string | null
}
export interface MetadataUploadUrlInput extends IMetadataUploadUrlInput, InputObject {
  id: string
}
export interface IWorkflowStateInput {
  name: string
  configuration: any
  description: string
  workflowId?: string | null
  type: WorkflowStateType
  id: string
  exitWorkflowId?: string | null
  entryWorkflowId?: string | null
}
export interface IGetCollectionItems_Content {
  __typename?: string | null
  collection?: GetCollectionItems_Collection | null
}
export interface ISupplementaryUploadUrl_Query {
  __typename?: string | null
  content: SupplementaryUploadUrl_Content
}
export interface ISupplementaryDownloadUrlInput {
}
export interface SetWorkflowStateCompleteInput extends ISetWorkflowStateCompleteInput, InputObject {
  state: MetadataWorkflowCompleteState
}
export interface GetMetadata_MetadataSupplementarySource extends IMetadataSupplementarySource, IGetMetadata_MetadataSupplementarySource {
  __typename?: 'MetadataSupplementarySource'
  id: string
  identifier?: string | null
}
export interface TraitByIdInput extends ITraitByIdInput, InputObject {
  id: string
}
export interface WorkflowInput extends IWorkflowInput {
  id: string
  name: string
  description: string
  queue: string
  configuration: any
  activities: WorkflowActivityInput
}
export interface MetadataUploadUrl_Metadata extends IMetadata, IMetadataUploadUrl_Metadata {
  __typename?: 'Metadata'
  content: MetadataUploadUrl_MetadataContent
}
export interface Prompts extends IPrompts {
  __typename?: 'Prompts'
  all: Prompt[]
  prompt?: Prompt | null
}
export interface ISetWorkflowStateComplete_MetadataMutation {
  __typename?: string | null
  setWorkflowStateComplete: boolean
}
export interface SupplementaryDownloadUrl_Content extends IContent, ISupplementaryDownloadUrl_Content {
  __typename?: 'Content'
  metadataSupplementary?: SupplementaryDownloadUrl_MetadataSupplementary | null
}
