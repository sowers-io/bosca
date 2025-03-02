use crate::datastores::notifier::Notifier;
use async_graphql::*;
use deadpool_postgres::{GenericClient, Pool};
use std::sync::Arc;
use log::error;
use uuid::Uuid;
use crate::models::workflow::workflow_schedule::{WorkflowSchedule, WorkflowScheduleInput};

#[derive(Clone)]
pub struct WorkflowScheduleDataStore {
    pool: Arc<Pool>,
    notifier: Arc<Notifier>,
}

impl WorkflowScheduleDataStore {
    pub fn new(pool: Arc<Pool>, notifier: Arc<Notifier>) -> Self {
        Self { pool, notifier }
    }

    async fn on_schedule_changed(&self, id: &Uuid) -> Result<(), Error> {
        if let Err(e) = self.notifier.workflow_schedule_changed(id).await {
            error!("Failed to notify workflow schedule changes: {:?}", e);
        }
        Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<WorkflowSchedule>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_schedules")
            .await?;
        let result = connection.query(&stmt, &[]).await?;
        Ok(result.iter().map(|c| c.into()).collect())
    }

    pub async fn get(&self, id: &Uuid) -> Result<Option<WorkflowSchedule>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from workflow_schedules where id = $1")
            .await?;
        let result = connection.query(&stmt, &[id]).await?;
        Ok(result.first().map(|c| c.into()))
    }

    pub async fn add(&self, metadata_id: Option<Uuid>, collection_id: Option<Uuid>, schedule: &WorkflowScheduleInput) -> Result<Uuid, Error> {
        let schedule = schedule.create_schedule(metadata_id, collection_id)?;
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("insert into workflow_schedules (metadata_id, collection_id, workflow_id, attributes, configuration, rrule, starts, ends, enabled) values ($1, $2, $3, $4, $5, $6, $7, $8, $9) returning id")
            .await?;
        let rrule = schedule.rrule.to_string();
        let result = txn.query(&stmt, &[&schedule.metadata_id, &schedule.collection_id, &schedule.workflow_id, &schedule.attributes, &schedule.configuration, &rrule, &schedule.starts, &schedule.ends, &schedule.enabled]).await?;
        let id = result.first().unwrap().get("id");
        txn.commit().await?;
        self.on_schedule_changed(&id).await?;
        Ok(id)
    }

    pub async fn delete(&self, id: &Uuid) -> Result<(), Error> {
        let mut connection = self.pool.get().await?;
        let txn = connection.transaction().await?;
        let stmt = txn
            .prepare_cached("delete from workflow_schedules where id = $1")
            .await?;
        txn.execute(&stmt, &[id]).await?;
        txn.commit().await?;
        self.on_schedule_changed(id).await?;
        Ok(())
    }
}
