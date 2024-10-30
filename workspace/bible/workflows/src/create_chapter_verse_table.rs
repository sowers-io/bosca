use async_trait::async_trait;
use bosca_client::client::{Client, WorkflowJob};
use bosca_workflows::activity::{Activity, ActivityContext, Error};

pub struct CreateChapterVerseTable {
    id: String,
}

impl Default for CreateChapterVerseTable {
    fn default() -> Self {
        Self::new()
    }
}

impl CreateChapterVerseTable {
    pub fn new() -> CreateChapterVerseTable {
        CreateChapterVerseTable {
            id: "bible.usx.chapter.ai".to_string(),
        }
    }
}

#[async_trait]
impl Activity for CreateChapterVerseTable {
    fn id(&self) -> &String {
        &self.id
    }

    async fn execute(
        &self,
        _client: &Client,
        _context: &mut ActivityContext,
        _job: &WorkflowJob,
    ) -> Result<(), Error> {
        Ok(())
    }
}
