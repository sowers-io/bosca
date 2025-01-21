use async_trait::async_trait;
use serde_json::Value;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::client::add_activity::ActivityInput;
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

    fn create_activity_input(&self) -> ActivityInput {
        ActivityInput {
            id: self.id.to_owned(),
            name: "Create Chapter Verse Table".to_string(),
            description: "Create a JSON table of Chapter Verses".to_string(),
            child_workflow_id: None,
            configuration: Value::Null,
            inputs: vec![],
            outputs: vec![],
        }
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
