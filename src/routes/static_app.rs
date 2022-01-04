//use crate::routes::extractors;
//use axum::{
//    body::{box_body, Body, BoxBody},
//    handler::Handler,
//    http::{Request, Response, StatusCode, Uri},
//};
//use tower::{util::AndThen, Service, ServiceBuilder, ServiceExt};
//use tower_http::services::fs::{ServeDir, ServeFileSystemResponseBody};

const FILE_DIR: &str = "./app/dist";

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
