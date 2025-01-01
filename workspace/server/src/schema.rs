use async_graphql::Schema;
use crate::graphql::mutation::MutationObject;
use crate::graphql::query::QueryObject;
use crate::graphql::subscription::SubscriptionObject;

pub type BoscaSchema = Schema<QueryObject, MutationObject, SubscriptionObject>;