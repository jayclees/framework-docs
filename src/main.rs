mod action;
mod entity;
mod routes;
mod state;

use crate::routes::register_routes;
use crate::state::AppState;
use serde::Serialize;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use sturdy::app::builder::Builder;
use sturdy::cli::resolve_addr;
use sturdy::error::register_panic_hook;
use sturdy::routing::router::Router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    register_panic_hook(root.clone());

    // You may set this using `cargo run -- --host=0.0.0.0 --port=8080`
    let (host, port) = resolve_addr();
    let state = AppState::init();
    let router = Router::new(|router| {
        register_routes(&state, router);
    });
    let addr = format!("{host}:{port}");

    let app = Builder::new(root)
        .listen(addr)
        .router(router)
        .template()
        .db()
        .state(Box::new(state))
        .build()
        .await;

    sturdy::app::run(Arc::new(app)).await
}
