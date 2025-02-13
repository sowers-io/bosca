use crate::datastores::notifier::Notifier;
use crate::models::content::document::{Document, DocumentInput};
use crate::models::content::document_block::DocumentBlock;
use crate::models::content::document_block_metadata::DocumentBlockMetadata;
use crate::models::content::document_template::{DocumentTemplate, DocumentTemplateInput};
use crate::models::content::document_template_attribute_workflow::DocumentTemplateAttributeWorkflow;
use crate::models::content::document_template_attributes::DocumentTemplateAttribute;
use crate::models::content::document_template_block::DocumentTemplateBlock;
use async_graphql::*;
use deadpool_postgres::{GenericClient, Pool, Transaction};
use log::error;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct DocumentsDataStore {
    pool: Arc<Pool>,
    notifier: Arc<Notifier>,
}

impl DocumentsDataStore {
    pub fn new(pool: Arc<Pool>, notifier: Arc<Notifier>) -> Self {
        Self { pool, notifier }
    }

    async fn on_metadata_changed(&self, id: &Uuid) -> Result<(), Error> {
        if let Err(e) = self.notifier.metadata_changed(id).await {
            error!("Failed to notify metadata changes: {:?}", e);
        }
        Ok(())
    }

    pub async fn get_template(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<Option<DocumentTemplate>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached(
                "select * from document_templates where metadata_id = $1 and version = $2",
            )
            .await?;
        let rows = connection.query(&stmt, &[metadata_id, &version]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    pub async fn get_template_blocks(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<Vec<DocumentTemplateBlock>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from document_template_blocks where metadata_id = $1 and version = $2 order by sort asc").await?;
        let results = connection.query(&stmt, &[metadata_id, &version]).await?;
        Ok(results.iter().map(|r| r.into()).collect())
    }

    pub async fn get_template_attributes(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<Vec<DocumentTemplateAttribute>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from document_template_metadata_attributes where metadata_id = $1 and version = $2").await?;
        let results = connection.query(&stmt, &[metadata_id, &version]).await?;
        Ok(results.iter().map(|r| r.into()).collect())
    }

    pub async fn get_template_attribute_workflows(
        &self,
        metadata_id: &Uuid,
        version: i32,
        key: &String,
    ) -> Result<Vec<DocumentTemplateAttributeWorkflow>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from document_template_metadata_attribute_workflow_ids where metadata_id = $1 and version = $2 and key = $3").await?;
        let results = connection
            .query(&stmt, &[metadata_id, &version, key])
            .await?;
        Ok(results.iter().map(|r| r.into()).collect())
    }

    pub async fn add_template(
        &self,
        metadata_id: &Uuid,
        version: i32,
        template: &DocumentTemplateInput,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("insert into document_templates (metadata_id, version, name, description, allow_user_defined_blocks) values ($1, $2, $3, $4, $5) returning id").await?;
        txn.execute(
            &stmt,
            &[
                metadata_id,
                &version,
                &template.name,
                &template.description,
                &template.allow_user_defined_blocks,
            ],
        )
        .await?;
        self.add_template_txn(&txn, metadata_id, version, template)
            .await?;
        txn.commit().await?;
        self.on_metadata_changed(metadata_id).await?;
        Ok(())
    }

    pub async fn edit_template(
        &self,
        metadata_id: &Uuid,
        version: i32,
        template: &DocumentTemplateInput,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("update document_templates set name = $1, description = $2, allow_user_defined_blocks = $3, modified = now() where metadata_id = $4 and version = $5").await?;
        txn.execute(
            &stmt,
            &[
                &template.name,
                &template.description,
                &template.allow_user_defined_blocks,
                &metadata_id,
                &version,
            ],
        )
        .await?;
        txn.execute(
            "delete from document_template_attribute_workflow_ids where metadata_id = $1 and version = $2",
            &[metadata_id, &version],
        )
        .await?;
        txn.execute(
            "delete from document_template_metadata_attributes where metadata_id = $1 and version = $2",
            &[metadata_id, &version],
        )
        .await?;
        txn.execute(
            "delete from document_template_blocks where metadata_id = $1 and version = $2",
            &[metadata_id, &version],
        )
        .await?;
        self.add_template_txn(&txn, metadata_id, version, template)
            .await?;
        txn.commit().await?;
        self.on_metadata_changed(metadata_id).await?;
        Ok(())
    }

    async fn add_template_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        template: &DocumentTemplateInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into document_template_metadata_attributes (metadata_id, version, key, name, description, type) values ($1, $2, $3, $4, $5, $6)").await?;
        let stmt_wid = txn.prepare_cached("insert into document_template_attribute_workflow_ids (metadata_id, version, key, workflow_id, auto_run) values ($1, $2, $3, $4, $5)").await?;
        for attr in template.attributes.iter() {
            txn.execute(
                &stmt,
                &[
                    metadata_id,
                    &version,
                    &attr.key,
                    &attr.name,
                    &attr.description,
                    &attr.attribute_type,
                ],
            )
            .await?;
            for wid in &attr.workflow_ids {
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
        let stmt = txn.prepare_cached("insert into document_template_blocks (metadata_id, version, name, description, type, validation, content, sort) values ($1, $2, $3, $4, $5, $6, $7, $8)").await?;
        for (index, block) in template.blocks.iter().enumerate() {
            let index = index as i32;
            txn.execute(
                &stmt,
                &[
                    metadata_id,
                    &version,
                    &block.name,
                    &block.description,
                    &block.block_type,
                    &block.validation,
                    &block.content,
                    &index,
                ],
            )
            .await?;
        }
        Ok(())
    }

    pub async fn get_document(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<Option<Document>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from documents where metadata_id = $1 and version = $2")
            .await?;
        let rows = connection.query(&stmt, &[metadata_id, &version]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    pub async fn get_blocks(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<Option<DocumentBlock>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from document_blocks where metadata_id = $1 and version = $2 order by sort asc")
            .await?;
        let rows = connection.query(&stmt, &[metadata_id, &version]).await?;
        Ok(rows.first().map(|r| r.into()))
    }

    pub async fn get_metadata(
        &self,
        metadata_id: &Uuid,
        version: i32,
        block_id: i64,
    ) -> Result<Vec<DocumentBlockMetadata>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from document_block_metadata where metadata_id = $1 and version = $2 and block_id = $3")
            .await?;
        let result = connection
            .query(&stmt, &[metadata_id, &version, &block_id])
            .await?;
        Ok(result.iter().map(|r| r.into()).collect())
    }

    pub async fn add_document(
        &self,
        metadata_id: &Uuid,
        version: i32,
        document: &DocumentInput,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("insert into documents (metadata_id, version, template_metadata_id, template_metadata_version, title, allow_user_defined_blocks) values ($1, $2, $3, $4, $5, $6)").await?;
        let template_metadata_id = document.template_metadata_id.as_ref().map(|id| Uuid::parse_str(id.as_str()).unwrap());
        txn.execute(
            &stmt,
            &[
                metadata_id,
                &version,
                &template_metadata_id,
                &document.template_metadata_version,
                &document.title,
                &document.allow_user_defined_blocks,
            ],
        )
        .await?;
        self.add_document_txn(&txn, metadata_id, version, document)
            .await?;
        txn.commit().await?;
        self.on_metadata_changed(metadata_id).await?;
        Ok(())
    }

    pub async fn edit_document(
        &self,
        metadata_id: &Uuid,
        version: i32,
        document: &DocumentInput,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("insert into documents (metadata_id, version, template_metadata_id, template_metadata_version, title, allow_user_defined_blocks) values ($1, $2, $3, $4, $5, $6) on conflict (metadata_id, version) do update set template_metadata_id = $3, template_metadata_version = $4, title = $5, allow_user_defined_blocks = $6").await?;
        let template_metadata_id = document.template_metadata_id.as_ref().map(|id| Uuid::parse_str(id.as_str()).unwrap());
        txn.execute(
            &stmt,
            &[
                metadata_id,
                &version,
                &template_metadata_id,
                &document.template_metadata_version,
                &document.title,
                &document.allow_user_defined_blocks,
            ],
        )
        .await?;
        txn.execute(
            "delete from document_block_metadata where metadata_id = $1 and version = $2",
            &[metadata_id, &version],
        )
        .await?;
        txn.execute(
            "delete from document_blocks where metadata_id = $1 and version = $2",
            &[metadata_id, &version],
        )
        .await?;
        self.add_document_txn(&txn, metadata_id, version, document)
            .await?;
        txn.commit().await?;
        self.on_metadata_changed(metadata_id).await?;
        Ok(())
    }

    async fn add_document_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        document: &DocumentInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into document_blocks (metadata_id, version, type, sort, content) values ($1, $2, $3, $4, $5) returning id").await?;
        let stmt_wid = txn.prepare_cached("insert into document_block_metadata (metadata_id, version, block_id, metadata_reference_id, attributes, sort) values ($1, $2, $3, $4, $5, $6)").await?;
        for (index, block) in document.blocks.iter().enumerate() {
            let index = index as i32;
            let block_id_results = txn
                .query(
                    &stmt,
                    &[
                        metadata_id,
                        &version,
                        &block.block_type,
                        &index,
                        &block.content,
                    ],
                )
                .await?;
            let block_id: i64 = block_id_results.first().unwrap().get("id");
            for (ref_index, r) in block.references.iter().enumerate() {
                let ref_index = ref_index as i32;
                let mid = Uuid::parse_str(r.metadata_id.as_str())?;
                txn.execute(
                    &stmt_wid,
                    &[
                        &metadata_id,
                        &version,
                        &block_id,
                        &mid,
                        &r.attributes,
                        &ref_index,
                    ],
                )
                .await?;
            }
        }
        Ok(())
    }
}
