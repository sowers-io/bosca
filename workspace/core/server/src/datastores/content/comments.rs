use crate::models::content::comment::{Comment, CommentInput};
use crate::models::content::comment_status::CommentStatus;
use async_graphql::Error;
use bosca_database::TracingPool;
use uuid::Uuid;
use crate::context::BoscaContext;
use crate::models::workflow::enqueue_request::EnqueueRequest;
use crate::workflow::core_workflow_ids::COMMENT_PROCESS;

#[derive(Clone)]
pub struct CommentsDataStore {
    pool: TracingPool,
}

impl CommentsDataStore {
    pub fn new(pool: TracingPool) -> Self {
        Self { pool }
    }

    #[tracing::instrument(skip(self, ctx, profile_id, impersonator_id, metadata_id, version, comment))]
    pub async fn add_metadata_comment(
        &self,
        ctx: &BoscaContext,
        profile_id: &Uuid,
        impersonator_id: Option<Uuid>,
        metadata_id: &Uuid,
        version: i32,
        comment: &CommentInput,
    ) -> Result<i64, Error> {
        let id = {
            let connection = self.pool.get().await?;
            let stmt = connection
                .prepare_cached("insert into metadata_comments (parent_id, metadata_id, version, profile_id, impersonator_id, visibility, content, attributes, system_attributes) values ($1, $2, $3, $4, $5, $6, $7, $8, $9) returning id")
                .await?;
            let rows = connection
                .query_one(
                    &stmt,
                    &[
                        &comment.parent_id,
                        metadata_id,
                        &version,
                        &profile_id,
                        &impersonator_id,
                        &comment.visibility,
                        &comment.content,
                        &comment.attributes,
                        &comment.system_attributes,
                    ],
                )
                .await?;
            rows.get(0)
        };

        let mut request = EnqueueRequest {
            workflow_id: Some(COMMENT_PROCESS.to_string()),
            metadata_id: Some(metadata_id.clone()),
            metadata_version: Some(version),
            comment_id: Some(id),
            ..Default::default()
        };
        ctx.workflow.enqueue_workflow(ctx, &mut request).await?;

        Ok(id)
    }

    #[tracing::instrument(skip(self, profile_id, metadata_id, version, id))]
    pub async fn get_metadata_comment(
        &self,
        profile_id: &Uuid,
        metadata_id: &Uuid,
        version: &i32,
        id: &i64,
        manager: bool,
    ) -> Result<Option<Comment>, Error> {
        let connection = self.pool.get().await?;
        let rows = if manager {
            let stmt = connection
                .prepare_cached("select * from metadata_comments where metadata_id = $1 and version = $2 and id = $3 and deleted = false")
                .await?;
            connection
                .query(&stmt, &[metadata_id, &version, &id])
                .await?
        } else {
            let stmt = connection
                .prepare_cached("select * from metadata_comments where metadata_id = $1 and version = $2 and id = $3 and deleted = false and ((visibility = 'public' and status = 'approved') or profile_id = $4)")
                .await?;
            connection
                .query(&stmt, &[metadata_id, version, id, profile_id])
                .await?
        };
        Ok(rows.first().map(|r| r.into()))
    }

    #[tracing::instrument(skip(self, id))]
    pub async fn get_metadata_comment_by_id(
        &self,
        id: &i64,
    ) -> Result<Option<Comment>, Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("select * from metadata_comments where id = $1")
            .await?;
        let rows = connection
            .query(&stmt, &[id])
            .await?;
        Ok(rows.first().map(|r| r.into()))
    }

    #[tracing::instrument(skip(self, profile_id, metadata_id, version, offset, limit))]
    pub async fn get_metadata_comments(
        &self,
        profile_id: &Option<Uuid>,
        metadata_id: &Uuid,
        version: &i32,
        manager: bool,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<Comment>, Error> {
        let connection = self.pool.get().await?;
        let rows = if manager {
            let stmt = connection
                .prepare_cached("select * from metadata_comments where metadata_id = $1 and version = $2 and deleted = false order by created desc offset $3 limit $4")
                .await?;
            connection
                .query(&stmt, &[metadata_id, &version, &offset, &limit])
                .await?
        } else if let Some(profile_id) = profile_id {
            let stmt = connection
                .prepare_cached("select * from metadata_comments where metadata_id = $1 and version = $2 and deleted = false and ((visibility = 'public' and status = 'approved') or (profile_id = $3)) order by created desc offset $4 limit $5")
                .await?;
            connection
                .query(&stmt, &[metadata_id, &version, profile_id, &offset, &limit])
                .await?
        } else {
            let stmt = connection
                .prepare_cached("select * from metadata_comments where metadata_id = $1 and version = $2 and deleted = false and (visibility = 'public' and status = 'approved') order by created desc offset $3 limit $4")
                .await?;
            connection
                .query(&stmt, &[metadata_id, &version, &offset, &limit])
                .await?
        };
        Ok(rows.iter().map(|r| r.into()).collect())
    }

    #[tracing::instrument(skip(self, profile_id, metadata_id, version))]
    pub async fn get_metadata_comments_count(
        &self,
        profile_id: &Option<Uuid>,
        metadata_id: &Uuid,
        version: &i32,
        manager: bool,
    ) -> Result<i64, Error> {
        let connection = self.pool.get().await?;
        let row = if manager {
            let stmt = connection
                .prepare_cached("select count(*) from metadata_comments where metadata_id = $1 and version = $2 and deleted = false")
                .await?;
            connection
                .query_one(&stmt, &[metadata_id, &version])
                .await?
        } else if let Some(profile_id) = profile_id {
            let stmt = connection
                .prepare_cached("select count(*) from metadata_comments where metadata_id = $1 and version = $2 and deleted = false and ((visibility = 'public' and status = 'approved') or (profile_id = $3)) order by created desc")
                .await?;
            connection
                .query_one(&stmt, &[metadata_id, &version, profile_id])
                .await?
        } else {
            let stmt = connection
                .prepare_cached("select count(*) from metadata_comments where metadata_id = $1 and version = $2 and deleted = false and (visibility = 'public' and status = 'approved')")
                .await?;
            connection
                .query_one(&stmt, &[metadata_id, &version])
                .await?
        };
        Ok(row.get(0))
    }

    #[tracing::instrument(skip(self, metadata_id, version, status))]
    pub async fn set_metadata_comment_status(
        &self,
        metadata_id: &Uuid,
        version: &i32,
        comment_id: i64,
        status: CommentStatus,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        if status == CommentStatus::Blocked {
            let stmt = connection
                .prepare_cached("update metadata_comments set status = 'blocked'::comment_status, deleted = true, modified = now() where metadata_id = $1 and version = $2 and id = $3")
                .await?;
            connection
                .query(&stmt, &[metadata_id, &version, &comment_id])
                .await?;
        } else {
            let stmt = connection
                .prepare_cached("update metadata_comments set status = $4, modified = now() where metadata_id = $1 and version = $2 and id = $3")
                .await?;
            connection
                .query(&stmt, &[metadata_id, &version, &comment_id, &status])
                .await?;
        }
        Ok(())
    }

    #[tracing::instrument(skip(self, metadata_id, version, attributes))]
    pub async fn set_metadata_comment_attributes(
        &self,
        metadata_id: &Uuid,
        version: &i32,
        comment_id: i64,
        attributes: &serde_json::Value,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update metadata_comments set attributes = $4, modified = now() where metadata_id = $1 and version = $2 and id = $3")
            .await?;
        connection
            .query(&stmt, &[metadata_id, &version, &comment_id, attributes])
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, metadata_id, version, attributes))]
    pub async fn set_metadata_comment_system_attributes(
        &self,
        metadata_id: &Uuid,
        version: &i32,
        comment_id: i64,
        attributes: &serde_json::Value,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update metadata_comments set system_attributes = $4, modified = now() where metadata_id = $1 and version = $2 and id = $3")
            .await?;
        connection
            .query(&stmt, &[metadata_id, &version, &comment_id, attributes])
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, metadata_id, version, attributes))]
    pub async fn merge_metadata_comment_system_attributes(
        &self,
        metadata_id: &Uuid,
        version: &i32,
        comment_id: i64,
        attributes: &serde_json::Value,
    ) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update metadata_comments set system_attributes = coalesce(system_attributes, '{}'::jsonb) || $4, modified = now() where metadata_id = $1 and version = $2 and id = $3")
            .await?;
        connection
            .query(&stmt, &[metadata_id, &version, &comment_id, attributes])
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, metadata_id, version))]
    pub async fn delete_metadata_comment(
        &self,
        metadata_id: &Uuid,
        version: &i32,
        comment_id: i64,
    ) -> Result<(), Error> {
        // TODO: log who deleted the comment
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update metadata_comments set deleted = true where metadata_id = $1 and version = $2 and id = $3")
            .await?;
        connection
            .query(&stmt, &[metadata_id, &version, &comment_id])
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip(self, metadata_id, version))]
    pub async fn delete_metadata_comment_by_profile_id(
        &self,
        metadata_id: &Uuid,
        version: &i32,
        comment_id: i64,
        profile_id: &Uuid,
    ) -> Result<(), Error> {
        // TODO: log who deleted the comment
        let connection = self.pool.get().await?;
        let stmt = connection
            .prepare_cached("update metadata_comments set deleted = true where metadata_id = $1 and version = $2 and id = $3 and profile_id = $4")
            .await?;
        connection
            .query(&stmt, &[metadata_id, &version, &comment_id, profile_id])
            .await?;
        Ok(())
    }
}
