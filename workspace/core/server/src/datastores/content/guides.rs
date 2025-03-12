use crate::models::content::guide::{Guide, GuideInput};
use crate::models::content::guide_step::{GuideStep, GuideStepInput};
use crate::models::content::guide_step_module::{GuideStepModule, GuideStepModuleInput};
use crate::models::content::guide_template::{GuideTemplate, GuideTemplateInput};
use crate::models::content::guide_template_step::GuideTemplateStep;
use crate::models::content::guide_template_step_module::GuideTemplateStepModule;
use async_graphql::*;
use deadpool_postgres::{GenericClient, Pool, Transaction};
use rrule::RRuleSet;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GuidesDataStore {
    pool: Arc<Pool>,
}

impl GuidesDataStore {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
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

    pub async fn get_template_step(
        &self,
        metadata_id: &Uuid,
        version: i32,
        id: i64,
    ) -> Result<Option<GuideTemplateStep>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from guide_template_steps where metadata_id = $1 and version = $2 and id = $3")
            .await?;
        let rows = connection
            .query(&stmt, &[metadata_id, &version, &id])
            .await?;
        Ok(rows.first().map(|r| r.into()))
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
        let rows = connection
            .query(&stmt, &[metadata_id, &version, &step_id])
            .await?;
        Ok(rows.iter().map(|r| r.into()).collect())
    }

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

    pub async fn add_template_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        template: &GuideTemplateInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into guide_templates (metadata_id, version, rrule, type) values ($1, $2, $3, $4)").await?;
        txn.execute(
            &stmt,
            &[metadata_id, &version, &template.rrule, &template.guide_type],
        )
        .await?;
        self.add_template_steps_txn(txn, metadata_id, version, template)
            .await?;
        Ok(())
    }

    pub async fn edit_template_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        template: &GuideTemplateInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into guide_templates (metadata_id, version, rrule, type) values ($1, $2, $3, $4) on conflict (metadata_id, version) do update set rrule = $3, type = $4").await?;
        txn.execute(
            &stmt,
            &[
                &metadata_id,
                &version,
                &template.rrule,
                &template.guide_type,
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
            let template_metadata_id = step
                .template_metadata_id
                .as_ref()
                .map(|id| Uuid::parse_str(id.as_str()).unwrap());
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

    pub async fn add_guide_txn(
        &self,
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
            self.add_guide_step_txn(txn, metadata_id, version, index as i32, step)
                .await?;
        }
        Ok(())
    }

    pub async fn edit_guide(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        guide: &GuideInput,
    ) -> Result<(), Error> {
        let stmt = txn.prepare_cached("insert into guides (metadata_id, version, template_metadata_id, template_metadata_version, guide_metadata_id, guide_metadata_version, rrule, type) values ($1, $2, $3, $4, $5, $6, $7, $8) on conflict (metadata_id, version) do update set template_metadata_id = $3, template_metadata_version = $4, rrule = $5, type = $6, guide_metadata_id = $7, guide_metadata_version = $8").await?;
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
        self.add_guide_txn(txn, metadata_id, version, guide).await?;
        Ok(())
    }

    pub async fn add_guide_step(
        &self,
        metadata_id: &Uuid,
        version: i32,
        step: &GuideStepInput,
        sort: i32,
    ) -> Result<GuideStep, Error> {
        let mut conn = self.pool.get().await?;
        let txn = conn.transaction().await?;
        let step = self
            .add_guide_step_txn(&txn, metadata_id, version, sort, step)
            .await?;
        txn.commit().await?;
        Ok(step)
    }

    async fn add_guide_step_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        index: i32,
        step: &GuideStepInput,
    ) -> Result<GuideStep, Error> {
        let stmt = txn.prepare_cached("insert into guide_steps (metadata_id, version, step_metadata_id, step_metadata_version, sort) values ($1, $2, $3, $4, $5) returning id").await?;
        let step_metadata_id = step
            .step_metadata_id
            .as_ref()
            .map(|id| Uuid::parse_str(id.as_str()).unwrap());
        let sort = index;
        let result = txn
            .query_one(
                &stmt,
                &[
                    metadata_id,
                    &version,
                    &step_metadata_id,
                    &step.step_metadata_version,
                    &sort,
                ],
            )
            .await?;
        let step_id: i64 = result.get("id");
        for (index, module) in step.modules.iter().enumerate() {
            let sort = index as i32;
            self.add_guide_step_module_txn(txn, metadata_id, version, step_id, sort, module)
                .await?;
        }
        Ok(GuideStep {
            id: step_id,
            step_metadata_id,
            step_metadata_version: step.step_metadata_version,
            metadata_id: *metadata_id,
            metadata_version: version,
        })
    }

    pub async fn add_guide_step_module(
        &self,
        metadata_id: &Uuid,
        version: i32,
        step_id: i64,
        index: i32,
        module: &GuideStepModuleInput,
    ) -> Result<GuideStepModule, Error> {
        let mut conn = self.pool.get().await?;
        let txn = conn.transaction().await?;
        let module = self
            .add_guide_step_module_txn(&txn, metadata_id, version, step_id, index, module)
            .await?;
        txn.commit().await?;
        Ok(module)
    }


    async fn add_guide_step_module_txn(
        &self,
        txn: &Transaction<'_>,
        metadata_id: &Uuid,
        version: i32,
        step_id: i64,
        index: i32,
        module: &GuideStepModuleInput,
    ) -> Result<GuideStepModule, Error> {
        let stmt_module = txn.prepare_cached("insert into guide_step_modules (metadata_id, version, step, module_metadata_id, module_metadata_version, sort) values ($1, $2, $3, $4, $5, $6) returning id").await?;
        let module_metadata_id = Uuid::parse_str(&module.module_metadata_id)?;
        let result = txn
            .query_one(
                &stmt_module,
                &[
                    metadata_id,
                    &version,
                    &step_id,
                    &module_metadata_id,
                    &module.module_metadata_version,
                    &index,
                ],
            )
            .await?;
        let module_id: i64 = result.get("id");
        Ok(GuideStepModule {
            id: module_id,
            module_metadata_id,
            module_metadata_version: version,
        })
    }
}
