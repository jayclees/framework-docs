mod action;
mod entity;
mod routes;

use crate::action::pages::{DocPage, Seo, StandardPage};
use framework::app::{App, Env};
use framework::cli::Registry;
use framework::error::register_panic_hook;
use framework::routing::router::Router;
use framework::support::logger::Logger;
use minijinja::{path_loader, Environment};
use minijinja_autoreload::AutoReloader;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use serde::Serialize;

#[derive(Debug)]
struct AppState {
    env: Env,
    pub pages: Vec<StandardPage>,
    pub doc_pages: Arc<HashMap<&'static str, DocPage>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (host, port, vite_url) = process_args();
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let logger = Logger::new(root.clone());
    register_panic_hook(logger.clone());
    dotenvy::dotenv()?;

    let mut doc_pages = HashMap::new();
    doc_pages.insert("intro", DocPage {
        title: "About Sturdy Framework",
        description: "Sturdy Framework is a modern, Rust-based framework built for the modern web.",
        md_template: "intro.md",
        route_name: "docs.intro",
    });
    doc_pages.insert(
        "install",
        DocPage {
            title: "Install",
            description: "Get started with Sturdy Framework.",
            md_template: "install.md",
            route_name: "docs.install",
        },
    );
    doc_pages.insert(
        "routing",
        DocPage {
            title: "Routing",
            description: "Register your app/website's routes with Sturdy Framework.",
            md_template: "routing.md",
            route_name: "docs.routing",
        },
    );
    doc_pages.insert(
        "actions",
        DocPage {
            title: "Route Actions",
            description: "Actions are Rust structs that implement the Action trait.",
            md_template: "actions.md",
            route_name: "docs.actions",
        },
    );
    doc_pages.insert(
        "auto-reload",
        DocPage {
            title: "Auto Reload",
            description: "Learn how to initialize the watcher and run the development environment.",
            md_template: "auto-reload.md",
            route_name: "docs.auto-reload",
        },
    );

    let state = AppState {
        env: Env {
            env: "local".to_string(),
            debug: false,
            vite_url: None,
        },
        pages: vec![
            StandardPage::new(
                "/",
                "landing",
                "landing.html",
                Seo(
                    "Sturdy Framework",
                    "A New Framework Designed For The Modern Web.",
                ),
            ),
            StandardPage::new(
                "/about",
                "about",
                "about.html",
                Seo(
                    "About - Sturdy Framework",
                    "A New Framework Designed For The Modern Web.",
                ),
            ),
            StandardPage::new(
                "/docs",
                "docs",
                "docs/index.html",
                Seo(
                    "Documentation - Sturdy Framework",
                    "Learn about the features of Sturdy Framework.",
                ),
            ),
        ],
        doc_pages: Arc::new(doc_pages),
    };

    let router = Router::new(|router| {
        for page in state.pages.clone() {
            router.getn(page.path, page.clone(), page.route_name);
        }
        for (key, doc_page) in state.doc_pages.clone().iter() {
            let route_name = doc_page.route_name;
            router.getn(
                format!("/docs/{}", key).as_str(),
                doc_page.to_owned(),
                route_name,
            );
        }
    });
    let template_reloader = reloader();
    let db = db().await?;
    let env = Env::new("local".to_string(), true, Some(vite_url));
    let addr = format!("{host}:{port}");
    let app = App::new(
        router,
        addr,
        template_reloader,
        db,
        logger,
        env,
        Box::new(state),
    )
    .await;
    let app = Arc::new(app);

    framework::app::run(app).await
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

fn reloader() -> AutoReloader {
    // If DISABLE_AUTORELOAD is set, then the path tracking is disabled.
    let disable_autoreload = env::var("DISABLE_AUTORELOAD").as_deref() == Ok("1");

    // If FAST_AUTORELOAD is set, then fast reloading is enabled.
    let fast_autoreload = env::var("FAST_AUTORELOAD").as_deref() == Ok("1");

    // The closure is invoked every time the environment is outdated to recreate it.
    AutoReloader::new(move |notifier| {
        let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resource/template");
        let mut env = Environment::new();
        env.set_loader(path_loader(&template_path));

        if fast_autoreload {
            notifier.set_fast_reload(true);
        }

        // if watch_path is never called, no fs watcher is created
        if !disable_autoreload {
            notifier.watch_path(&template_path, true);
        }

        Ok(env)
    })
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
