use crate::datastores::notifier::Notifier;
use crate::models::content::document_template::{DocumentTemplate, DocumentTemplateInput};
use async_graphql::*;
use deadpool_postgres::{GenericClient, Pool, Transaction};
use log::error;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct MetadataDocumentsDataStore {
    pool: Arc<Pool>,
    notifier: Arc<Notifier>,
}

impl MetadataDocumentsDataStore {
    pub fn new(pool: Arc<Pool>, notifier: Arc<Notifier>) -> Self {
        Self { pool, notifier }
    }

    async fn on_metadata_changed(&self, id: &Uuid) -> Result<(), Error> {
        if let Err(e) = self.notifier.metadata_changed(id).await {
            error!("Failed to notify metadata changes: {:?}", e);
        }
        Ok(())
    }

    pub async fn get_templates(&self, id: &Uuid) -> Result<Vec<DocumentTemplate>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from document_templates")
            .await?;
        let rows = connection.query(&stmt, &[id]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn add_template(&self, template: &DocumentTemplateInput) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("insert into document_templates (name, description, allow_user_defined_blocks) values ($1, $2, $3) returning id").await?;
        let result = txn.query(&stmt, &[&template.name, &template.description, &template.allow_user_defined_blocks]).await?;
        let template_id: i64 = result.first().unwrap().get("id");
        self.add_template_txn(&txn, template_id, template).await?;
        txn.commit().await?;
        Ok(())
    }

    pub async fn edit_template(&self, id: i64, template: &DocumentTemplateInput) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("update document_templates set name = $1, description = $2, allow_user_defined_blocks = $3, modified = now() where id = $4").await?;
        txn.execute(&stmt, &[&template.name, &template.description, &template.allow_user_defined_blocks, &id]).await?;
        txn.execute("delete from document_template_metadata_attribute_workflow_ids where template_id = $1", &[&id]).await?;
        txn.execute("delete from document_template_metadata_attributes where template_id = $1", &[&id]).await?;
        txn.execute("delete from document_template_blocks where template_id = $1", &[&id]).await?;
        self.add_template_txn(&txn, id, template).await?;
        txn.commit().await?;
        Ok(())
    }

    async fn add_template_txn(&self, txn: &Transaction<'_>, template_id: i64, template: &DocumentTemplateInput) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into document_template_categories (template_id, category_id) values ($1, $2)").await?;
        for category_id in &template.category_ids {
            let id = Uuid::parse_str(category_id.as_str())?;
            txn.execute(&stmt, &[&template_id, &id]).await?;
        }
        let stmt = txn.prepare_cached("insert into document_template_metadata_attributes (template_id, key, name, description, type) values ($1, $2, $3, $4, $5)").await?;
        let stmt_wid = txn.prepare_cached("insert into document_template_metadata_attribute_workflow_ids (template_id, key, workflow_id, auto_run) values ($1, $2, $3, $4)").await?;
        for attr in template.attributes.iter() {
            txn.execute(&stmt, &[&template_id, &attr.key, &attr.name, &attr.description, &attr.attribute_type]).await?;
            for wid in &attr.workflow_ids {
                txn.execute(&stmt_wid, &[&template_id, &attr.key, &wid.workflow_id, &wid.auto_run]).await?;
            }
        }
        let stmt = txn.prepare_cached("insert into document_template_blocks (template_id, name, description, type, sort) values ($1, $2, $3, $4, $5)").await?;
        for block in &template.blocks {
            txn.execute(&stmt, &[&template_id, &block.name, &block.description, &block.block_type, &block.sort]).await?;
        }
        Ok(())
    }
}
