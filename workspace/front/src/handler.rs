use axum::body::Body;
use crate::context::{FrontCache, FrontContext};
use axum::extract::{Request, State};
use http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use tera::Context;
use tokio::fs::File;
use crate::minify_css::minify_css;
use crate::minify_html::minify_html;
use tokio_util::io::ReaderStream;

pub async fn handler(State(ctx): State<FrontContext>, request: Request) -> Result<(HeaderMap, Body), (StatusCode, String)> {
    let uri = request.uri();
    let path = request.uri().path();
    if path.starts_with("/images") {
        let file = match File::open(format!("templates/default/src{}", path)).await {
            Ok(file) => file,
            Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
        };
        // convert the `AsyncRead` into a `Stream`
        let stream = ReaderStream::new(file);
        // convert the `Stream` into an `axum::body::HttpBody`
        let body = Body::from_stream(stream);
        return Ok((HeaderMap::new(), body));
    }
    let file = if path.ends_with("/") {
        format!("{}index.html", path)
    } else if !path.contains(".") {
        format!("{}.html", path)
    } else {
        path.to_owned()
    };
    if let Some(content) = ctx.cache.read().await.get(&file) {
        return Ok((content.headers.clone(), Body::from(content.body.clone())));
    }
    if !file.ends_with(".html") && !file.ends_with(".css") && !file.ends_with(".js") && !file.ends_with(".svg") {
        return Err((StatusCode::NOT_FOUND, format!("Not Found: {}", uri.path())));
    }
    let template_name = &file[1..file.len()];
    let result = ctx.render(template_name, &Context::default());
    let content_type: String;
    let minified = if template_name.ends_with(".html") {
        content_type = "text/html".to_owned();
        if ctx.minify {
            minify_html(result)
        } else {
            result
        }
    } else if template_name.ends_with(".css") {
        content_type = "text/css".to_owned();
        if ctx.minify {
            minify_css(result)
        } else {
            result
        }
    } else if template_name.ends_with(".js") {
        content_type = "text/javascript".to_owned();
        result
    } else if template_name.ends_with(".svg") {
        content_type = "image/svg+xml".to_owned();
        result
    } else {
        content_type = "text/plain".to_owned();
        result
    };
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::try_from("Content-Type".to_owned()).unwrap(), HeaderValue::try_from(content_type).unwrap());
    {
        let mut cache = ctx.cache.write().await;
        cache.insert(file.to_owned(), FrontCache {
            headers: headers.clone(),
            body: minified.clone(),
        });
    }
    Ok((headers, Body::from(minified)))
}