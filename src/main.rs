mod action;
mod entity;
mod routes;

use crate::action::pages::{DocIndexPage, DocPage, Seo, StandardPage};
use framework::app::{App, Env};
use framework::cli::Registry;
use framework::error::register_panic_hook;
use framework::routing::router::Router;
use framework::support::logger::Logger;
use minijinja::{path_loader, Environment};
use minijinja_autoreload::AutoReloader;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug)]
struct AppState {
    env: Env,
    app_name: &'static str,
    pub standard_pages: Vec<StandardPage>,
    pub doc_pages: Arc<HashMap<&'static str, DocPage>>,
}

impl AppState {
    pub fn doc_pages_vec(&self) -> Vec<(&'static str, &DocPage)> {
        let mut doc_pages: Vec<(&'static str, &DocPage)> = Vec::with_capacity(self.doc_pages.len());

        for (k, v) in self.doc_pages.iter() {
            doc_pages.push((*k, v));
        }

        // Vec::from_iter is scrambling the order.
        doc_pages.sort_by(|(_, a), (_, b)| a.index.cmp(&b.index));
        doc_pages
    }
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
        title: "Introduction",
        description: "Sturdy Framework is a modern, Rust-based framework built for the modern web.",
        md_template: "intro.md",
        route_name: "docs.intro",
        index: 0,
    });
    doc_pages.insert(
        "getting-started",
        DocPage {
            title: "Getting Started",
            description: "Get started with Sturdy Framework.",
            md_template: "getting-started.md",
            route_name: "docs.getting-started",
            index: 1,
        },
    );
    doc_pages.insert(
        "routing",
        DocPage {
            title: "Routing",
            description: "Register your app/website's routes with Sturdy Framework.",
            md_template: "routing.md",
            route_name: "docs.routing",
            index: 2,
        },
    );
    doc_pages.insert(
        "actions",
        DocPage {
            title: "Actions",
            description: "Actions are Rust structs that implement the Action trait.",
            md_template: "actions.md",
            route_name: "docs.actions",
            index: 3,
        },
    );
    doc_pages.insert(
        "templates",
        DocPage {
            title: "Templates",
            description: "Use minijinja's powerful templating engine.",
            md_template: "templates.md",
            route_name: "docs.templates",
            index: 4,
        },
    );
    doc_pages.insert(
        "auto-reload",
        DocPage {
            title: "Auto Reload",
            description: "Learn how to initialize the watcher and run the development environment.",
            md_template: "auto-reload.md",
            route_name: "docs.auto-reload",
            index: 5,
        },
    );

    let state = AppState {
        env: Env {
            env: "local".to_string(),
            debug: false,
            vite_url: None,
        },
        app_name: "Sturdy Framework",
        standard_pages: vec![
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
        ],
        doc_pages: Arc::new(doc_pages),
    };

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
