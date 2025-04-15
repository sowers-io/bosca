use crate::context::BoscaContext;
use crate::graphql::mutation::MutationObject;
use crate::graphql::query::QueryObject;
use crate::graphql::subscription::SubscriptionObject;
use crate::logger::Logger;
use crate::queries::PersistedQueriesCache;
use crate::security::authorization_extension::Authorization;
use async_graphql::extensions::apollo_persisted_queries::ApolloPersistedQueries;
use async_graphql::extensions::Tracing;
use async_graphql::Schema;
use crate::caching_headers::CachingHeaders;

pub fn new_schema(
    ctx: BoscaContext,
    persisted_queries: ApolloPersistedQueries<PersistedQueriesCache>,
) -> Schema<QueryObject, MutationObject, SubscriptionObject> {
    Schema::build(QueryObject, MutationObject, SubscriptionObject)
        .data(ctx.clone())
        .extension(Tracing)
        .extension(Authorization)
        .extension(CachingHeaders)
        .extension(persisted_queries)
        .extension(Logger)
        .data(ctx.clone())
        .finish()
}
