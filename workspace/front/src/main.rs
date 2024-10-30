mod context;
mod handler;
mod minify_html;
mod minify_css;

use crate::context::{FrontCache, FrontContext};
use crate::handler::handler;
use axum::{
    Router,
};
use std::collections::HashMap;
use std::sync::Arc;
use tera::Tera;
use tokio::sync::RwLock;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    let tera = Arc::new(Tera::new("templates/default/src/**/*.{html,css,svg}").unwrap());
    let cache = Arc::new(RwLock::new(HashMap::<String, FrontCache>::new()));

    // tera.add_template_files(&)
    // tera.add_raw_templates(vec![
    //     ("grandparent", "{% block hey %}hello{% endblock hey %}"),
    //     ("parent", "{% extends \"grandparent\" %}{% block hey %}Parent{% endblock hey %}"),
    // ]).unwrap();

    let app = Router::new()
        .fallback(handler)
        .with_state(FrontContext {
            tera,
            cache,
            minify: true,
        });

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}