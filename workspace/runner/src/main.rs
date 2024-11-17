use bosca_bible_workflows::create_chapter_verse_table::CreateChapterVerseTable;
use bosca_bible_workflows::process_bible::ProcessBibleActivity;
use bosca_workflows::activity::{Activity, Error};
use bosca_client::client::Client;
use bosca_workflows::{get_default_activities, process_queue};
use futures::future::join_all;
use log::{error, info, warn};
use std::collections::HashMap;
use std::env;
use std::process::exit;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;
#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};
#[cfg(windows)]
use tokio::signal::windows::ctrl_c;
use bosca_bible_workflows::upload_book::BookUpload;
use bosca_bible_workflows::upload_chapter::ChapterUpload;
use bosca_bible_workflows::upload_verse::VerseUpload;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

async fn health_shutdown(shutdown: Arc<AtomicBool>) {
    loop {
        if shutdown.load(Relaxed) {
            warn!("health received shutdown");
            return;
        }
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}

async fn start_health_check(shutdown: Arc<AtomicBool>) -> Result<(), Error> {
    let app = Router::new().route("/health", get(health));
    info!(target: "bosca", "Listening on http://0.0.0.0:9000");
    axum::serve(TcpListener::bind("0.0.0.0:9000").await?, app).with_graceful_shutdown(health_shutdown(shutdown)).await?;
    warn!("health check finished");
    Ok(())
}

async fn start(
    shutdown: Arc<AtomicBool>,
    activities_by_id: Arc<HashMap<String, Arc<Box<dyn Activity + Send + Sync>>>>,
    client: &Client,
    max_running: i32,
    queue: String,
    health_check: bool,
) -> Result<(), Error> {
    if health_check {
        return start_health_check(Arc::clone(&shutdown)).await;
    }
    process_queue(Arc::clone(&shutdown), activities_by_id, client, max_running, queue).await
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    structured_logger::Builder::with_level("info")
        .with_target_writer(
            "*",
            structured_logger::async_json::new_writer(tokio::io::stdout()),
        )
        .init();

    let url = env::var("BOSCA_URL").unwrap_or_else(|_| {
        println!("Environment variable BOSCA_URL could not be read");
        exit(1);
    });
    let username = env::var("BOSCA_USERNAME").unwrap_or_else(|_| {
        println!("Environment variable BOSCA_USERNAME could not be read");
        exit(1);
    });
    let password = env::var("BOSCA_PASSWORD").unwrap_or_else(|_| {
        println!("Environment variable BOSCA_PASSWORD could not be read");
        exit(1);
    });
    let queues = env::var("BOSCA_QUEUES").unwrap_or_else(|_| {
        println!("Environment variable BOSCA_QUEUES could not be read");
        exit(1);
    });

    let mut activities = get_default_activities();
    activities.push(Box::new(ProcessBibleActivity::default()));
    activities.push(Box::new(ChapterUpload::default()));
    activities.push(Box::new(BookUpload::default()));
    activities.push(Box::new(VerseUpload::default()));
    activities.push(Box::new(CreateChapterVerseTable::default()));

    let mut activities_by_id = HashMap::<String, Arc<Box<dyn Activity + Send + Sync>>>::new();
    for activity in activities.into_iter() {
        activities_by_id.insert(activity.id().clone(), Arc::new(activity));
    }

    let client = Client::new(&url);
    client.login(&username, &password).await.unwrap();

    info!(target: "workflow", "running");

    let shutdown = Arc::new(AtomicBool::new(false));
    #[cfg(unix)]
    {
        let mut interrupt = signal(SignalKind::interrupt()).unwrap();
        let mut terminate = signal(SignalKind::terminate()).unwrap();

        let shutdown_spawn = Arc::clone(&shutdown);
        tokio::spawn(async move {
            tokio::select! {
                _ = interrupt.recv() => {
                    warn!("Receiving SIGINT, shutting down");
                    shutdown_spawn.store(true, Relaxed);
                },
                _ = terminate.recv() => {
                    warn!("Receiving SIGTERM, shutting down");
                    shutdown_spawn.store(true, Relaxed);
                }
            }
        });
    }

    #[cfg(windows)]
    {
        let mut interrupt = ctrl_c().unwrap();

        let shutdown_spawn = Arc::clone(&shutdown);
        tokio::spawn(async move {
            tokio::select! {
                _ = interrupt.recv() => {
                    warn!("Receiving ctrl_c, shutting down");
                    shutdown_spawn.store(true, Relaxed);
                },
            }
        });
    }

    let shutdown_login_spawn = Arc::clone(&shutdown);
    let login_client = client.clone();
    tokio::spawn(async move {
        loop {
            if shutdown_login_spawn.load(Relaxed) {
                return;
            }
            tokio::time::sleep(Duration::from_secs(300)).await;
            if let Err(err) = login_client.login(&username, &password).await {
                error!("failed to refresh token: {}", err);
            }
        }
    });

    let activities_by_id = Arc::new(activities_by_id);
    let mut processors = Vec::new();
    for queue in queues.split(";") {
        let queue_cfg = queue.trim().to_string();
        let queue_cfg = queue_cfg.split(",").collect::<Vec<&str>>();
        let queue = queue_cfg.first().unwrap().to_string();
        let mut max_running = 0;
        if queue_cfg.len() == 2 {
            max_running = queue_cfg.last().unwrap().parse::<i32>().unwrap();
        }
        let activities_by_id = Arc::clone(&activities_by_id);
        processors.push(start(
            Arc::clone(&shutdown),
            activities_by_id,
            &client,
            max_running,
            queue.clone(),
            false,
        ));
    }
    processors.push(start(
        Arc::clone(&shutdown),
        activities_by_id,
        &client,
        0,
        "".to_owned(),
        true,
    ));

    join_all(processors).await;

    info!(target: "workflow", "finishing");
}
