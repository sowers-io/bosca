use crate::context::BoscaContext;
use crate::datastores::content::tag::update_metadata_etag;
use crate::datastores::notifier::Notifier;
use crate::models::content::document::{Document, DocumentInput};
use crate::models::content::document_template::{DocumentTemplate, DocumentTemplateInput};
use crate::models::content::document_template_container::DocumentTemplateContainer;
use crate::models::content::metadata::MetadataInput;
use crate::models::content::metadata_profile::MetadataProfileInput;
use crate::models::content::template_attribute::TemplateAttribute;
use crate::models::content::template_workflow::TemplateWorkflow;
use crate::models::security::permission::{Permission, PermissionAction};
use async_graphql::*;
use deadpool_postgres::{GenericClient, Pool, Transaction};
use log::error;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;
use crate::models::content::document_template_container_type::DocumentTemplateContainerType;

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

    pub async fn get_templates(&self) -> Result<Vec<DocumentTemplate>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from document_templates")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
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

    pub async fn get_template_attributes(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<Vec<TemplateAttribute>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from document_template_attributes where metadata_id = $1 and version = $2 order by sort asc").await?;
        let results = connection.query(&stmt, &[metadata_id, &version]).await?;
        Ok(results.iter().map(|r| r.into()).collect())
    }

    pub async fn get_template_containers(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<Vec<DocumentTemplateContainer>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from document_template_containers where metadata_id = $1 and version = $2 order by sort asc").await?;
        let results = connection.query(&stmt, &[metadata_id, &version]).await?;
        Ok(results.iter().map(|r| r.into()).collect())
    }

    pub async fn get_container_template_workflows(
        &self,
        metadata_id: &Uuid,
        version: i32,
        id: &String,
    ) -> Result<Vec<TemplateWorkflow>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from document_template_container_workflows where metadata_id = $1 and version = $2 and id = $3").await?;
        let results = connection
            .query(&stmt, &[metadata_id, &version, id])
            .await?;
        Ok(results.iter().map(|r| r.into()).collect())
    }

    pub async fn get_template_attribute_workflows(
        &self,
        metadata_id: &Uuid,
        version: i32,
        key: &String,
    ) -> Result<Vec<TemplateWorkflow>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from document_template_attribute_workflows where metadata_id = $1 and version = $2 and key = $3").await?;
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
        template: &DocumentTemplateInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into document_templates (metadata_id, version, configuration, schema, default_attributes, content) values ($1, $2, $3, $4, $5, $6)").await?;
        txn.execute(
            &stmt,
            &[
                metadata_id,
                &version,
                &template.configuration,
                &template.schema,
                &template.default_attributes,
                &template.content,
            ],
        )
        .await?;
        self.add_template_items_txn(txn, metadata_id, version, template)
            .await?;
        Ok(())
    }

    pub async fn edit_template_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        template: &DocumentTemplateInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into document_templates (metadata_id, version, configuration, schema, default_attributes, content) values ($1, $2, $3, $4, $5, $6) on conflict (metadata_id, version) do update set configuration = $3, schema = $4, default_attributes = $5, content = $6").await?;
        txn.execute(
            &stmt,
            &[
                &metadata_id,
                &version,
                &template.configuration,
                &template.schema,
                &template.default_attributes,
                &template.content,
            ],
        )
        .await?;
        txn.execute(
            "delete from document_template_attributes where metadata_id = $1 and version = $2",
            &[metadata_id, &version],
        )
        .await?;
        txn.execute(
            "delete from document_template_containers where metadata_id = $1 and version = $2",
            &[metadata_id, &version],
        )
        .await?;
        txn.execute(
            "delete from document_template_attribute_workflows where metadata_id = $1 and version = $2",
            &[metadata_id, &version],
        )
        .await?;
        self.add_template_items_txn(txn, metadata_id, version, template)
            .await?;
        Ok(())
    }

    async fn add_template_items_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        template: &DocumentTemplateInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into document_template_attributes (metadata_id, version, key, name, description, configuration, type, ui, list, sort, supplementary_key) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)").await?;
        let stmt_wid = txn.prepare_cached("insert into document_template_attribute_workflows (metadata_id, version, key, workflow_id, auto_run) values ($1, $2, $3, $4, $5)").await?;
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
        if let Some(containers) = &template.containers {
            let stmt = txn.prepare_cached("insert into document_template_containers (metadata_id, version, id, name, description, supplementary_key, type, sort) values ($1, $2, $3, $4, $5, $6, $7, $8)").await?;
            let stmt_wid = txn.prepare_cached("insert into document_template_container_workflows (metadata_id, version, id, workflow_id, auto_run) values ($1, $2, $3, $4, $5)").await?;
            for (index, container) in containers.iter().enumerate() {
                let sort = index as i32;
                let ct = container.container_type.unwrap_or(DocumentTemplateContainerType::Standard);
                txn.execute(
                    &stmt,
                    &[
                        metadata_id,
                        &version,
                        &container.id,
                        &container.name,
                        &container.description,
                        &container.supplementary_key,
                        &ct,
                        &sort,
                    ],
                )
                .await?;
                for wid in &container.workflows {
                    txn.execute(
                        &stmt_wid,
                        &[
                            metadata_id,
                            &version,
                            &container.id,
                            &wid.workflow_id,
                            &wid.auto_run,
                        ],
                    )
                    .await?;
                }
            }
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

    pub async fn add_document_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        document: &DocumentInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into documents (metadata_id, version, template_metadata_id, template_metadata_version, title, content) values ($1, $2, $3, $4, $5, $6)").await?;
        let template_metadata_id = document
            .template_metadata_id
            .as_ref()
            .map(|id| Uuid::parse_str(id.as_str()).unwrap());
        txn.execute(
            &stmt,
            &[
                metadata_id,
                &version,
                &template_metadata_id,
                &document.template_metadata_version,
                &document.title,
                &document.content,
            ],
        )
        .await?;
        Ok(())
    }

    pub async fn set_document(
        &self,
        metadata_id: &Uuid,
        version: i32,
        document: &DocumentInput,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("insert into documents (metadata_id, version, template_metadata_id, template_metadata_version, title, content) values ($1, $2, $3, $4, $5, $6) on conflict (metadata_id, version) do update set template_metadata_id = $3, template_metadata_version = $4, title = $5, content = $6").await?;
        let template_metadata_id = document
            .template_metadata_id
            .as_ref()
            .map(|id| Uuid::parse_str(id.as_str()).unwrap());
        txn.execute(
            &stmt,
            &[
                metadata_id,
                &version,
                &template_metadata_id,
                &document.template_metadata_version,
                &document.title,
                &document.content,
            ],
        )
        .await?;
        let stmt = txn
            .prepare_cached("update metadata set modified = now() where id = $1")
            .await?;
        txn.execute(&stmt, &[metadata_id]).await?;
        update_metadata_etag(&txn, metadata_id).await?;
        txn.execute(&stmt, &[metadata_id]).await?;
        txn.commit().await?;
        self.on_metadata_changed(metadata_id).await?;
        Ok(())
    }

    pub async fn edit_document_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        document: &DocumentInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into documents (metadata_id, version, template_metadata_id, template_metadata_version, title, content) values ($1, $2, $3, $4, $5, $6) on conflict (metadata_id, version) do update set template_metadata_id = $3, template_metadata_version = $4, title = $5, content = $6").await?;
        let template_metadata_id = document
            .template_metadata_id
            .as_ref()
            .map(|id| Uuid::parse_str(id.as_str()).unwrap());
        txn.execute(
            &stmt,
            &[
                metadata_id,
                &version,
                &template_metadata_id,
                &document.template_metadata_version,
                &document.title,
                &document.content,
            ],
        )
        .await?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn add_document_from_template(
        &self,
        ctx: &BoscaContext,
        parent_collection_id: Option<Uuid>,
        template_id: &Uuid,
        template_version: i32,
        title: &str,
        content_type: &str,
        permissions: &[Permission],
    ) -> Result<(Uuid, i32), Error> {
        let mut conn = self.pool.get().await?;
        let txn = conn.transaction().await?;
        let (id, version) = self
            .add_document_from_template_txn(
                ctx,
                &txn,
                parent_collection_id,
                title,
                template_id,
                template_version,
                content_type,
                permissions,
            )
            .await?;
        txn.commit().await?;
        Ok((id, version))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn add_document_from_template_txn(
        &self,
        ctx: &BoscaContext,
        txn: &Transaction<'_>,
        parent_collection_id: Option<Uuid>,
        title: &str,
        template_id: &Uuid,
        template_version: i32,
        content_type: &str,
        permissions: &[Permission],
    ) -> Result<(Uuid, i32), Error> {
        let template = ctx
            .check_metadata_version_action(template_id, template_version, PermissionAction::View)
            .await?;
        if template.content_type != "bosca/v-document-template" {
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
        let editor_type = template
            .attributes
            .get("editor.type")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let mut attrs = json!({
            "editor.type": editor_type.to_string(),
        });
        if let Some(default_attributes) = &template_document.default_attributes {
            if let serde_json::Value::Object(ref mut attrs_obj) = attrs {
                if let serde_json::Value::Object(default_obj) = default_attributes.clone() {
                    attrs_obj.extend(default_obj.into_iter());
                }
            }
        }
        let profile = ctx.profile.get_by_principal(&ctx.principal.id).await?;
        let categories = ctx.content.metadata.get_categories(&template.id).await?;
        let metadata = MetadataInput {
            parent_collection_id: parent_collection_id.map(|p| p.to_string()),
            category_ids: Some(categories.iter().map(|c| c.id.to_string()).collect()),
            name: title.to_string(),
            content_type: content_type.to_string(),
            language_tag: template.language_tag,
            attributes: Some(attrs),
            document: Some(DocumentInput {
                template_metadata_id: Some(template.id.to_string()),
                template_metadata_version: Some(template.version),
                title: title.to_string(),
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
        let (id, version, _) = ctx
            .content
            .metadata
            .add_txn(ctx, txn, &metadata, true, &None)
            .await?;
        ctx.content.metadata_permissions.add_metadata_permissions_txn(txn, &id, permissions).await?;
        Ok((id, version))
    }
}
