use axum::{
    http::{StatusCode, Uri},
    response::{Html, Response},
};
use sycamore::prelude::*;
use tokio::fs::read;

pub async fn static_app(uri: Uri) -> Result<Html<String>, StatusCode> {
    let index_html = String::from_utf8(
        read("app/dist/index.html")
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    )
    .expect("index.html should be valid utf-8");

    let rendered = sycamore::render_to_string(|| {
        view! {
            app::App()
        }
    });

    let index_html = index_html.replace("%sycamore.body", &rendered);
    Ok(Html(index_html))
}

//
//
//pub async fn static_app(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
//    let res = get_static_file(uri.clone()).await?;
//
//    if res.status() == StatusCode::NOT_FOUND {
//        match format!("{}.html", uri).parse() {
//            Ok(uri_html) => get_static_file(uri_html).await,
//            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid URI".to_string())),
//        }
//    } else {
//        Ok(res)
//    }
//}
//
//async fn get_static_file(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
//    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();
//
//    match ServeDir::new("./app/dist").oneshot(req).await {
//        Ok(res) => Ok(res.map(box_body)),
//        Err(err) => Err((
//            StatusCode::INTERNAL_SERVER_ERROR,
//            format!("Something went wrong: {}", err),
//        )),
//    }
//}
