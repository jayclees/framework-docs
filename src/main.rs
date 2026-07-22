mod action;
mod entity;
mod routes;
mod state;

use crate::action::pages::{DocIndexPage, Seo};
use crate::state::AppState;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::Serialize;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use sturdy::app::{App, Env};
use sturdy::cli::Registry;
use sturdy::error::register_panic_hook;
use sturdy::routing::router::Router;
use sturdy::support::logger::Logger;
use sturdy::template::reloader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (host, port, vite_url) = process_args();
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let logger = Logger::new(root.clone());
    register_panic_hook(logger.clone());
    dotenvy::dotenv()?;

    let state = AppState::init();
    let router = Router::new(|router| {
        for page in state.standard_pages.clone() {
            router.getn(page.path, page.clone(), page.route_name);
        }
        router.getn(
            "/docs",
            DocIndexPage {
                seo: Seo(
                    "Documentation - Sturdy Framework",
                    "Learn about the features of Sturdy Framework.",
                ),
            },
            "docs.index",
        );
        for (key, doc_page) in state.doc_pages.clone().iter() {
            let route_name = doc_page.route_name;
            router.getn(
                format!("/docs/{}", key).as_str(),
                doc_page.to_owned(),
                route_name,
            );
        }
    });
    let app = App::new(
        router,
        format!("{host}:{port}"),
        reloader(root.clone()),
        db().await?,
        logger,
        Env::new(env::var("APP_ENV")?, true, Some(vite_url)),
        Box::new(state),
    )
    .await;
    let app = Arc::new(app);

    sturdy::app::run(app).await
}

async fn db() -> Result<DatabaseConnection, Box<dyn Error + Send + Sync>> {
    let mut opt = ConnectOptions::new(env::var("DATABASE_URL")?);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false) // disable SQLx logging
        .sqlx_logging_level(log::LevelFilter::Info);
    let db = Database::connect(opt).await?;
    db.get_schema_registry("bus_pattern_framework::entity::*")
        .sync(&db)
        .await?;
    Ok(db)
}

fn process_args() -> (String, String, String) {
    let registry = Registry::default();
    let parsed = registry.parse(env::args().skip(1).collect());

    match parsed {
        Ok(parsed) => {
            if parsed.help_requested() {
                registry.print_help();
                drop(parsed);
                drop(registry);
                std::process::exit(0);
            }

            (parsed.host(), parsed.port(), parsed.vite_url())
        }
        Err(error) => {
            registry.eprint_help(error.msg().clone());
            panic!("Failed...");
        }
    }
}
