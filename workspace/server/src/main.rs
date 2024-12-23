mod datastores;
mod files;
mod graphql;
mod models;
mod queue;
mod security;
mod util;
mod worklfow;
mod context;

use crate::files::{download, upload};
use crate::graphql::content::storage::{ObjectStorage, ObjectStorageInterface};
use crate::models::content::collection::{CollectionInput, CollectionType};
use crate::models::security::credentials::PasswordCredential;
use crate::models::security::permission::{Permission, PermissionAction};
use crate::security::authorization_extension::{get_anonymous_principal, get_auth_header, get_cookie_header, Authorization};
use crate::security::jwt::{Jwt, Keys};
use crate::util::yaml::parse_string;
use crate::worklfow::configuration::configure;
use crate::worklfow::job_queue::JobQueues;
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use axum::extract::{DefaultBodyLimit, State};
use axum::routing::post;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use std::str::FromStr;
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use graphql::mutation::MutationObject;
use graphql::query::QueryObject;
use http::HeaderMap;
use log::{info, warn};
use meilisearch_sdk::client::Client;
use object_store::local::LocalFileSystem;
use serde_json::Value;
use std::env;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::str::from_utf8;
use std::sync::Arc;
use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;
use base64::Engine;
use object_store::aws::AmazonS3Builder;
use opentelemetry::{global, KeyValue};
use tokio::net::TcpListener;
use uuid::Uuid;

use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, Resource};
use opentelemetry_sdk::trace::{BatchConfigBuilder, BatchSpanProcessor, TracerProvider};
use rustls::crypto::ring;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::pem::PemObject;
use rustls::RootCertStore;
#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};
#[cfg(windows)]
use tokio::signal::windows::ctrl_c;
use tokio_postgres::NoTls;
use tokio_postgres_rustls::MakeRustlsConnect;
use tower_http::timeout::TimeoutLayer;
use crate::context::BoscaContext;
use crate::datastores::content::ContentDataStore;
use crate::datastores::security::SecurityDataStore;
use crate::datastores::workflow::WorkflowDataStore;
use crate::models::content::search::SearchDocumentInput;
use crate::queue::message_queues::MessageQueues;
use crate::util::RUNNING_BACKGROUND;
use crate::util::storage::index_documents_no_checks;

use mimalloc::MiMalloc;
use bosca_telemetry::graphql_opentelemetry::OpenTelemetry;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

type BoscaSchema = Schema<QueryObject, MutationObject, EmptySubscription>;

async fn graphiql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("/graphql")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

async fn graphql_handler(
    State(schema): State<BoscaSchema>,
    headers: HeaderMap,
    request: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = request.into_inner();
    if let Some(data) = get_auth_header(&headers) {
        request = request.data(data);
    } else if let Some(data) = get_cookie_header(&headers) {
        request = request.data(data);
    }
    schema.execute(request).await.into()
}

fn build_jwt() -> Jwt {
    let keys = match env::var("JWT_SECRET") {
        Ok(secret) => Keys::new(secret.as_bytes()),
        _ => {
            println!("Environment variable JWT_SECRET could not be read");
            exit(1);
        }
    };
    let audience = match env::var("JWT_AUDIENCE") {
        Ok(audience) => audience,
        _ => {
            println!("Environment variable JWT_SECRET could not be read");
            exit(1);
        }
    };
    let issuer = match env::var("JWT_ISSUER") {
        Ok(issuer) => issuer,
        _ => {
            println!("Environment variable JWT_SECRET could not be read");
            exit(1);
        }
    };
    Jwt::new(keys, &audience, &issuer)
}

fn build_pool(key: &str) -> Arc<Pool> {
    let mut config = Config::new();
    match env::var(key) {
        Ok(db_url) => config.url = Some(db_url),
        _ => {
            println!("Environment variable {key} could not be read");
            exit(1);
        }
    }
    config.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    let cert_file_key = format!("{}_CERT_FILE", key);
    if let Ok(cert_file) = env::var(cert_file_key.as_str()) {
        let mut store = RootCertStore {
            roots: webpki_roots::TLS_SERVER_ROOTS.into(),
        };
        let path_buf = PathBuf::from_str(cert_file.as_str()).unwrap();
        let path = path_buf.as_path();
        let cert = CertificateDer::from_pem_file(path).unwrap();
        store.add(cert).unwrap();
        let tls_config = rustls::ClientConfig::builder()
            .with_root_certificates(store)
            .with_no_client_auth();
        let tls = MakeRustlsConnect::new(tls_config);
        return Arc::new(config.create_pool(Some(Runtime::Tokio1), tls).unwrap());
    }
    let cert_b64_key = format!("{}_CERT_B64", key);
    if let Ok(cert) = env::var(cert_b64_key.as_str()) {
        let mut store = RootCertStore {
            roots: webpki_roots::TLS_SERVER_ROOTS.into(),
        };
        let bytes = cert.into_bytes();
        let decoded = base64::prelude::BASE64_STANDARD.decode(bytes).unwrap();
        let cert = CertificateDer::from_pem_slice(&decoded).unwrap();
        store.add(cert).unwrap();
        let tls_config = rustls::ClientConfig::builder()
            .with_root_certificates(store)
            .with_no_client_auth();
        let tls = MakeRustlsConnect::new(tls_config);
        return Arc::new(config.create_pool(Some(Runtime::Tokio1), tls).unwrap());
    }
    Arc::new(config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap())
}

fn build_filesystem_object_storage() -> ObjectStorage {
    let current_dir = match env::var("STORAGE") {
        Ok(dir) => Path::new(dir.as_str()).to_path_buf(),
        _ => env::current_dir().unwrap().join(Path::new("files")),
    };
    let path = current_dir.as_path();
    if !path.exists() {
        create_dir_all(path).unwrap();
    }
    info!("Using file object storage at path: {path:?}");
    ObjectStorage::new(ObjectStorageInterface::FileSystem(Arc::new(
        LocalFileSystem::new_with_prefix(path).unwrap(),
    )))
}

fn build_s3_object_storage() -> ObjectStorage {
    info!("Using s3 object storage");
    ObjectStorage::new(ObjectStorageInterface::S3(Arc::new(AmazonS3Builder::from_env().build().unwrap())))
}

fn build_object_storage() -> ObjectStorage {
    match env::var("STORAGE") {
        Ok(name) => match name.as_str() {
            "s3" => build_s3_object_storage(),
            _ => build_filesystem_object_storage()
        },
        _ => build_filesystem_object_storage(),
    }
}

fn build_search_client() -> Arc<Client> {
    let url = match env::var("SEARCH_URL") {
        Ok(url) => url,
        _ => {
            println!("Environment variable SEARCH_URL could not be read");
            exit(1);
        }
    };
    let key = match env::var("SEARCH_KEY") {
        Ok(url) => url,
        _ => {
            println!("Environment variable SEARCH_KEY could not be read");
            exit(1);
        }
    };
    Arc::new(Client::new(url, Some(key)).unwrap())
}

async fn initialize_workflow(ctx: &BoscaContext) {
    let current = ctx.workflow.get_workflows().await.unwrap();
    if !current.is_empty() {
        return;
    }
    let default_workflow_contents = from_utf8(include_bytes!("../workflows.yaml")).unwrap();
    let yaml = parse_string(default_workflow_contents).unwrap();
    if configure(&yaml, &ctx.workflow).await {
        let storage_system = ctx.workflow.get_default_search_storage_system().await.unwrap();
        let index_name = storage_system.configuration.get("indexName").unwrap().as_str().unwrap().to_owned();
        let create_task = ctx.search.create_index(index_name.clone(), Some("_id")).await.unwrap();
        ctx.search.wait_for_task(create_task, None, None).await.unwrap();
        let index = ctx.search.get_index(index_name).await.unwrap();
        let mut settings = index.get_settings().await.unwrap();
        settings.filterable_attributes = Some(vec!["_type".to_owned()]);
        index.set_settings(&settings).await.unwrap();
    } else {
        ctx.workflow.create_queues().await.unwrap();
    }
}

async fn initialize_security(datastore: &SecurityDataStore) {
    match datastore.get_principal_by_identifier("admin").await {
        Ok(_) => {}
        Err(_) => {
            let password = PasswordCredential::new("admin".to_string(), "password".to_string());
            let group = datastore.get_administrators_group().await.unwrap();
            let groups = vec![&group].into_iter().map(|g| &g.id).collect();
            datastore
                .add_principal(true, Value::Null, &password, &groups)
                .await
                .unwrap();
            let groups = vec![];
            datastore
                .add_anonymous_principal(Value::Null, &groups)
                .await
                .unwrap();
        }
    }
}

async fn initialize_content(ctx: &BoscaContext) {
    let root_collection_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
    match ctx.content
        .get_collection(&root_collection_id)
        .await
        .unwrap()
    {
        Some(_) => {}
        None => {
            let input = CollectionInput {
                parent_collection_id: None,
                name: "Root".to_string(),
                collection_type: Some(CollectionType::Root),
                attributes: None,
                labels: None,
                state: None,
                description: None,
                index: None,
                ordering: None,
                metadata: None,
                collections: None,
                ready: Some(true),
            };
            ctx.content.add_collection(&input).await.unwrap();
            let group = ctx.security.get_administrators_group().await.unwrap();
            let permission = Permission {
                entity_id: root_collection_id,
                group_id: group.id,
                action: PermissionAction::Manage,
            };
            ctx.content
                .add_collection_permission(&permission)
                .await
                .unwrap();
            let search_docs = vec![SearchDocumentInput {
                collection_id: Some(root_collection_id.to_string()),
                metadata_id: None,
                content: "".to_owned()
            }];
            let storage_system = ctx.workflow.get_default_search_storage_system().await.unwrap();
            index_documents_no_checks(ctx, &search_docs, &storage_system).await.unwrap();
        }
    }
}

#[cfg(unix)]
async fn shutdown_hook() {
    let mut interrupt = signal(SignalKind::interrupt()).unwrap();
    let mut terminate = signal(SignalKind::terminate()).unwrap();
    tokio::select! {
        _ = interrupt.recv() => {
            warn!("Received SIGINT, shutting down");
            loop {
                if RUNNING_BACKGROUND.load(Relaxed) > 0 {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                } else {
                    break
                }
            }
        },
        _ = terminate.recv() => {
            warn!("Received SIGTERM, shutting down");
            loop {
                if RUNNING_BACKGROUND.load(Relaxed) > 0 {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                } else {
                    break
                }
            }
        }
    }
}

#[cfg(windows)]
async fn shutdown_hook() {
    let mut interrupt = ctrl_c().unwrap();
    tokio::select! {
        _ = interrupt.recv() => {
            warn!("Received ctr_c, shutting down");
            loop {
                if RUNNING_BACKGROUND.load(Relaxed) > 0 {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                } else {
                    break
                }
            }
        },
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    structured_logger::Builder::with_level("info")
        .with_target_writer(
            "*",
            structured_logger::async_json::new_writer(tokio::io::stdout()),
        )
        .init();

    ring::default_provider().install_default().unwrap();

    let bosca_pool = build_pool("DATABASE_URL");
    let job_pool = build_pool("DATABASE_JOBS_URL");

    let messages = MessageQueues::new(job_pool);
    let jobs = JobQueues::new(messages.clone());
    let ctx = BoscaContext {
        security: SecurityDataStore::new(
            Arc::clone(&bosca_pool),
            build_jwt(),
        ),
        workflow: WorkflowDataStore::new(
            Arc::clone(&bosca_pool),
            jobs.clone(),
        ),
        content: ContentDataStore::new(bosca_pool),
        search: build_search_client(),
        storage: build_object_storage(),
        principal: get_anonymous_principal(),
        messages,
    };

    initialize_workflow(&ctx).await;
    initialize_security(&ctx.security).await;
    initialize_content(&ctx).await;
    
    let mut provider_builder = TracerProvider::builder().with_config(
        opentelemetry_sdk::trace::Config::default().with_resource(Resource::new(vec![KeyValue::new(
            opentelemetry_semantic_conventions::resource::SERVICE_NAME,
            "bosca-server",
        )])),
    );

    if let Ok(endpoint) = env::var("OTLP_TRACE_ENDPOINT") {
        info!(target: "bosca", "sending traces to: {}", endpoint);

        let exporter = opentelemetry_otlp::new_exporter()
            .http()
            .with_http_client(reqwest::Client::new())
            .with_endpoint(endpoint)
            .build_span_exporter()
            .unwrap();

        let batch = BatchSpanProcessor::builder(exporter, runtime::Tokio)
            .with_batch_config(
                BatchConfigBuilder::default()
                    .with_max_queue_size(4096)
                    .build()
            )
            .build();

        provider_builder = provider_builder.with_span_processor(batch);
    } else {
        info!(target: "bosca", "no exporter configured");
    }

    let provider = provider_builder.build();
    let tracer = provider.tracer("Bosca");
    let _ = global::set_tracer_provider(provider);

    let telemetry = OpenTelemetry::new(tracer);

    let schema = Schema::build(QueryObject, MutationObject, EmptySubscription)
        .extension(Authorization)
        .extension(telemetry)
        .data(ctx.clone())
        .finish();

    let upload_limit: usize = match env::var("UPLOAD_LIMIT") {
        Ok(limit) => limit.parse().unwrap(),
        _ => 1073741824,
    };

    let files = Router::new()
        .route("/upload", post(upload))
        .route("/download", get(download))
        .with_state(ctx);

    let app = Router::new()
        .route("/", get(graphiql))
        .nest("/files", files)
        .route("/graphql", post(graphql_handler))
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .layer(DefaultBodyLimit::max(upload_limit))
        .layer(TimeoutLayer::new(Duration::from_secs(600)))
        .with_state(schema);

    info!(target: "bosca", "Listening on http://0.0.0.0:8000");

    axum::serve(TcpListener::bind("0.0.0.0:8000").await.unwrap(), app)
        .with_graceful_shutdown(shutdown_hook())
        .await
        .unwrap();
}
