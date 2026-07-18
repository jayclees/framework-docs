use crate::action::pages::{DocsPage, StandardPage};
use framework::routing::router::Router;

pub fn register_routes(router: &mut Router) -> () {
    router.getn(
        "/",
        StandardPage::new(
            "Sturdy Framework",
            "A New Framework Designed For The Modern Web",
            "landing.html",
        ),
        "landing",
    );
    router.getn(
        "/about",
        StandardPage::new(
            "About - Sturdy Framework",
            "A New Framework Designed For The Modern Web",
            "about.html",
        ),
        "about",
    );
    router.getn(
        "/docs",
        StandardPage::new(
            "Docs - Sturdy Framework",
            "Learn about the features for Sturdy Framework.",
            "docs/index.html",
        ),
        "docs.index",
    );
    router.getn(
        "/docs/{slug}",
        DocsPage,
        "docs.index",
    );
}
