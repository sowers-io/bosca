use crate::context::BoscaContext;
use crate::graphql::profiles::profile::ProfileObject;
use crate::models::content::comment::Comment;
use crate::models::content::comment_status::CommentStatus;
use async_graphql::{Context, Error, Object};

pub struct CommentsObject {
    pub manager: bool,
    pub comments: Vec<Comment>,
    pub count: i64,
}

impl CommentsObject {
    pub fn new(manager: bool, comments: Vec<Comment>, count: i64) -> Self {
        Self {
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
            .map(|c| CommentObject::new(self.manager, c))
            .collect()
    }
    pub async fn count(&self) -> &i64 {
        &self.count
    }
}

pub struct CommentObject {
    pub manager: bool,
    pub comment: Comment,
}

impl CommentObject {
    pub fn new(manager: bool, comment: Comment) -> Self {
        Self { manager, comment }
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
}
