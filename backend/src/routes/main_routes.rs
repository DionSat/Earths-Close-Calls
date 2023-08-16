use axum::response::Response;
use axum::routing::*;
use axum::Router;
use http::StatusCode;
use hyper::Body;
use sqlx::PgPool;

use crate::db::Store;
use crate::handlers::{admin_page, neo_date_page, neo_id_page, register_page, root};
use crate::{handlers, layers};

pub async fn app(pool: PgPool) -> Router {
    let db = Store::with_pool(pool);

    let (cors_layer, trace_layer) = layers::get_layers();

    Router::new()
        // The router matches these FROM TOP TO BOTTOM explicitly!
        .route("/", get(root))
        .route("/register", get(register_page))
        .route("/admin", get(admin_page))
        .route("/ban", post(handlers::ban_user))
        .route("/neos", get(handlers::get_neos))
        .route("/neo/date/", get(handlers::get_neo_by_date))
        .route("/neo/date", get(neo_date_page))
        .route("/neo/:neo_id", get(handlers::get_neo_by_id))
        .route("/neo/id", get(neo_id_page))
        .route("/neo", post(handlers::create_neo))
        .route("/users", post(handlers::register))
        .route("/users/admin", post(handlers::register_admin))
        .route("/login", post(handlers::login))
        .route("/protected", get(handlers::protected))
        .route("/*_", get(handle_404))
        .layer(cors_layer)
        .layer(trace_layer)
        .with_state(db)
}

async fn handle_404() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("The requested page could not be found"))
        .unwrap()
}
