use crate::context::{BoscaContext, PermissionCheck};
use crate::datastores::guide_cache::GuideCache;
use crate::datastores::notifier::Notifier;
use crate::models::content::document::DocumentInput;
use crate::models::content::guide::{Guide, GuideInput};
use crate::models::content::guide_step::{GuideStep, GuideStepInput};
use crate::models::content::guide_step_module::{GuideStepModule, GuideStepModuleInput};
use crate::models::content::guide_template::{GuideTemplate, GuideTemplateInput};
use crate::models::content::guide_template_step::GuideTemplateStep;
use crate::models::content::guide_template_step_module::GuideTemplateStepModule;
use crate::models::content::metadata::MetadataInput;
use crate::models::content::metadata_profile::MetadataProfileInput;
use crate::models::security::permission::{Permission, PermissionAction};
use async_graphql::*;
use bosca_database::TracingPool;
use chrono::{Timelike, Utc};
use deadpool_postgres::{GenericClient, Transaction};
use log::{error, info};
use rrule::{RRuleSet, Tz};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GuidesDataStore {
    cache: GuideCache,
    pool: TracingPool,
    notifier: Arc<Notifier>,
}

impl GuidesDataStore {
    pub fn new(pool: TracingPool, cache: GuideCache, notifier: Arc<Notifier>) -> Self {
        Self {
            cache,
            pool,
            notifier,
        }
    }

    #[tracing::instrument(skip(self, ctx, id))]
    async fn on_metadata_changed(&self, ctx: &BoscaContext, id: &Uuid) -> Result<(), Error> {
        ctx.content.metadata.update_storage(ctx, id).await?;
        if let Err(e) = self.notifier.metadata_changed(id).await {
            error!("Failed to notify metadata changes: {e:?}");
        }
        Ok(())
    }

    /// Validates template attributes for consistency and correctness
    fn validate_template_attributes(
        &self,
        metadata_id: &Uuid,
        version: i32,
        attributes: &[crate::models::content::template_attribute::TemplateAttributeInput],
    ) -> Result<(), Error> {
        let mut seen_keys = std::collections::HashSet::new();

        for attr in attributes {
            // Validate required fields are not empty
            if attr.key.trim().is_empty() {
                return Err(Error::new(format!(
                    "Template attribute key cannot be empty (template: {metadata_id}, version: {version})"
                )));
            }
            if attr.name.trim().is_empty() {
                return Err(Error::new(format!(
                    "Template attribute name cannot be empty for key '{}' (template: {}, version: {})",
                    attr.key, metadata_id, version
                )));
            }

            // Check for duplicate keys
            if !seen_keys.insert(attr.key.clone()) {
                return Err(Error::new(format!(
                    "Duplicate template attribute key: '{}' (template: {}, version: {})",
                    attr.key, metadata_id, version
                )));
            }

            // Validate key format (should be valid identifier)
            if !attr
                .key
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '.' || c == '-')
            {
                return Err(Error::new(format!(
                    "Invalid template attribute key format: '{}'. Only alphanumeric characters, underscores, dots, and hyphens are allowed (template: {}, version: {})",
                    attr.key, metadata_id, version
                )));
            }

            // Validate supplementary key if present
            if let Some(ref supplementary_key) = attr.supplementary_key {
                if supplementary_key.trim().is_empty() {
                    return Err(Error::new(format!(
                        "Template attribute supplementary_key cannot be empty when provided for key '{}' (template: {}, version: {})",
                        attr.key, metadata_id, version
                    )));
                }
            }
        }

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_templates(&self) -> Result<Vec<GuideTemplate>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from guide_templates")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, metadata_id, version))]
    pub async fn get_template(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<Option<GuideTemplate>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from guide_templates where metadata_id = $1 and version = $2")
            .await?;
        let rows = connection.query(&stmt, &[metadata_id, &version]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    #[tracing::instrument(skip(self, metadata_id, version))]
    pub async fn get_template_attributes(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<Vec<crate::models::content::template_attribute::TemplateAttribute>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from guide_template_attributes where metadata_id = $1 and version = $2 order by sort asc").await?;
        let results = connection.query(&stmt, &[metadata_id, &version]).await?;
        Ok(results.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, metadata_id, version))]
    pub async fn get_template_steps(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<Vec<GuideTemplateStep>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from guide_template_steps where metadata_id = $1 and version = $2 order by sort asc")
            .await?;
        let rows = connection.query(&stmt, &[metadata_id, &version]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, metadata_id, version, id))]
    pub async fn get_template_step(
        &self,
        metadata_id: &Uuid,
        version: i32,
        id: i64,
    ) -> Result<Option<GuideTemplateStep>, Error> {
        info!("ID: {:?}", metadata_id);
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from guide_template_steps where template_metadata_id = $1 and template_metadata_version = $2 and id = $3")
            .await?;
        let rows = connection
            .query(&stmt, &[metadata_id, &version, &id])
            .await?;
        Ok(rows.first().map(|r| r.into()))
    }

    #[tracing::instrument(skip(self, metadata_id, version, step_id))]
    pub async fn get_template_step_modules(
        &self,
        metadata_id: &Uuid,
        version: i32,
        step_id: i64,
    ) -> Result<Vec<GuideTemplateStepModule>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from guide_template_step_modules where metadata_id = $1 and version = $2 and step = $3 order by sort asc")
            .await?;
        let rows = connection
            .query(&stmt, &[metadata_id, &version, &step_id])
            .await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, metadata_id, version, step_id, module_id))]
    pub async fn get_template_step_module(
        &self,
        metadata_id: &Uuid,
        version: i32,
        step_id: i64,
        module_id: i64,
    ) -> Result<Option<GuideTemplateStepModule>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from guide_template_step_modules where metadata_id = $1 and version = $2 and step = $3 and id = $4 order by sort asc")
            .await?;
        let rows = connection
            .query(&stmt, &[metadata_id, &version, &step_id, &module_id])
            .await?;
        Ok(rows.first().map(|r| r.into()))
    }

    #[tracing::instrument(skip(
        self,
        ctx,
        metadata_id,
        version,
        step_id,
        template_metadata_id,
        template_metadata_version
    ))]
    pub async fn add_template_step_module(
        &self,
        ctx: &BoscaContext,
        metadata_id: &Uuid,
        version: i32,
        step_id: i64,
        template_metadata_id: &Uuid,
        template_metadata_version: i32,
    ) -> Result<i64, Error> {
        let connection = self.pool.get().await?;

        // Get current max sort order for this step
        let stmt_max_sort = connection
            .prepare_cached("select coalesce(max(sort), -1) + 1 as next_sort from guide_template_step_modules where metadata_id = $1 and version = $2 and step = $3")
            .await?;
        let row = connection
            .query_one(&stmt_max_sort, &[metadata_id, &version, &step_id])
            .await?;
        let sort: i32 = row.get("next_sort");

        // Insert the new module
        let stmt = connection
            .prepare_cached("insert into guide_template_step_modules (metadata_id, version, step, template_metadata_id, template_metadata_version, sort) values ($1, $2, $3, $4, $5, $6) returning id")
            .await?;
        let result = connection
            .query_one(
                &stmt,
                &[
                    metadata_id,
                    &version,
                    &step_id,
                    template_metadata_id,
                    &template_metadata_version,
                    &sort,
                ],
            )
            .await?;
        let module_id: i64 = result.get("id");

        self.on_metadata_changed(ctx, metadata_id).await?;
        Ok(module_id)
    }

    #[tracing::instrument(skip(self, ctx, metadata_id, version, step_id, module_id))]
    pub async fn remove_template_step_module(
        &self,
        ctx: &BoscaContext,
        metadata_id: &Uuid,
        version: i32,
        step_id: i64,
        module_id: i64,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("delete from guide_template_step_modules where metadata_id = $1 and version = $2 and step = $3 and id = $4")
            .await?;
        connection
            .execute(&stmt, &[metadata_id, &version, &step_id, &module_id])
            .await?;

        self.on_metadata_changed(ctx, metadata_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, metadata_id, version, step_id, module_ids))]
    pub async fn reorder_template_step_modules(
        &self,
        ctx: &BoscaContext,
        metadata_id: &Uuid,
        version: i32,
        step_id: i64,
        module_ids: &[i64],
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;

        let stmt = txn
            .prepare_cached("update guide_template_step_modules set sort = $1 where metadata_id = $2 and version = $3 and step = $4 and id = $5")
            .await?;

        for (index, module_id) in module_ids.iter().enumerate() {
            let sort = index as i32;
            txn.execute(&stmt, &[&sort, metadata_id, &version, &step_id, module_id])
                .await?;
        }

        txn.commit().await?;
        self.on_metadata_changed(ctx, metadata_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, txn, metadata_id, version, template))]
    pub async fn add_template_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        template: &GuideTemplateInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into guide_templates (metadata_id, version, rrule, type, default_attributes, configuration) values ($1, $2, $3, $4, $5, $6)").await?;
        txn.execute(
            &stmt,
            &[
                metadata_id,
                &version,
                &template.rrule,
                &template.guide_type,
                &template.default_attributes,
                &template.configuration,
            ],
        )
        .await?;
        self.add_template_steps_txn(txn, metadata_id, version, template)
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, txn, metadata_id, version, template))]
    pub async fn edit_template_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        template: &GuideTemplateInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into guide_templates (metadata_id, version, rrule, type, default_attributes, configuration) values ($1, $2, $3, $4, $5, $6) on conflict (metadata_id, version) do update set rrule = $3, type = $4, default_attributes = $5, configuration = $6").await?;
        txn.execute(
            &stmt,
            &[
                &metadata_id,
                &version,
                &template.rrule,
                &template.guide_type,
                &template.default_attributes,
                &template.configuration,
            ],
        )
        .await?;
        txn.execute(
            "delete from guide_template_steps where metadata_id = $1 and version = $2",
            &[metadata_id, &version],
        )
        .await?;
        self.add_template_steps_txn(txn, metadata_id, version, template)
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, txn, metadata_id, version, template))]
    async fn add_template_steps_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        template: &GuideTemplateInput,
    ) -> Result<(), Error> {
        let stmt_steps = txn.prepare_cached("insert into guide_template_steps (metadata_id, version, template_metadata_id, template_metadata_version, sort) values ($1, $2, $3, $4, $5) returning id").await?;
        let stmt_step_modules = txn.prepare_cached("insert into guide_template_step_modules (metadata_id, version, step, template_metadata_id, template_metadata_version, sort) values ($1, $2, $3, $4, $5, $6)").await?;
        for (index, step) in template.steps.iter().enumerate() {
            let sort = index as i32;
            let template_metadata_id: Uuid = Uuid::parse_str(&step.template_metadata_id)?;
            let result = txn
                .query_one(
                    &stmt_steps,
                    &[
                        metadata_id,
                        &version,
                        &template_metadata_id,
                        &step.template_metadata_version,
                        &sort,
                    ],
                )
                .await?;
            let step_id: i64 = result.get("id");
            for (index, module) in step.modules.iter().enumerate() {
                let sort = index as i32;
                let template_metadata_id = Uuid::parse_str(module.template_metadata_id.as_str())?;
                txn.execute(
                    &stmt_step_modules,
                    &[
                        metadata_id,
                        &version,
                        &step_id,
                        &template_metadata_id,
                        &module.template_metadata_version,
                        &sort,
                    ],
                )
                .await?;
            }
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, metadata_id, version))]
    pub async fn get_guide(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<Option<Guide>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from guides where metadata_id = $1 and version = $2")
            .await?;
        let rows = connection.query(&stmt, &[metadata_id, &version]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    #[tracing::instrument(skip(self, ctx, metadata_id, version, rrule))]
    pub async fn set_guide_rrule(
        &self,
        ctx: &BoscaContext,
        metadata_id: &Uuid,
        version: i32,
        rrule: RRuleSet,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update guides set rrule = $1 where metadata_id = $2 and version = $3")
            .await?;
        let rrule = rrule.to_string();
        connection
            .execute(&stmt, &[&rrule, metadata_id, &version])
            .await?;
        self.on_metadata_changed(ctx, metadata_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, metadata_id, version, id))]
    pub async fn get_guide_step(
        &self,
        metadata_id: &Uuid,
        version: i32,
        id: i64,
    ) -> Result<Option<GuideStep>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select * from guide_steps where metadata_id = $1 and version = $2 and id = $3",
            )
            .await?;
        let rows = connection
            .query_one(&stmt, &[metadata_id, &version, &id])
            .await?;
        if rows.is_empty() {
            return Ok(None);
        }
        Ok(Some((&rows).into()))
    }

    #[tracing::instrument(skip(self, metadata_id, version))]
    pub async fn get_guide_step_ids(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<Vec<i64>, Error> {
        if let Some(ids) = self.cache.get_guide_step_ids(metadata_id).await {
            return Ok(ids);
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select id from guide_steps where metadata_id = $1 and version = $2 order by sort asc")
            .await?;
        let rows = connection.query(&stmt, &[metadata_id, &version]).await?;
        let ids = rows.iter().map(|r| r.get("id")).collect();
        self.cache.set_guide_step_ids(metadata_id, &ids).await;
        Ok(ids)
    }

    #[tracing::instrument(skip(self, metadata_id, version, offset, limit))]
    pub async fn get_guide_steps(
        &self,
        metadata_id: &Uuid,
        version: i32,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<Vec<GuideStep>, Error> {
        let connection = self.pool.get().await?;
        if let Some(offset) = offset {
            if let Some(limit) = limit {
                let stmt = connection
                    .prepare_cached("select * from guide_steps where metadata_id = $1 and version = $2 order by sort asc offset $3 limit $4")
                    .await?;
                let rows = connection
                    .query(&stmt, &[metadata_id, &version, &offset, &limit])
                    .await?;
                return Ok(rows.iter().map(|r| r.into()).collect());
            }
            let stmt = connection
                .prepare_cached("select * from guide_steps where metadata_id = $1 and version = $2 order by sort asc offset $3")
                .await?;
            let rows = connection
                .query(&stmt, &[metadata_id, &version, &offset])
                .await?;
            return Ok(rows.iter().map(|r| r.into()).collect());
        }
        let stmt = connection
            .prepare_cached("select * from guide_steps where metadata_id = $1 and version = $2 order by sort asc")
            .await?;
        let rows = connection.query(&stmt, &[metadata_id, &version]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, metadata_id, version))]
    pub async fn get_guide_step_count(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<i64, Error> {
        if let Some(count) = self.cache.get_guide_step_count(metadata_id).await {
            return Ok(count);
        }
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select count(*) as count from guide_steps where metadata_id = $1 and version = $2",
            )
            .await?;
        let rows = connection
            .query_one(&stmt, &[metadata_id, &version])
            .await?;
        let count: i64 = rows.get("count");
        self.cache.set_guide_step_count(metadata_id, count).await;
        Ok(count)
    }

    #[tracing::instrument(skip(self, metadata_id, version, step_id))]
    pub async fn get_guide_step_modules(
        &self,
        metadata_id: &Uuid,
        version: i32,
        step_id: i64,
    ) -> Result<Vec<GuideStepModule>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from guide_step_modules where metadata_id = $1 and version = $2 and step = $3 order by sort asc")
            .await?;
        let rows = connection
            .query(&stmt, &[metadata_id, &version, &step_id])
            .await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, ctx, txn, metadata_id, version, guide))]
    pub async fn add_guide_txn(
        &self,
        ctx: &BoscaContext,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        guide: &GuideInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into guides (metadata_id, version, template_metadata_id, template_metadata_version, rrule, type) values ($1, $2, $3, $4, $5, $6)").await?;
        let template_metadata_id = guide
            .template_metadata_id
            .as_ref()
            .map(|id| Uuid::parse_str(id.as_str()).unwrap());
        let rrule: Option<RRuleSet> = guide.rrule.as_ref().map(|r| r.parse().unwrap());
        let rrule = rrule.map(|r| r.to_string()).unwrap_or_default();
        txn.execute(
            &stmt,
            &[
                metadata_id,
                &version,
                &template_metadata_id,
                &guide.template_metadata_version,
                &rrule,
                &guide.guide_type,
            ],
        )
        .await?;
        for (index, step) in guide.steps.iter().enumerate() {
            self.add_guide_step_txn(ctx, txn, metadata_id, version, index as i32, step)
                .await?;
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, txn, metadata_id, version, guide))]
    pub async fn edit_guide_txn(
        &self,
        ctx: &BoscaContext,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        guide: &GuideInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into guides (metadata_id, version, template_metadata_id, template_metadata_version, rrule, type) values ($1, $2, $3, $4, $5, $6) on conflict (metadata_id, version) do update set template_metadata_id = $3, template_metadata_version = $4, rrule = $5, type = $6").await?;
        let template_metadata_id = guide
            .template_metadata_id
            .as_ref()
            .map(|id| Uuid::parse_str(id.as_str()).unwrap());
        let rrule: Option<RRuleSet> = guide.rrule.as_ref().map(|r| r.parse().unwrap());
        let rrule = rrule.map(|r| r.to_string()).unwrap_or_default();
        txn.execute(
            &stmt,
            &[
                metadata_id,
                &version,
                &template_metadata_id,
                &guide.template_metadata_version,
                &rrule,
                &guide.guide_type,
            ],
        )
        .await?;
        txn.execute(
            "delete from guide_steps where metadata_id = $1 and version = $2",
            &[metadata_id, &version],
        )
        .await?;
        for (index, step) in guide.steps.iter().enumerate() {
            self.add_guide_step_txn(ctx, txn, metadata_id, version, index as i32, step)
                .await?;
        }
        self.cache.evict_guide(metadata_id).await;
        Ok(())
    }

    #[tracing::instrument(skip(
        self,
        ctx,
        txn,
        metadata,
        metadata_id,
        metadata_version,
        collection_item_attributes
    ))]
    async fn add_document_txn(
        &self,
        ctx: &BoscaContext,
        txn: &Transaction<'_>,
        metadata: &Option<MetadataInput>,
        metadata_id: &Option<String>,
        metadata_version: Option<i32>,
        collection_item_attributes: Option<serde_json::Value>,
    ) -> Result<(Uuid, i32), Error> {
        Ok(if let Some(metadata) = metadata {
            let (id, version, _, _) = Box::pin(ctx.content.metadata.add_txn(
                ctx,
                txn,
                metadata,
                true,
                &collection_item_attributes,
            ))
            .await?;
            (id, version)
        } else if let Some(id) = &metadata_id {
            if let Some(version) = metadata_version {
                (Uuid::parse_str(id.as_str())?, version)
            } else {
                return Err(Error::new("missing metadata or metadata id and version"));
            }
        } else {
            return Err(Error::new("missing metadata or metadata id and version"));
        })
    }

    #[tracing::instrument(skip(self, ctx, txn, metadata_id, version, index, step))]
    async fn add_guide_step_txn(
        &self,
        ctx: &BoscaContext,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        index: i32,
        step: &GuideStepInput,
    ) -> Result<GuideStep, Error> {
        let (step_metadata_id, step_metadata_version) = self
            .add_document_txn(
                ctx,
                txn,
                &step.metadata,
                &step.step_metadata_id,
                step.step_metadata_version,
                None,
            )
            .await?;
        let stmt = txn.prepare_cached("insert into guide_steps (metadata_id, version, step_metadata_id, step_metadata_version, sort) values ($1, $2, $3, $4, $5) returning id").await?;
        let sort = index;
        let result = txn
            .query_one(
                &stmt,
                &[
                    metadata_id,
                    &version,
                    &step_metadata_id,
                    &step_metadata_version,
                    &sort,
                ],
            )
            .await?;
        let step_id: i64 = result.get("id");
        for (index, module) in step.modules.iter().enumerate() {
            let sort = index as i32;
            self.add_guide_step_module_txn(ctx, txn, metadata_id, version, step_id, sort, module)
                .await?;
        }
        Ok(GuideStep {
            id: step_id,
            step_metadata_id,
            step_metadata_version,
            metadata_id: *metadata_id,
            metadata_version: version,
            sort,
        })
    }

    #[tracing::instrument(skip(self, txn, metadata_id, version, step_id))]
    pub async fn delete_guide_step_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        step_id: i64,
    ) -> Result<(), Error> {
        let stmt = txn
            .prepare_cached(
                "delete from guide_steps where metadata_id = $1 and version = $2 and id = $3",
            )
            .await?;
        txn.execute(&stmt, &[metadata_id, &version, &step_id])
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, txn, metadata_id, version, step_id, index, module))]
    #[allow(clippy::too_many_arguments)]
    async fn add_guide_step_module_txn(
        &self,
        ctx: &BoscaContext,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        step_id: i64,
        index: i32,
        module: &GuideStepModuleInput,
    ) -> Result<GuideStepModule, Error> {
        let (module_metadata_id, module_metadata_version) = self
            .add_document_txn(
                ctx,
                txn,
                &module.metadata,
                &module.module_metadata_id,
                module.module_metadata_version,
                None,
            )
            .await?;

        let stmt_module = txn.prepare_cached("insert into guide_step_modules (metadata_id, version, step, module_metadata_id, module_metadata_version, sort) values ($1, $2, $3, $4, $5, $6) returning id").await?;
        let result = txn
            .query_one(
                &stmt_module,
                &[
                    metadata_id,
                    &version,
                    &step_id,
                    &module_metadata_id,
                    &module_metadata_version,
                    &index,
                ],
            )
            .await?;
        let module_id: i64 = result.get("id");

        Ok(GuideStepModule {
            id: module_id,
            module_metadata_id,
            module_metadata_version,
        })
    }

    #[tracing::instrument(skip(self, txn, metadata_id, version, step_id, module_id))]
    #[allow(clippy::too_many_arguments)]
    pub async fn delete_guide_step_module_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        step_id: i64,
        module_id: i64,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("delete from guide_step_modules where metadata_id = $1 and version = $2 and step_id = $3 and id = $4").await?;
        txn.execute(&stmt, &[metadata_id, &version, &step_id, &module_id])
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(
        self,
        ctx,
        parent_collection_id,
        template_metadata_id,
        template_metadata_version,
        permissions
    ))]
    pub async fn add_guide_from_template(
        &self,
        ctx: &BoscaContext,
        parent_collection_id: &Uuid,
        template_metadata_id: &Uuid,
        template_metadata_version: i32,
        permissions: &[Permission],
    ) -> Result<(Uuid, i32), Error> {
        let mut conn = self.pool.get().await?;
        let txn = conn.transaction().await?;
        let check = PermissionCheck::new_with_metadata_id_with_version(
            *template_metadata_id,
            template_metadata_version,
            PermissionAction::View,
        );
        let template = ctx.metadata_permission_check(check).await?;
        if template.content_type != "bosca/v-guide-template" {
            return Err(Error::new("invalid template"));
        }
        let Some(template_document) = ctx
            .content
            .documents
            .get_template(&template.id, template.version)
            .await?
        else {
            return Err(Error::new("missing template"));
        };
        let Some(template_guide) = ctx
            .content
            .guides
            .get_template(&template.id, template.version)
            .await?
        else {
            return Err(Error::new("missing guide"));
        };
        let mut attrs = json!({
            "editor.type": "Guide",
        });
        if let Some(default_attributes) = &template_document.default_attributes {
            if let serde_json::Value::Object(ref mut attrs_obj) = attrs {
                if let serde_json::Value::Object(default_obj) = default_attributes.clone() {
                    attrs_obj.extend(default_obj.into_iter());
                }
            }
        }
        let profile = ctx.profile.get_by_principal(&ctx.principal.id).await?;
        let categories = ctx
            .content
            .collections
            .get_categories(parent_collection_id)
            .await?;
        let template_steps = ctx
            .content
            .guides
            .get_template_steps(&template.id, template.version)
            .await?;
        let metadata = MetadataInput {
            parent_collection_id: Some(parent_collection_id.to_string()),
            category_ids: Some(categories.iter().map(|c| c.id.to_string()).collect()),
            name: "New Guide".to_string(),
            content_type: "bosca/v-guide".to_string(),
            language_tag: template.language_tag,
            attributes: Some(attrs),
            guide: Some(GuideInput {
                guide_type: template_guide.guide_type,
                rrule: template_guide.rrule.map(|rrule| {
                    let n = Utc::now()
                        .with_timezone(&Tz::UTC)
                        .with_hour(0)
                        .unwrap()
                        .with_minute(0)
                        .unwrap()
                        .with_second(0)
                        .unwrap()
                        .with_nanosecond(0)
                        .unwrap();
                    RRuleSet::new(n)
                        .set_exdates(rrule.get_exdate().clone())
                        .set_rdates(rrule.get_rdate().clone())
                        .set_rrules(rrule.get_rrule().clone())
                        .to_string()
                }),
                template_metadata_id: Some(template.id.to_string()),
                template_metadata_version: Some(template.version),
                steps: vec![],
            }),
            document: Some(DocumentInput {
                template_metadata_id: Some(template.id.to_string()),
                template_metadata_version: Some(template.version),
                title: "New Guide".to_string(),
                content: template_document.content.clone(),
            }),
            profiles: profile.map(|p| {
                vec![MetadataProfileInput {
                    profile_id: p.id.to_string(),
                    relationship: "author".to_string(),
                }]
            }),
            ..Default::default()
        };
        let (metadata_id, version, _, _) = ctx
            .content
            .metadata
            .add_txn(ctx, &txn, &metadata, true, &None)
            .await?;
        for (index, template_step) in template_steps.iter().enumerate() {
            self.add_guide_step_from_template_txn(
                ctx,
                &txn,
                &metadata_id,
                version,
                index,
                template_step,
                permissions,
            )
            .await?;
        }
        txn.commit().await?;
        Ok((metadata_id, version))
    }

    #[tracing::instrument(skip(
        self,
        ctx,
        metadata_id,
        metadata_version,
        index,
        step,
        permissions
    ))]
    pub async fn add_guide_step_from_template(
        &self,
        ctx: &BoscaContext,
        metadata_id: &Uuid,
        metadata_version: i32,
        index: usize,
        step: &GuideTemplateStep,
        permissions: &[Permission],
    ) -> Result<GuideStep, Error> {
        let mut conn = self.pool.get().await?;
        let txn = conn.transaction().await?;
        let step = self
            .add_guide_step_from_template_txn(
                ctx,
                &txn,
                metadata_id,
                metadata_version,
                index,
                step,
                permissions,
            )
            .await?;
        txn.commit().await?;
        self.on_metadata_changed(ctx, metadata_id).await?;
        Ok(step)
    }

    #[tracing::instrument(skip(
        self,
        ctx,
        txn,
        metadata_id,
        metadata_version,
        index,
        step,
        permissions
    ))]
    #[allow(clippy::too_many_arguments)]
    pub async fn add_guide_step_from_template_txn(
        &self,
        ctx: &BoscaContext,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        metadata_version: i32,
        index: usize,
        step: &GuideTemplateStep,
        permissions: &[Permission],
    ) -> Result<GuideStep, Error> {
        let title = if index == 0 {
            "New Step".to_string()
        } else {
            format!("New Step {}", index + 1)
        };

        let (step_metadata_id, step_metadata_version) = ctx
            .content
            .documents
            .add_document_from_template_txn(
                ctx,
                txn,
                None,
                &title,
                &step.template_metadata_id,
                step.template_metadata_version,
                "bosca/v-guide-step",
                permissions,
            )
            .await?;

        let mut guide_step_ids = Vec::new();

        let stmt = txn
            .prepare_cached("select id from guide_steps where metadata_id = $1 and version = $2")
            .await?;
        let response = txn.query(&stmt, &[&metadata_id, &metadata_version]).await?;
        for res in &response {
            let id: Option<i64> = res.get("id");
            if let Some(id) = id {
                guide_step_ids.push(id);
            }
        }

        let mut cur = 0;
        loop {
            if cur == guide_step_ids.len() {
                break;
            }
            let id = guide_step_ids[cur];
            let sort = cur as i32;
            txn.execute(
                "update guide_steps set sort = $1 where id = $2",
                &[&sort, &id],
            )
            .await?;
            if cur == index {
                break;
            }
            cur += 1;
        }

        let stmt = txn.prepare_cached("insert into guide_steps (metadata_id, version, step_metadata_id, step_metadata_version, sort) values ($1, $2, $3, $4, $5) returning id").await?;

        let step_sort = cur as i32;
        let result = txn
            .query_one(
                &stmt,
                &[
                    metadata_id,
                    &metadata_version,
                    &step_metadata_id,
                    &step_metadata_version,
                    &step_sort,
                ],
            )
            .await?;
        let step_id: i64 = result.get("id");

        loop {
            if cur == guide_step_ids.len() {
                break;
            }
            let id = guide_step_ids[cur];
            let sort = cur as i32;
            txn.execute(
                "update guide_steps set sort = $1 where id = $2",
                &[&sort, &id],
            )
            .await?;
            cur += 1;
        }

        let modules = ctx
            .content
            .guides
            .get_template_step_modules(
                &step.template_metadata_id,
                step.template_metadata_version,
                index as i64,
            )
            .await?;

        let mut new_modules = Vec::new();
        for (index, module) in modules.iter().enumerate() {
            let module = self
                .add_guide_module_from_template_txn(
                    ctx,
                    txn,
                    metadata_id,
                    metadata_version,
                    step_id,
                    index,
                    module,
                    permissions,
                )
                .await?;
            new_modules.push(module);
        }

        ctx.content
            .metadata_permissions
            .add_metadata_permissions_txn(txn, metadata_id, permissions)
            .await?;

        Ok(GuideStep {
            id: step_id,
            metadata_id: *metadata_id,
            metadata_version,
            step_metadata_id,
            step_metadata_version,
            sort: step_sort,
        })
    }

    #[tracing::instrument(skip(
        self,
        ctx,
        metadata_id,
        metadata_version,
        step_id,
        index,
        module,
        permissions
    ))]
    #[allow(clippy::too_many_arguments)]
    pub async fn add_guide_module_from_template(
        &self,
        ctx: &BoscaContext,
        metadata_id: &Uuid,
        metadata_version: i32,
        step_id: i64,
        index: usize,
        module: &GuideTemplateStepModule,
        permissions: &[Permission],
    ) -> Result<GuideStepModule, Error> {
        let mut conn = self.pool.get().await?;
        let txn = conn.transaction().await?;
        let module = self
            .add_guide_module_from_template_txn(
                ctx,
                &txn,
                metadata_id,
                metadata_version,
                step_id,
                index,
                module,
                permissions,
            )
            .await?;
        txn.commit().await?;
        Ok(module)
    }

    #[tracing::instrument(skip(
        self,
        ctx,
        metadata_id,
        metadata_version,
        step_id,
        index,
        module,
        permissions
    ))]
    #[allow(clippy::too_many_arguments)]
    pub async fn add_guide_module_from_template_txn(
        &self,
        ctx: &BoscaContext,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        metadata_version: i32,
        step_id: i64,
        index: usize,
        module: &GuideTemplateStepModule,
        permissions: &[Permission],
    ) -> Result<GuideStepModule, Error> {
        let title = if index == 0 {
            "New Module".to_string()
        } else {
            format!("New Module {}", index + 1)
        };
        let (module_metadata_id, module_metadata_version) = ctx
            .content
            .documents
            .add_document_from_template_txn(
                ctx,
                txn,
                None,
                &title,
                &module.template_metadata_id,
                module.template_metadata_version,
                "bosca/v-guide-module",
                permissions,
            )
            .await?;
        let stmt_module = txn.prepare_cached("insert into guide_step_modules (metadata_id, version, step, module_metadata_id, module_metadata_version, sort) values ($1, $2, $3, $4, $5, $6) returning id").await?;
        let sort = index as i32;
        let result = txn
            .query_one(
                &stmt_module,
                &[
                    metadata_id,
                    &metadata_version,
                    &step_id,
                    &module_metadata_id,
                    &module_metadata_version,
                    &sort,
                ],
            )
            .await?;
        ctx.content
            .metadata_permissions
            .add_metadata_permissions_txn(txn, metadata_id, permissions)
            .await?;
        let module_id: i64 = result.get("id");
        Ok(GuideStepModule {
            id: module_id,
            module_metadata_id,
            module_metadata_version,
        })
    }

    #[tracing::instrument(skip(self, ctx, metadata_id, version, step_id))]
    pub async fn delete_guide_step(
        &self,
        ctx: &BoscaContext,
        metadata_id: &Uuid,
        version: i32,
        step_id: i64,
    ) -> Result<(), Error> {
        let Some(step) = self.get_guide_step(metadata_id, version, step_id).await? else {
            return Err(Error::new(format!(
                "Guide step not found (guide: {metadata_id}, version: {version}, step_id: {step_id})"
            )));
        };
        let modules = self
            .get_guide_step_modules(metadata_id, version, step_id)
            .await?;
        let mut conn = self.pool.get().await?;
        let txn = conn.transaction().await?;
        for module in modules.iter() {
            self.delete_guide_step_module_txn(&txn, metadata_id, version, step_id, module.id)
                .await?;
            ctx.content
                .metadata
                .mark_deleted(ctx, &module.module_metadata_id)
                .await?;
        }
        self.delete_guide_step_txn(&txn, metadata_id, version, step_id)
            .await?;
        txn.commit().await?;
        ctx.content
            .metadata
            .mark_deleted(ctx, &step.step_metadata_id)
            .await?;
        for module in modules {
            self.on_metadata_changed(ctx, &module.module_metadata_id)
                .await?;
        }
        self.on_metadata_changed(ctx, &step.step_metadata_id)
            .await?;
        self.on_metadata_changed(ctx, metadata_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, metadata_id, version))]
    pub async fn delete_guide(
        &self,
        ctx: &BoscaContext,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<(), Error> {
        let steps = self
            .get_guide_steps(metadata_id, version, None, None)
            .await?;
        for step in steps.iter() {
            self.delete_guide_step(ctx, metadata_id, version, step.id)
                .await?;
        }
        ctx.content.metadata.mark_deleted(ctx, metadata_id).await?;
        self.on_metadata_changed(ctx, metadata_id).await?;
        Ok(())
    }

    // TODO: decide if this is needed
    // #[tracing::instrument(skip(self, ctx, metadata_id, version, template))]
    // pub async fn edit_template(
    //     &self,
    //     ctx: &BoscaContext,
    //     metadata_id: &Uuid,
    //     version: i32,
    //     template: &GuideTemplateInput,
    // ) -> Result<(), Error> {
    //     let mut conn = self.pool.get().await?;
    //     let txn = conn.transaction().await?;
    //     self.edit_template_txn(&txn, metadata_id, version, template).await?;
    //     txn.commit().await?;
    //     self.on_metadata_changed(ctx, metadata_id).await?;
    //     Ok(())
    // }

    #[tracing::instrument(skip(self, metadata_id, version, default_attributes))]
    pub async fn set_default_attributes(
        &self,
        metadata_id: &Uuid,
        version: i32,
        default_attributes: &serde_json::Value,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("update guide_templates set default_attributes = $1 where metadata_id = $2 and version = $3").await?;
        txn.execute(&stmt, &[default_attributes, metadata_id, &version])
            .await?;
        txn.commit().await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, metadata_id, version, configuration))]
    pub async fn set_configuration(
        &self,
        metadata_id: &Uuid,
        version: i32,
        configuration: &serde_json::Value,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("update guide_templates set configuration = $1 where metadata_id = $2 and version = $3").await?;
        txn.execute(&stmt, &[configuration, metadata_id, &version])
            .await?;
        txn.commit().await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, metadata_id, version, guide_type))]
    pub async fn set_template_type(
        &self,
        metadata_id: &Uuid,
        version: i32,
        guide_type: &str,
    ) -> Result<(), Error> {
        // Convert the string to GuideType enum
        let guide_type_enum = match guide_type.to_uppercase().as_str() {
            "LINEAR" => crate::models::content::guide_type::GuideType::Linear,
            "LINEAR_PROGRESS" => crate::models::content::guide_type::GuideType::LinearProgress,
            "CALENDAR" => crate::models::content::guide_type::GuideType::Calendar,
            "CALENDAR_PROGRESS" => crate::models::content::guide_type::GuideType::CalendarProgress,
            _ => return Err(Error::new(format!("Invalid guide type: {guide_type}"))),
        };

        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached(
                "update guide_templates set type = $1 where metadata_id = $2 and version = $3",
            )
            .await?;
        txn.execute(&stmt, &[&guide_type_enum, metadata_id, &version])
            .await?;
        txn.commit().await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, metadata_id, version, rrule))]
    pub async fn set_template_rrule(
        &self,
        metadata_id: &Uuid,
        version: i32,
        rrule: &str,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached(
                "update guide_templates set rrule = $1 where metadata_id = $2 and version = $3",
            )
            .await?;
        txn.execute(&stmt, &[&rrule, metadata_id, &version]).await?;
        txn.commit().await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, ctx, metadata_id, version, attributes))]
    pub async fn set_template_attributes(
        &self,
        ctx: &BoscaContext,
        metadata_id: &Uuid,
        version: i32,
        attributes: &[crate::models::content::template_attribute::TemplateAttributeInput],
    ) -> Result<(), Error> {
        // Validate attributes before proceeding
        self.validate_template_attributes(metadata_id, version, attributes)?;

        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        txn.execute(
            "delete from guide_template_attributes where metadata_id = $1 and version = $2",
            &[metadata_id, &version],
        )
        .await?;

        let stmt_attr = txn.prepare_cached("insert into guide_template_attributes (metadata_id, version, key, name, description, supplementary_key, configuration, type, ui, list, sort) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)").await?;

        for (index, attr) in attributes.iter().enumerate() {
            let sort = index as i32;
            txn.execute(
                &stmt_attr,
                &[
                    metadata_id,
                    &version,
                    &attr.key,
                    &attr.name,
                    &attr.description,
                    &attr.supplementary_key,
                    &attr.configuration,
                    &attr.attribute_type,
                    &attr.ui,
                    &attr.list,
                    &sort,
                ],
            )
            .await?;
        }

        txn.commit().await?;
        self.on_metadata_changed(ctx, metadata_id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, metadata_id, version, sort, attr))]
    pub async fn add_template_attribute(
        &self,
        metadata_id: &Uuid,
        version: i32,
        sort: i32,
        attr: &crate::models::content::template_attribute::TemplateAttributeInput,
    ) -> Result<(), Error> {
        // Validate single attribute
        self.validate_template_attributes(metadata_id, version, &[attr.clone()])?;

        // Check for duplicate key in existing attributes
        let existing_attrs = self.get_template_attributes(metadata_id, version).await?;
        if existing_attrs
            .iter()
            .any(|existing| existing.key == attr.key)
        {
            return Err(Error::new(format!(
                "Template attribute with key '{}' already exists (template: {}, version: {})",
                attr.key, metadata_id, version
            )));
        }

        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;

        let stmt_attr = txn.prepare_cached("insert into guide_template_attributes (metadata_id, version, key, name, description, supplementary_key, configuration, type, ui, list, sort) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)").await?;

        txn.execute(
            &stmt_attr,
            &[
                metadata_id,
                &version,
                &attr.key,
                &attr.name,
                &attr.description,
                &attr.supplementary_key,
                &attr.configuration,
                &attr.attribute_type,
                &attr.ui,
                &attr.list,
                &sort,
            ],
        )
        .await?;

        txn.commit().await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, metadata_id, version, key))]
    pub async fn delete_template_attribute(
        &self,
        metadata_id: &Uuid,
        version: i32,
        key: &str,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("delete from guide_template_attributes where metadata_id = $1 and version = $2 and key = $3").await?;
        txn.execute(&stmt, &[metadata_id, &version, &key]).await?;
        txn.commit().await?;
        Ok(())
    }

    #[tracing::instrument(skip(
        self,
        metadata_id,
        version,
        step_metadata_id,
        step_metadata_version
    ))]
    pub async fn add_template_step(
        &self,
        metadata_id: &Uuid,
        version: i32,
        step_metadata_id: &str,
        step_metadata_version: Option<i32>,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let step_metadata_id = Uuid::parse_str(step_metadata_id)?;

        // Get the current highest sort value
        let stmt_max_sort = txn.prepare_cached("select coalesce(max(sort), -1) + 1 as next_sort from guide_template_steps where metadata_id = $1 and version = $2").await?;
        let row = txn
            .query_one(&stmt_max_sort, &[metadata_id, &version])
            .await?;
        let sort: i32 = row.get("next_sort");

        let stmt = txn.prepare_cached("insert into guide_template_steps (metadata_id, version, template_metadata_id, template_metadata_version, sort) values ($1, $2, $3, $4, $5)").await?;
        txn.execute(
            &stmt,
            &[
                metadata_id,
                &version,
                &step_metadata_id,
                &step_metadata_version,
                &sort,
            ],
        )
        .await?;
        txn.commit().await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, metadata_id, version, step_id))]
    pub async fn remove_template_step(
        &self,
        metadata_id: &Uuid,
        version: i32,
        step_id: i64,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("delete from guide_template_steps where metadata_id = $1 and version = $2 and id = $3").await?;
        txn.execute(&stmt, &[metadata_id, &version, &step_id])
            .await?;
        txn.commit().await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, metadata_id, version, step_ids))]
    pub async fn reorder_template_steps(
        &self,
        metadata_id: &Uuid,
        version: i32,
        step_ids: &[i64],
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;

        let stmt = txn.prepare_cached("update guide_template_steps set sort = $1 where metadata_id = $2 and version = $3 and id = $4").await?;

        for (index, step_id) in step_ids.iter().enumerate() {
            let sort = index as i32;
            txn.execute(&stmt, &[&sort, metadata_id, &version, step_id])
                .await?;
        }

        txn.commit().await?;
        Ok(())
    }
}
