use async_graphql::InputObject;

#[derive(InputObject)]
pub struct CollectionWorkflowState {
    pub collection_id: String,
    pub state_id: String,
    pub status: String,
    pub immediate: bool,
}

#[derive(InputObject)]
pub struct CollectionWorkflowCompleteState {
    pub collection_id: String,
    pub status: String,
}
