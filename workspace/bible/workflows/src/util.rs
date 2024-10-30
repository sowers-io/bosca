use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use bosca_bible_processor::bible::Bible;
use bosca_bible_processor::process_path;
use bosca_client::client::find_collection::FindAttributeInput;
use bosca_client::client::{Client, WorkflowJob};
use bosca_client::download::download_path;
use bosca_workflows::activity::{ActivityContext, Error};

// TODO
#[allow(clippy::type_complexity)]
static BIBLE_CACHE: std::sync::LazyLock<Arc<Mutex<HashMap<String, Arc<Bible>>>>> = std::sync::LazyLock::new(|| Arc::new(Mutex::new(HashMap::<String, Arc<Bible>>::new())));

pub async fn get_bible(client: &Client, context: &mut ActivityContext, job: &WorkflowJob) -> Result<Arc<Bible>, Error> {
    let find_attributes = vec![
        FindAttributeInput {
            key: "bible.type".to_owned(),
            value: "bible".to_owned()
        },
        FindAttributeInput {
            key: "bible.system.id".to_owned(),
            value: job.metadata.as_ref().unwrap().attributes.get("bible.system.id").unwrap().as_str().unwrap().to_owned()
        }
    ];
    let collections = client.find_collection(find_attributes).await?;
    if collections.is_empty() {
        return Err(Error::new("couldn't find bible".to_owned()));
    }
    let bible_collection = collections.first().unwrap();
    let bible_metadata_id = bible_collection.attributes.get("bible.raw.id").unwrap().as_str().unwrap();
    if let Some(bible) = BIBLE_CACHE.lock().await.get(bible_metadata_id) {
        return Ok(Arc::clone(bible))
    }
    let metadata_id = &job.metadata.as_ref().unwrap().id;
    let download_url = client.get_metadata_download_url(bible_metadata_id).await?;
    let path_id = format!("{}{}", metadata_id, bible_metadata_id);
    let download = download_path(&path_id, &download_url).await?;
    context.add_file_clean(download.clone());
    let bible = Arc::new(match process_path(download.as_str()) {
        Ok(bible) => bible,
        Err(e) => return Err(Error::new(e.to_string())),
    });
    BIBLE_CACHE.lock().await.insert(bible_metadata_id.to_owned(), Arc::clone(&bible));
    Ok(bible)
}