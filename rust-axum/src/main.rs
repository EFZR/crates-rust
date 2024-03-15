pub mod _dev_utils;
mod config;
mod ctx;
mod error;
mod log;
mod model;
mod web;

pub use self::error::{Error, Result};
pub use config::config;

use crate::web::mw_auth;
use crate::web::mw_res_map::mw_res_map;
use crate::web::{routes_login, routes_static, routes_tickets};
use axum::middleware;
use axum::Router;
use model::ModelController;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::{self, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time() // For early local developement
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // FOR DEV ONLY
    _dev_utils::init_dev().await;

    let mc: ModelController = ModelController::new().await?;

    let routes_apis = routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(mw_res_map))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::serve_dir());

    // region:          --- Start Server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    info!("{} - {:?}\n", "LISTENING", listener);

    axum::serve(listener, routes_all).await.unwrap();
    // endregion:       --- Start Server

    Ok(())
}
