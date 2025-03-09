use crate::context::BoscaContext;
use crate::graphql::mutation::MutationObject;
use crate::graphql::query::QueryObject;
use crate::graphql::subscription::SubscriptionObject;
use crate::logger::Logger;
use crate::queries::PersistedQueriesCache;
use crate::security::authorization_extension::Authorization;
use async_graphql::extensions::apollo_persisted_queries::ApolloPersistedQueries;
use async_graphql::extensions::OpenTelemetry;
use async_graphql::Schema;
use opentelemetry_sdk::trace::Tracer;

pub fn new_schema(
    ctx: BoscaContext,
    telemetry: OpenTelemetry<Tracer>,
    persisted_queries: ApolloPersistedQueries<PersistedQueriesCache>,
) -> Schema<QueryObject, MutationObject, SubscriptionObject> {
    Schema::build(QueryObject, MutationObject, SubscriptionObject)
        .data(ctx.clone())
        .extension(Authorization)
        .extension(telemetry)
        .extension(persisted_queries)
        .extension(Logger)
        .data(ctx.clone())
        .finish()
}
