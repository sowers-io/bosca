use async_graphql::*;
use deadpool_postgres::{GenericClient, Pool, Transaction};
use std::sync::Arc;
use uuid::Uuid;
use crate::models::content::collection_template::{CollectionTemplate, CollectionTemplateInput};
use crate::models::content::template_attribute::TemplateAttribute;
use crate::models::content::template_workflow::TemplateWorkflow;

#[derive(Clone)]
pub struct CollectionTemplatesDataStore {
    pool: Arc<Pool>,
}

impl CollectionTemplatesDataStore {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }

    pub async fn get_templates(&self) -> Result<Vec<CollectionTemplate>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select * from document_templates",
            )
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

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

    pub async fn add_template_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        template: &CollectionTemplateInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into collection_templates (metadata_id, version, default_attributes, configuration, collection_filter, metadata_filter) values ($1, $2, $3, $4, $5, $6)").await?;
        let collection_filter = template.collection_filter.as_ref().map(|f| serde_json::to_value(f).unwrap());
        let metadata_filter = template.metadata_filter.as_ref().map(|f| serde_json::to_value(f).unwrap());
        txn.execute(
            &stmt,
            &[
                metadata_id,
                &version,
                &template.default_attributes,
                &template.configuration,
                &collection_filter,
                &metadata_filter,
            ],
        )
        .await?;
        self.add_template_items_txn(&txn, metadata_id, version, template)
            .await?;
        Ok(())
    }

    pub async fn edit_template_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        template: &CollectionTemplateInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("update collection_templates set default_attributes = $1, configuration = $2, collection_filter = $3, metadata_filter = $4 where metadata_id = $5 and version = $6").await?;
        let collection_filter = template.collection_filter.as_ref().map(|f| serde_json::to_value(f).unwrap());
        let metadata_filter = template.metadata_filter.as_ref().map(|f| serde_json::to_value(f).unwrap());
        txn.execute(
            &stmt,
            &[
                &template.default_attributes,
                &template.configuration,
                &collection_filter,
                &metadata_filter,
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
        txn.execute(
            "delete from collection_template_attribute_workflows where metadata_id = $1 and version = $2",
            &[metadata_id, &version],
        )
        .await?;
        self.add_template_items_txn(&txn, metadata_id, version, template)
            .await?;
        Ok(())
    }

    async fn add_template_items_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        template: &CollectionTemplateInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into collection_template_attributes (metadata_id, version, key, name, description, configuration, type, ui, list, sort, supplementary_key) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)").await?;
        let stmt_wid = txn.prepare_cached("insert into collection_template_attribute_workflows (metadata_id, version, key, workflow_id, auto_run) values ($1, $2, $3, $4, $5)").await?;
        for (index, attr) in template.attributes.iter().enumerate() {
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
}
