use crate::models::content::item::ContentItem;
use async_graphql::extensions::{Extension, ExtensionContext, ExtensionFactory, NextPrepareRequest};
use async_graphql::{Context, Request, ServerResult};
use std::sync::atomic::AtomicI64;
use std::sync::Arc;

#[derive(Clone)]
pub struct CachingHeaderManager {
    last_modified: Arc<AtomicI64>,
}

impl CachingHeaderManager {
    pub fn new() -> CachingHeaderManager {
        CachingHeaderManager {
            last_modified: Arc::new(AtomicI64::new(0)),
        }
    }

    pub fn apply(&self, ctx: &Context<'_>, item: &impl ContentItem) {
        let last_modified = self
            .last_modified
            .load(std::sync::atomic::Ordering::Relaxed);
        let last_item_modified = item.modified().timestamp();
        if last_modified < last_item_modified {
            self.last_modified
                .store(last_item_modified, std::sync::atomic::Ordering::Relaxed);
            if let Some(etag) = item.etag() {
                ctx.insert_http_header(http::header::ETAG, etag.clone());
            }
            ctx.insert_http_header(http::header::LAST_MODIFIED, item.modified().to_rfc2822());
        }
    }

    pub fn get<'a>(ctx: &Context<'a>) -> async_graphql::Result<&'a CachingHeaderManager> {
        ctx.data::<CachingHeaderManager>()
    }
}

pub struct CachingHeaders;

impl ExtensionFactory for CachingHeaders {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(CachingHeadersExtension::default())
    }
}

#[derive(Default, Debug)]
struct CachingHeadersExtension {}

#[async_trait::async_trait]
impl Extension for CachingHeadersExtension {
    async fn prepare_request(
        &self,
        ctx: &ExtensionContext<'_>,
        request: Request,
        next: NextPrepareRequest<'_>,
    ) -> ServerResult<Request> {
        let mgr = CachingHeaderManager::new();
        next.run(ctx, request.data(mgr)).await
    }
}