use crate::graphql::content::metadata_mutation::WorkflowConfigurationInput;
use crate::models::workflow::workflows::Workflow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Default, Clone)]
pub struct EnqueueRequest {
    pub trait_id: Option<String>,

    pub profile_id: Option<Uuid>,

    pub workflow_id: Option<String>,
    pub workflow: Option<Workflow>,

    pub metadata_id: Option<Uuid>,
    pub metadata_version: Option<i32>,

    pub comment_id: Option<i64>,

    pub collection_id: Option<Uuid>,

    pub storage_system_ids: Option<Vec<Uuid>>,

    pub configurations: Option<Vec<WorkflowConfigurationInput>>,
    pub delay_until: Option<DateTime<Utc>>,
    pub wait_for_completion: bool
}

// #[derive(InputObject)]
// pub struct EnqueueRequestInput {
//     pub workflow_id: String,
//
//     pub metadata_id: Option<String>,
//     pub metadata_version: Option<i32>,
//
//     pub collection_id: Option<String>,
//
//     pub storage_system_ids: Option<Vec<String>>,
//
//     pub configurations: Option<Vec<WorkflowConfigurationInput>>,
//     pub wait_for_completion: Option<bool>,
//     pub delay_until: Option<DateTime<Utc>>,
// }

// impl EnqueueRequestInput {
//     fn to_enqueue_request(self, workflow: Option<Workflow>) -> EnqueueRequest {
//         EnqueueRequest {
//             workflow,
//             workflow_id: None,
//             trait_id: None,
//             metadata_id: self
//                 .metadata_id
//                 .as_ref()
//                 .map(|s| Uuid::parse_str(s.as_str()).unwrap()),
//             metadata_version: self.metadata_version,
//             collection_id: self
//                 .collection_id
//                 .as_ref()
//                 .map(|s| Uuid::parse_str(s.as_str()).unwrap()),
//             storage_system_ids: self.storage_system_ids.as_ref().map(|ids| {
//                 ids.iter()
//                     .map(|id| Uuid::parse_str(id.as_str()).unwrap())
//                     .collect()
//             }),
//             configurations: self.configurations,
//             wait_for_completion: self.wait_for_completion,
//             delay_until: self.delay_until,
//         }
//     }
// }
