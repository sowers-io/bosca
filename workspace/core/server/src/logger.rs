use std::{fmt::Write, sync::Arc};

use async_graphql::{
    extensions::{Extension, ExtensionContext, ExtensionFactory, NextExecute, NextParseQuery},
    parser::types::{ExecutableDocument, OperationType, Selection},
    PathSegment, Response, ServerResult, Variables,
};

pub struct Logger;

impl ExtensionFactory for Logger {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(LoggerExtension)
    }
}

struct LoggerExtension;

#[async_trait::async_trait]
impl Extension for LoggerExtension {

    #[tracing::instrument(skip(self, ctx, query, variables, next))]
    async fn parse_query(
        &self,
        ctx: &ExtensionContext<'_>,
        query: &str,
        variables: &Variables,
        next: NextParseQuery<'_>,
    ) -> ServerResult<ExecutableDocument> {
        let document = next.run(ctx, query, variables).await?;
        if log::log_enabled!(log::Level::Debug) {
            let is_schema = document
                .operations
                .iter()
                .filter(|(_, operation)| operation.node.ty == OperationType::Query)
                .any(|(_, operation)| operation.node.selection_set.node.items.iter().any(|selection| matches!(&selection.node, Selection::Field(field) if field.node.name.node == "__schema")));
            if !is_schema {
                log::debug!(
                    target: "async-graphql",
                    "[Execute] {}", ctx.stringify_execute_doc(&document, variables)
                );
            }
        }
        Ok(document)
    }

    #[tracing::instrument(skip(self, ctx, operation_name, next))]
    async fn execute(
        &self,
        ctx: &ExtensionContext<'_>,
        operation_name: Option<&str>,
        next: NextExecute<'_>,
    ) -> Response {
        let resp = next.run(ctx, operation_name).await;
        if resp.is_err() {
            for err in &resp.errors {
                if !err.path.is_empty() {
                    let mut path = String::new();
                    for (idx, s) in err.path.iter().enumerate() {
                        if idx > 0 {
                            path.push('.');
                        }
                        match s {
                            PathSegment::Index(idx) => {
                                let _ = write!(&mut path, "{idx}");
                            }
                            PathSegment::Field(name) => {
                                let _ = write!(&mut path, "{name}");
                            }
                        }
                    }

                    log::error!(
                        target: "async-graphql",
                        "[Error] path={} message={}", path, err.message,
                    );
                } else {
                    log::error!(
                        target: "async-graphql",
                        "[Error] message={}", err.message,
                    );
                }
            }
        }
        resp
    }
}
