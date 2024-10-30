use std::collections::HashMap;
use std::sync::Arc;
use http::HeaderMap;
use tera::{Context, Tera};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct FrontContext {
    pub tera: Arc<Tera>,
    pub cache: Arc<RwLock<HashMap<String, FrontCache>>>,
    pub minify: bool,
}

pub struct FrontCache {
    pub headers: HeaderMap,
    pub body: String,
}

impl FrontContext {
    pub fn render(&self, template_name: &str, ctx: &Context) -> String {
        match self.tera.render(template_name, ctx) {
            Ok(data) => data,
            Err(_) => self.tera.render("404.html", ctx).unwrap_or_else(|_| "Not Found".to_owned())
        }
    }
}