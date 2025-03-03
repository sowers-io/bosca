use crate::datastores::notifier::Notifier;
use crate::models::content::guide::{Guide, GuideInput};
use crate::models::content::guide_step::GuideStep;
use crate::models::content::guide_template::{GuideTemplate, GuideTemplateInput};
use crate::models::content::template_attribute::TemplateAttribute;
use crate::models::content::template_attribute_workflow::TemplateAttributeWorkflow;
use async_graphql::*;
use deadpool_postgres::{GenericClient, Pool, Transaction};
use log::error;
use rrule::RRuleSet;
use std::sync::Arc;
use uuid::Uuid;
use crate::models::content::guide_step_module::GuideStepModule;
use crate::models::content::guide_template_step::GuideTemplateStep;
use crate::models::content::guide_template_step_module::GuideTemplateStepModule;

#[derive(Clone)]
pub struct GuidesDataStore {
    pool: Arc<Pool>,
    notifier: Arc<Notifier>,
}

impl GuidesDataStore {
    pub fn new(pool: Arc<Pool>, notifier: Arc<Notifier>) -> Self {
        Self { pool, notifier }
    }

    async fn on_metadata_changed(&self, id: &Uuid) -> Result<(), Error> {
        if let Err(e) = self.notifier.metadata_changed(id).await {
            error!("Failed to notify metadata changes: {:?}", e);
        }
        Ok(())
    }

    pub async fn get_templates(&self) -> Result<Vec<GuideTemplate>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from guide_templates")
            .await?;
        let rows = connection.query(&stmt, &[]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

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

    pub async fn get_template_attributes(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<Vec<TemplateAttribute>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from guide_template_attributes where metadata_id = $1 and version = $2 order by sort asc").await?;
        let results = connection.query(&stmt, &[metadata_id, &version]).await?;
        Ok(results.iter().map(|r| r.into()).collect())
    }

    pub async fn get_template_attribute_workflows(
        &self,
        metadata_id: &Uuid,
        version: i32,
        key: &String,
    ) -> Result<Vec<TemplateAttributeWorkflow>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from guide_template_attribute_workflow_ids where metadata_id = $1 and version = $2 and key = $3").await?;
        let results = connection
            .query(&stmt, &[metadata_id, &version, key])
            .await?;
        Ok(results.iter().map(|r| r.into()).collect())
    }

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

    pub async fn get_template_step_attributes(
        &self,
        metadata_id: &Uuid,
        version: i32,
        step_id: i64,
    ) -> Result<Vec<TemplateAttribute>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from guide_template_step_attributes where metadata_id = $1 and version = $2 and step = $3 order by sort asc")
            .await?;
        let rows = connection.query(&stmt, &[metadata_id, &version, &step_id]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn get_template_step_attribute_workflows(
        &self,
        metadata_id: &Uuid,
        version: i32,
        step_id: i64,
        key: &str,
    ) -> Result<Vec<TemplateAttributeWorkflow>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection.prepare_cached("select * from guide_template_step_attribute_workflow_ids where metadata_id = $1 and version = $2 and step = $3 and key = $4").await?;
        let key = key.to_string();
        let results = connection
            .query(&stmt, &[metadata_id, &version, &step_id, &key])
            .await?;
        Ok(results.iter().map(|r| r.into()).collect())
    }

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
        let rows = connection.query(&stmt, &[metadata_id, &version, &step_id]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    pub async fn add_template(
        &self,
        metadata_id: &Uuid,
        version: i32,
        template: &GuideTemplateInput,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("insert into guide_templates (metadata_id, version, rrule, type, default_attributes) values ($1, $2, $3, $4, $5)").await?;
        txn.execute(
            &stmt,
            &[
                metadata_id,
                &version,
                &template.rrule,
                &template.guide_type,
                &template.default_attributes,
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
        template: &GuideTemplateInput,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("insert into guide_templates (metadata_id, version, rrule, type, default_attributes) values ($1, $2, $3, $4, $5) on conflict (metadata_id, version) do update set rrule = $3, type = $4, default_attributes = $5").await?;
        txn.execute(
            &stmt,
            &[
                &template.rrule,
                &template.guide_type,
                &template.default_attributes,
                &metadata_id,
                &version,
            ],
        )
        .await?;
        txn.execute(
            "delete from guide_template_steps where metadata_id = $1 and version = $2",
            &[metadata_id, &version],
        )
        .await?;
        txn.execute(
            "delete from guide_template_attributes where metadata_id = $1 and version = $2",
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
        template: &GuideTemplateInput,
    ) -> Result<(), Error> {
        let stmt_attrs = txn.prepare_cached("insert into guide_template_attributes (metadata_id, version, key, name, description, configuration, type, ui, list, sort, supplementary_key) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)").await?;
        let stmt_attr_wid = txn.prepare_cached("insert into guide_template_attribute_workflow_ids (metadata_id, version, key, workflow_id, auto_run) values ($1, $2, $3, $4, $5)").await?;
        for (index, attr) in template.attributes.iter().enumerate() {
            let sort = index as i32;
            txn.execute(
                &stmt_attrs,
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
            for wid in &attr.workflow_ids {
                txn.execute(
                    &stmt_attr_wid,
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
        let stmt_steps = txn.prepare_cached("insert into guide_template_steps (metadata_id, version, name, description, configuration, sort) values ($1, $2, $3, $4, $5, $6) returning id").await?;
        let stmt_step_attrs = txn.prepare_cached("insert into guide_template_step_attributes (metadata_id, version, step, key, name, description, configuration, type, ui, list, sort, supplementary_key) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)").await?;
        let stmt_step_attr_wid = txn.prepare_cached("insert into guide_template_step_attribute_workflow_ids (metadata_id, version, step, key, workflow_id, auto_run) values ($1, $2, $3, $4, $5, $6)").await?;
        let stmt_step_modules = txn.prepare_cached("insert into guide_template_step_modules (metadata_id, version, step, template_metadata_id, template_metadata_version, sort) values ($1, $2, $3, $4, $5, $6)").await?;
        for (index, step) in template.steps.iter().enumerate() {
            let sort = index as i32;
            let result = txn
                .query_one(
                    &stmt_steps,
                    &[
                        metadata_id,
                        &version,
                        &step.name,
                        &step.description,
                        &step.configuration,
                        &sort,
                    ],
                )
                .await?;
            let step_id: i64 = result.get("id");
            for (index, attr) in step.attributes.iter().enumerate() {
                let sort = index as i32;
                txn.execute(
                    &stmt_step_attrs,
                    &[
                        metadata_id,
                        &version,
                        &step_id,
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
                for wid in &attr.workflow_ids {
                    txn.execute(
                        &stmt_step_attr_wid,
                        &[
                            metadata_id,
                            &version,
                            &step_id,
                            &attr.key,
                            &wid.workflow_id,
                            &wid.auto_run,
                        ],
                    )
                    .await?;
                }
            }
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

    pub async fn get_guide_steps(
        &self,
        metadata_id: &Uuid,
        version: i32,
    ) -> Result<Vec<GuideStep>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from guide_steps where metadata_id = $1 and version = $2 order by sort asc")
            .await?;
        let rows = connection.query(&stmt, &[metadata_id, &version]).await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

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

    pub async fn add_guide(
        &self,
        metadata_id: &Uuid,
        version: i32,
        guide: &GuideInput,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("insert into guides (metadata_id, version, template_metadata_id, template_metadata_version, rrule, type) values ($1, $2, $3, $4, $5, $6)").await?;
        let template_metadata_id = guide
            .template_metadata_id
            .as_ref()
            .map(|id| Uuid::parse_str(id.as_str()).unwrap());
        let rrule: Option<RRuleSet> = guide.rrule.as_ref().map(|r| r.parse().unwrap());
        let rrule = rrule.map(|r| r.to_string()).unwrap_or_else(|| "".to_string());
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
        self.edit_guide_txn(&txn, metadata_id, version, guide).await?;
        txn.commit().await?;
        self.on_metadata_changed(metadata_id).await?;
        Ok(())
    }

    pub async fn edit_guide(
        &self,
        metadata_id: &Uuid,
        version: i32,
        guide: &GuideInput,
    ) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn.prepare_cached("insert into guides (metadata_id, version, template_metadata_id, template_metadata_version, rrule, type) values ($1, $2, $3, $4, $5, $6) on conflict (metadata_id, version) do update set template_metadata_id = $3, template_metadata_version = $4, rrule = $5, type = $6").await?;
        let template_metadata_id = guide
            .template_metadata_id
            .as_ref()
            .map(|id| Uuid::parse_str(id.as_str()).unwrap());
        let rrule: Option<RRuleSet> = guide.rrule.as_ref().map(|r| r.parse().unwrap());
        let rrule = rrule.map(|r| r.to_string()).unwrap_or_else(|| "".to_string());
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
        self.edit_guide_txn(&txn, metadata_id, version, guide).await?;
        txn.commit().await?;
        self.on_metadata_changed(metadata_id).await?;
        Ok(())
    }

    async fn edit_guide_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        guide: &GuideInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into guide_steps (metadata_id, version, template_metadata_id, template_metadata_version, template_step, sort) values ($1, $2, $3, $4, $5, $6) returning id").await?;
        let stmt_module = txn.prepare_cached("insert into guide_step_modules (metadata_id, version, step, template_metadata_id, template_metadata_version, template_step, template_module, sort) values ($1, $2, $3, $4, $5, $6, $7, $8) returning id").await?;
        for (index, step) in guide.steps.iter().enumerate() {
            let template_metadata_id = guide
                .template_metadata_id
                .as_ref()
                .map(|id| Uuid::parse_str(id.as_str()).unwrap());
            let sort = index as i32;
            let result = txn
                .query_one(
                    &stmt,
                    &[
                        metadata_id,
                        &version,
                        &template_metadata_id,
                        &step.template_metadata_version,
                        &step.template_step_id,
                        &sort,
                    ],
                )
                .await?;
            let step_id: i64 = result.get("id");
            for (index, module) in step.modules.iter().enumerate() {
                let sort = index as i32;
                let template_id = module
                    .template_metadata_id
                    .as_ref()
                    .map(|id| Uuid::parse_str(id.as_str()).unwrap());
                txn.execute(
                    &stmt_module,
                    &[
                        metadata_id,
                        &version,
                        &step_id,
                        &template_id,
                        &module.template_metadata_version,
                        &module.template_step_id,
                        &module.template_module_id,
                        &sort,
                    ],
                )
                .await?;
            }
        }
        Ok(())
    }
}
