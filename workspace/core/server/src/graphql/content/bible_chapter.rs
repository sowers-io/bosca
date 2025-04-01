use crate::models::bible::chapter::Chapter;
use async_graphql::Object;
use serde_json::{json, Value};

pub struct BibleChapterObject {
    chapter: Chapter,
}

impl BibleChapterObject {
    pub fn new(chapter: Chapter) -> Self {
        Self { chapter }
    }
}

#[Object(name = "BibleChapter")]
impl BibleChapterObject {

    async fn usfm(&self) -> &String {
        self.chapter.reference.usfm()
    }

    async fn component(&self) -> Value {
        json!(self.chapter.component)
    }
}
