use crate::action::pages::{DocPage, Seo, StandardPage};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct AppState {
    app_name: &'static str,
    pub standard_pages: Vec<StandardPage>,
    pub doc_pages: Arc<HashMap<&'static str, DocPage>>,
}

impl AppState {
    pub fn init() -> AppState {
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

        AppState {
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
                StandardPage::new(
                    "/license",
                    "license",
                    "license.html",
                    Seo("License - Sturdy Framework", "Sturdy Framework license."),
                ),
            ],
            doc_pages: Arc::new(doc_pages),
        }
    }

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
