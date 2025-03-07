use async_graphql::InputObject;

#[derive(InputObject)]
pub struct MetadataWorkflowState {
    pub metadata_id: String,
    pub state_id: String,
    pub status: String,
    pub immediate: bool,
}

#[derive(InputObject)]
pub struct MetadataWorkflowCompleteState {
    pub metadata_id: String,
    pub status: String,
}
