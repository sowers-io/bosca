mod authed_subscription;
mod context;
mod datastores;
mod files;
mod graphql;
mod logger;
mod models;
mod queries;
mod redis;
mod schema;
mod security;
mod util;
mod worklfow;

use crate::files::{download, upload};
use crate::graphql::content::storage::{ObjectStorage, ObjectStorageInterface};
use crate::models::content::collection::{CollectionInput, CollectionType};
use crate::models::security::permission::{Permission, PermissionAction};
use crate::security::authorization_extension::{
    get_anonymous_principal, get_auth_header, get_cookie_header, Authorization,
};
use crate::security::jwt::{Jwt, Keys};
use crate::worklfow::queue::JobQueues;
use async_graphql::extensions::apollo_persisted_queries::ApolloPersistedQueries;
use async_graphql::{http::GraphiQLSource, Error, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::{DefaultBodyLimit, State};
use axum::routing::post;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use chrono::Utc;
use graphql::mutation::MutationObject;
use graphql::query::QueryObject;
use http::HeaderMap;
use log::{error, info, warn};
use meilisearch_sdk::client::Client;
use object_store::aws::AmazonS3Builder;
use object_store::local::LocalFileSystem;
use opentelemetry::{global, KeyValue};
use serde_json::Value;
use std::env;
use std::fs::create_dir_all;
use std::path::Path;
use std::process::exit;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use uuid::Uuid;

use crate::context::BoscaContext;
use crate::datastores::security::SecurityDataStore;
use crate::datastores::workflow::WorkflowDataStore;
use crate::util::RUNNING_BACKGROUND;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::{BatchConfigBuilder, BatchSpanProcessor, TracerProvider};
use opentelemetry_sdk::{runtime, Resource};
use rustls::crypto::ring;
#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};
#[cfg(windows)]
use tokio::signal::windows::ctrl_c;
use tower_http::timeout::TimeoutLayer;

use crate::datastores::persisted_queries::PersistedQueriesDataStore;
use bosca_telemetry::graphql_opentelemetry::OpenTelemetry;
use mimalloc::MiMalloc;
use tower_http::cors::CorsLayer;

use crate::authed_subscription::AuthGraphQLSubscription;
use crate::datastores::notifier::Notifier;
use crate::datastores::profile::ProfileDataStore;
use crate::graphql::subscription::SubscriptionObject;
use crate::logger::Logger;
use crate::models::profiles::profile::ProfileInput;
use crate::models::profiles::profile_visibility::ProfileVisibility;
use crate::redis::RedisClient;
use crate::schema::BoscaSchema;
use crate::util::profile::add_password_principal;
use bosca_database::build_pool;
use tokio::time::sleep;
use crate::datastores::content::content::ContentDataStore;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

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
    ObjectStorage::new(ObjectStorageInterface::S3(Arc::new(
        AmazonS3Builder::from_env().build().unwrap(),
    )))
}

fn build_object_storage() -> ObjectStorage {
    match env::var("STORAGE") {
        Ok(name) => match name.as_str() {
            "s3" => build_s3_object_storage(),
            _ => build_filesystem_object_storage(),
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

async fn build_redis_client(key: &str) -> Result<RedisClient, Error> {
    let url = match env::var(key) {
        Ok(url) => url,
        _ => "redis://127.0.0.1:6380".to_string(),
    };
    RedisClient::new(url).await
}

async fn initialize_security(
    security: &SecurityDataStore,
    workflow: &WorkflowDataStore,
    content: &ContentDataStore,
    profiles: &ProfileDataStore,
) {
    match security.get_principal_by_identifier("admin").await {
        Ok(_) => {}
        Err(_) => {
            let groups = vec![];
            security
                .add_anonymous_principal(Value::Null, &groups)
                .await
                .unwrap();

            let identifier = "admin".to_string();
            let password = "password".to_string();
            let profile = ProfileInput {
                name: "Administrator".to_string(),
                visibility: ProfileVisibility::Public,
                attributes: vec![],
            };
            let principal = add_password_principal(
                security,
                workflow,
                content,
                profiles,
                &identifier,
                &password,
                &profile,
                true,
                false,
            )
            .await
            .unwrap();

            let group = security.get_administrators_group().await.unwrap();
            security
                .add_principal_group(&principal.id, &group.id)
                .await
                .unwrap();
        }
    }
}

async fn initialize_content(ctx: &BoscaContext) {
    let root_collection_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap();
    match ctx
        .content
        .collections
        .get(&root_collection_id)
        .await
        .unwrap()
    {
        Some(_) => {}
        None => {
            ctx.workflow
                .initialize_default_search_index()
                .await
                .unwrap();
            initialize_collection(ctx, "Root", CollectionType::Root, Value::Null).await;
            initialize_collection(
                ctx,
                "Raw Bibles",
                CollectionType::System,
                serde_json::json!({"collection": "raw-bibles"}),
            )
            .await;
            initialize_collection(
                ctx,
                "Bibles",
                CollectionType::System,
                serde_json::json!({"collection": "bibles"}),
            )
            .await;
        }
    }
}

async fn initialize_collection(
    ctx: &BoscaContext,
    name: &str,
    collection_type: CollectionType,
    attributes: Value,
) {
    let input = CollectionInput {
        parent_collection_id: None,
        name: name.to_string(),
        collection_type: Some(collection_type),
        attributes: if attributes.is_null() {
            None
        } else {
            Some(attributes)
        },
        ..Default::default()
    };
    let collection_id = ctx.content.collections.add(&input).await.unwrap();
    let group = ctx.security.get_administrators_group().await.unwrap();
    let permission = Permission {
        entity_id: collection_id,
        group_id: group.id,
        action: PermissionAction::Manage,
    };
    ctx.content
        .collection_permissions
        .add(&permission)
        .await
        .unwrap();
    let principal = ctx
        .security
        .get_principal_by_identifier("admin")
        .await
        .unwrap();
    let collection = ctx
        .content
        .collections
        .get(&collection_id)
        .await
        .unwrap()
        .unwrap();
    ctx.content
        .collection_workflows
        .set_ready(&collection_id)
        .await
        .unwrap();
    ctx.content
        .collection_workflows
        .set_state(
            &principal,
            &collection,
            "published",
            "initializing collections",
            true,
            true,
        )
        .await
        .unwrap()
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
    let url_secret_key = match env::var("URL_SECRET_KEY") {
        Ok(url_secret_key) => url_secret_key,
        _ => {
            println!(
                "Environment variable URL_SECRET_KEY could not be read, generating a random value"
            );
            Uuid::new_v4().to_string()
        }
    };

    let redis_jobs_queue_client = build_redis_client("REDIS_JOBS_QUEUE_URL").await.unwrap();
    let redis_notifier_client = build_redis_client("REDIS_NOTIFIER_PUBSUB_URL")
        .await
        .unwrap();
    let notifier = Arc::new(Notifier::new(redis_notifier_client.clone()));
    let jobs = JobQueues::new(
        Arc::clone(&bosca_pool),
        redis_jobs_queue_client.clone(),
        Arc::clone(&notifier),
    );
    let search = build_search_client();
    let ctx = BoscaContext {
        security: SecurityDataStore::new(Arc::clone(&bosca_pool), build_jwt(), url_secret_key),
        workflow: WorkflowDataStore::new(
            Arc::clone(&bosca_pool),
            jobs.clone(),
            Arc::clone(&notifier),
            Arc::clone(&search),
        ),
        profile: ProfileDataStore::new(Arc::clone(&bosca_pool)),
        queries: PersistedQueriesDataStore::new(Arc::clone(&bosca_pool)).await,
        content: ContentDataStore::new(bosca_pool, Arc::clone(&notifier)),
        notifier,
        search,
        storage: build_object_storage(),
        principal: get_anonymous_principal(),
    };

    initialize_security(&ctx.security, &ctx.workflow, &ctx.content, &ctx.profile).await;
    initialize_content(&ctx).await;

    let jobs_expiration = jobs.clone();
    tokio::spawn(async move {
        loop {
            RUNNING_BACKGROUND.fetch_add(1, Relaxed);
            let now = Utc::now().timestamp();
            if let Err(e) = jobs_expiration.check_for_expiration(now).await {
                error!(target: "workflow", "failed to check for expiration: {:?}", e);
            }
            RUNNING_BACKGROUND.fetch_add(-1, Relaxed);
            sleep(Duration::from_secs(3)).await;
        }
    });

    let mut provider_builder = TracerProvider::builder().with_config(
        opentelemetry_sdk::trace::Config::default().with_resource(Resource::new(vec![
            KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "bosca-server",
            ),
        ])),
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
                    .build(),
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
    let persisted_queries = ApolloPersistedQueries::new(ctx.queries.cache.clone());

    let schema = Schema::build(QueryObject, MutationObject, SubscriptionObject)
        .data(ctx.clone())
        .extension(Authorization)
        .extension(telemetry)
        .extension(persisted_queries)
        .extension(Logger)
        .data(ctx.clone())
        .finish();

    let upload_limit: usize = match env::var("UPLOAD_LIMIT") {
        Ok(limit) => limit.parse().unwrap(),
        _ => 2147483648,
    };

    let files = Router::new()
        .route("/upload", post(upload))
        .route("/download", get(download))
        .with_state(ctx.clone());

    let app = Router::new()
        .route("/", get(graphiql))
        .nest("/files", files)
        .route("/graphql", post(graphql_handler))
        .route("/graphql", get(graphql_handler))
        .route_service("/ws", AuthGraphQLSubscription::new(schema.clone(), ctx))
        .layer(DefaultBodyLimit::max(upload_limit))
        .layer(CorsLayer::permissive())
        .layer(TimeoutLayer::new(Duration::from_secs(600)))
        .with_state(schema);

    info!(target: "bosca", "Listening on http://0.0.0.0:8000");

    axum::serve(TcpListener::bind("0.0.0.0:8000").await.unwrap(), app)
        .with_graceful_shutdown(shutdown_hook())
        .await
        .unwrap();
}
