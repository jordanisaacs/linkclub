use axum::body::Body;
use axum::routing::post;
use axum::AddExtensionLayer;
use axum::{routing::get, Router, Server};
use routes::*;
use sqlx::PgPool;
use std::future::Future;
use std::net::TcpListener;
use telemetry::{AxumMakeSpan, AxumOnFailure, AxumOnRequest, AxumOnResponse};
use tower_http::trace::TraceLayer;

pub mod configuration;
pub mod middleware;
pub mod routes;
pub mod telemetry;
//}
//                .map_err(|err: std::io::Error| StaticAppError::IoError(err.to_string()))

/// Retrieve a server to run
///
/// Takes a TcpListerer to run server on and a PgPool for
/// for routes to connect to database. PgPool is turned
/// into a layered middleware
pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<impl Future<Output = hyper::Result<()>>, hyper::Error> {
    let db_layer = AddExtensionLayer::new(db_pool);
    let api: Router<Body> = Router::new()
        .route("/health_check", get(health_check))
        .route("/user", post(add_user))
        .layer(db_layer);

    let app: Router<Body> = Router::new()
        .layer(middleware::static_app::StaticAppLayer::new(
            "app/dist".to_string(),
        ))
        .route("/user", post(add_user));

    //let test1 = ServiceBuilder::new()
    //    .map_err(|err: std::io::Error| StaticAppError::IoError(err.to_string()))
    //    .and_then(|res: Response<ServeFileSystemResponseBody>| async move {
    //        if res.status() == StatusCode::NOT_FOUND {
    //            Err(StaticAppError::NotFound)
    //        } else {
    //            Ok(res.map(boxed))
    //        };
    //    })
    //    .service(ServeDir::new("app/dist"));

    //let test = get_service(test1).handle_error(|error: StaticAppError| async move {
    //    match error {
    //        StaticAppError::NotFound => (StatusCode::OK, ()),
    //        StaticAppError::IoError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
    //    }
    //});
    //
    //let test = Router::new().layer(ServiceBuilder::new("app/dist"));

    let router = Router::new().nest("/api", api).nest("/app", app).layer(
        TraceLayer::new_for_http()
            .make_span_with(AxumMakeSpan())
            .on_request(AxumOnRequest)
            .on_response(AxumOnResponse)
            .on_body_chunk(())
            .on_eos(())
            .on_failure(AxumOnFailure),
    );

    let server = Server::from_tcp(listener)?.serve(router.into_make_service());

    Ok(server)
}
