use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};
use tower::{ServiceBuilder, ServiceExt};
use tower_http::services::fs::{ServeDir, ServeFileSystemResponseBody};

//const FILE_DIR: "./app/dist";
//
//struct StaticFile ( ServeFileSystemResponseBody);
//
//impl<B> FromRequest<B> for StaticFile
//where
//    B: Send,
//{
//    type Rejection = StatusCode;
//
//    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
//        let path = req.uri().path();
//
//        match ServiceBuilder::new().and_then(|response|: Response<ServeFileSystemResponseBody> || async move {
//            if response.status() == StatusCode::NOT_FOUND {
//                Err(StatusCode::NOT_FOUND)
//            } else {
//                StaticFile { response}
//            }
//        }).service(ServeDir::new("./app/dist").oneshot(path).await { }
//    }
//}
