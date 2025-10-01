use crate::context::BoscaContext;
use crate::graphql::profiles::profile::ProfileObject;
use crate::models::content::comment::Comment;
use crate::models::content::comment_status::CommentStatus;
use async_graphql::{Context, Error, Object};
use uuid::Uuid;

pub struct CommentsObject {
    pub metadata_id: Uuid,
    pub metadata_version: i32,
    pub manager: bool,
    pub comments: Vec<Comment>,
    pub count: i64,
}

impl CommentsObject {
    pub fn new(metadata_id: Uuid, metadata_version: i32, manager: bool, comments: Vec<Comment>, count: i64) -> Self {
        Self {
            metadata_id,
            metadata_version,
            manager,
            comments,
            count,
        }
    }
}

#[Object(name = "Comments")]
impl CommentsObject {
    pub async fn comments(&self) -> Vec<CommentObject> {
        self.comments
            .iter()
            .cloned()
            .map(|c| CommentObject::new(self.metadata_id, self.metadata_version, self.manager, c))
            .collect()
    }
    pub async fn count(&self) -> &i64 {
        &self.count
    }
}

pub struct CommentObject {
    pub metadata_id: Uuid,
    pub metadata_version: i32,
    pub manager: bool,
    pub comment: Comment,
}

impl CommentObject {
    pub fn new(metadata_id: Uuid, metadata_version: i32, manager: bool, comment: Comment) -> Self {
        Self { metadata_id, metadata_version, manager, comment }
    }
}

#[Object(name = "Comment")]
impl CommentObject {
    pub async fn id(&self) -> &i64 {
        &self.comment.id
    }
    pub async fn profile(&self, ctx: &Context<'_>) -> Result<Option<ProfileObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let profile = ctx.profile.get_by_id(&self.comment.profile_id).await?;
        Ok(profile.map(|p| ProfileObject::new(p)))
    }
    pub async fn created(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.comment.created
    }
    pub async fn modified(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.comment.modified
    }
    pub async fn attributes(&self) -> &Option<serde_json::Value> {
        &self.comment.attributes
    }
    pub async fn system_attributes(&self) -> &Option<serde_json::Value> {
        if self.manager {
            &self.comment.system_attributes
        } else {
            &None
        }
    }
    pub async fn status(&self) -> &CommentStatus {
        if self.manager {
            &self.comment.status
        } else {
            &CommentStatus::Approved
        }
    }
    pub async fn content(&self) -> &String {
        &self.comment.content
    }
    pub async fn likes(&self) -> &i32 {
        &self.comment.likes
    }
    pub async fn replies(&self, ctx: &Context<'_>, offset: i64, limit: i64) -> Result<CommentsObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let profile = ctx.profile.get_by_principal(&ctx.principal.id).await?;
        let profile_id = profile.map(|p| p.id);
        let comments = ctx
            .content
            .comments
            .get_metadata_comments_by_parent_id(
                &profile_id,
                &self.metadata_id,
                &self.metadata_version,
                &self.comment.id,
                self.manager,
                offset,
                limit,
            )
            .await?;
        let count = ctx
            .content
            .comments
            .get_metadata_comments_count_by_parent_id(
                &profile_id,
                &self.metadata_id,
                &self.metadata_version,
                &self.comment.id,
                self.manager,
            )
            .await?;
        Ok(CommentsObject::new(self.metadata_id, self.metadata_version, self.manager, comments, count))
    }
}
