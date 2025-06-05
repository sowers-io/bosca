use crate::context::BoscaContext;
use crate::models::content::collection_template::{CollectionTemplate, CollectionTemplateInput};
use crate::models::content::template_attribute::{TemplateAttribute, TemplateAttributeInput};
use crate::models::content::template_workflow::TemplateWorkflow;
use async_graphql::*;
use bosca_database::TracingPool;
use deadpool_postgres::{GenericClient, Transaction};
use uuid::Uuid;

#[derive(Clone)]
pub struct CollectionTemplatesDataStore {
    pool: TracingPool,
}

impl CollectionTemplatesDataStore {
    pub fn new(pool: TracingPool) -> Self {
        Self { pool }
    }

    #[tracing::instrument(skip(self, ctx, id))]
    async fn on_metadata_changed(&self, ctx: &BoscaContext, id: &Uuid) -> Result<(), Error> {
        ctx.content.metadata.on_metadata_changed(ctx, id).await?;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_templates(&self) -> Result<Vec<CollectionTemplate>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from document_templates")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, metadata_id, version))]
    pub async fn get_template(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<Option<CollectionTemplate>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select * from collection_templates where metadata_id = $1 and version = $2",
            )
            .await?;
        let rows = connection.query(&stmt, &[metadata_id, &version]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    #[tracing::instrument(skip(self, metadata_id, version))]
    pub async fn get_template_attributes(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<Vec<TemplateAttribute>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from collection_template_attributes where metadata_id = $1 and version = $2 order by sort asc").await?;
        let results = connection.query(&stmt, &[metadata_id, &version]).await?;
        Ok(results.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, metadata_id, version, key))]
    pub async fn get_template_attribute_workflows(
        &self,
        metadata_id: &Uuid,
        version: i32,
        key: &String,
    ) -> Result<Vec<TemplateWorkflow>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from collection_template_attribute_workflows where metadata_id = $1 and version = $2 and key = $3").await?;
        let results = connection
            .query(&stmt, &[metadata_id, &version, key])
            .await?;
        Ok(results.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, txn, metadata_id, version, template))]
    pub async fn add_template_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        template: &CollectionTemplateInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into collection_templates (metadata_id, version, default_attributes, configuration, filters, ordering) values ($1, $2, $3, $4, $5, $6)").await?;
        let filters = template
            .filters
            .as_ref()
            .map(|f| serde_json::to_value(f).unwrap());
        let ordering = template
            .ordering
            .as_ref()
            .map(|f| serde_json::to_value(f).unwrap());
        txn.execute(
            &stmt,
            &[
                metadata_id,
                &version,
                &template.default_attributes,
                &template.configuration,
                &filters,
                &ordering,
            ],
        )
        .await?;
        self.add_template_items_txn(txn, metadata_id, version, &template.attributes)
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, txn, metadata_id, version, template))]
    pub async fn edit_template_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        template: &CollectionTemplateInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("update collection_templates set default_attributes = $1, configuration = $2, filters = $3, ordering = $4 where metadata_id = $5 and version = $6").await?;
        let filters = template
            .filters
            .as_ref()
            .map(|f| serde_json::to_value(f).unwrap());
        let ordering = template
            .ordering
            .as_ref()
            .map(|f| serde_json::to_value(f).unwrap());
        txn.execute(
            &stmt,
            &[
                &template.default_attributes,
                &template.configuration,
                &filters,
                &ordering,
                &metadata_id,
                &version,
            ],
        )
        .await?;
        txn.execute(
            "delete from collection_template_attributes where metadata_id = $1 and version = $2",
            &[metadata_id, &version],
        )
        .await?;
        self.add_template_items_txn(txn, metadata_id, version, &template.attributes)
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, txn, metadata_id, version, attributes))]
    async fn add_template_items_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        attributes: &[TemplateAttributeInput],
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into collection_template_attributes (metadata_id, version, key, name, description, configuration, type, ui, list, sort, supplementary_key, location) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)").await?;
        let stmt_wid = txn.prepare_cached("insert into collection_template_attribute_workflows (metadata_id, version, key, workflow_id, auto_run) values ($1, $2, $3, $4, $5)").await?;
        for (index, attr) in attributes.iter().enumerate() {
            let sort = index as i32;
            txn.execute(
                &stmt,
                &[
                    metadata_id,
                    &version,
                    &attr.key,
                    &attr.name,
                    &attr.description,
                    &attr.configuration,
                    &attr.attribute_type,
                    &attr.ui,
                    &attr.list,
                    &sort,
                    &attr.supplementary_key,
                    &attr.location,
                ],
            )
            .await?;
            for wid in &attr.workflows {
                txn.execute(
                    &stmt_wid,
                    &[
                        metadata_id,
                        &version,
                        &attr.key,
                        &wid.workflow_id,
                        &wid.auto_run,
                    ],
                )
                .await?;
            }
        }
        Ok(())
    }

    pub async fn set_template_attributes(
        &self,
        ctx: &BoscaContext,
        metadata_id: &Uuid,
        version: i32,
        attributes: &[TemplateAttributeInput],
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        txn.execute(
            "delete from collection_template_attributes where metadata_id = $1 and version = $2",
            &[metadata_id, &version],
        )
            .await?;
        self.add_template_items_txn(&txn, metadata_id, version, attributes)
            .await?;
        txn.commit().await?;
        ctx.content.metadata.on_metadata_changed(ctx, metadata_id).await?;
        Ok(())
    }


    pub async fn set_configuration(
        &self,
        metadata_id: &Uuid,
        version: i32,
        configuration: &serde_json::Value,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("update collection_templates set configuration = $1 where metadata_id = $2 and version = $3").await?;
        txn.execute(&stmt, &[configuration, metadata_id, &version])
            .await?;
        txn.commit().await?;
        Ok(())
    }

    pub async fn set_default_attributes(
        &self,
        metadata_id: &Uuid,
        version: i32,
        default_attributes: &serde_json::Value,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("update collection_templates set default_attributes = $1 where metadata_id = $2 and version = $3").await?;
        txn.execute(&stmt, &[default_attributes, metadata_id, &version])
            .await?;
        txn.commit().await?;
        Ok(())
    }

    pub async fn set_filters(
        &self,
        ctx: &BoscaContext,
        metadata_id: &Uuid,
        version: i32,
        filters: &serde_json::Value,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("update collection_templates set filters = $1 where metadata_id = $2 and version = $3").await?;
        txn.execute(&stmt, &[filters, metadata_id, &version])
            .await?;
        txn.commit().await?;
        ctx.content
            .metadata
            .on_metadata_changed(ctx, metadata_id)
            .await?;
        Ok(())
    }

    pub async fn set_ordering(
        &self,
        ctx: &BoscaContext,
        metadata_id: &Uuid,
        version: i32,
        ordering: &serde_json::Value,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("update collection_templates set ordering = $1 where metadata_id = $2 and version = $3").await?;
        txn.execute(&stmt, &[ordering, metadata_id, &version])
            .await?;
        txn.commit().await?;
        ctx.content
            .metadata
            .on_metadata_changed(ctx, metadata_id)
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, metadata_id, version, sort, attr))]
    pub async fn add_template_attribute(
        &self,
        metadata_id: &Uuid,
        version: i32,
        sort: i32,
        attr: &TemplateAttributeInput,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt_del_wid = txn.prepare_cached("delete from collection_template_attribute_workflows where metadata_id = $1 and version = $2 and key = $3").await?;
        txn.execute(&stmt_del_wid, &[metadata_id, &version, &attr.key])
            .await?;
        let stmt = txn.prepare_cached("insert into collection_template_attributes (metadata_id, version, key, name, description, configuration, type, ui, list, sort, supplementary_key) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) on conflict (metadata_id, version, key) do update set name = $4, description = $5, configuration = $6, type = $7, ui = $8, list = $8, sort = $9, supplementary_key = $10").await?;
        let stmt_wid = txn.prepare_cached("insert into collection_template_attribute_workflows (metadata_id, version, key, workflow_id, auto_run) values ($1, $2, $3, $4, $5)").await?;
        txn.execute(
            &stmt,
            &[
                metadata_id,
                &version,
                &attr.key,
                &attr.name,
                &attr.description,
                &attr.configuration,
                &attr.attribute_type,
                &attr.ui,
                &attr.list,
                &sort,
                &attr.supplementary_key,
            ],
        )
        .await?;
        for wid in &attr.workflows {
            txn.execute(
                &stmt_wid,
                &[
                    metadata_id,
                    &version,
                    &attr.key,
                    &wid.workflow_id,
                    &wid.auto_run,
                ],
            )
            .await?;
        }
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
        let stmt_del_wid = txn.prepare_cached("delete from collection_template_attributes where metadata_id = $1 and version = $2 and key = $3").await?;
        let key = key.to_string();
        txn.execute(&stmt_del_wid, &[metadata_id, &version, &key])
            .await?;
        txn.commit().await?;
        Ok(())
    }
}
