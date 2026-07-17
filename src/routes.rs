use crate::action::pages::StandardPage;
use framework::routing::router::Router;

pub fn register_routes(router: &mut Router) -> () {
    router.getm(
        "/",
        StandardPage::new(
            "Sturdy Framework",
            "A New Framework Designed For The Modern Web",
            "landing.html",
        ),
        |route| route.name("landing"),
    );
    router.getm(
        "/about",
        StandardPage::new(
            "About - Sturdy Framework",
            "A New Framework Designed For The Modern Web",
            "about.html",
        ),
        |route| route.name("about"),
    );
    router.getm(
        "/docs",
        StandardPage::new(
            "Docs - Sturdy Framework",
            "Learn about the features for Sturdy Framework.",
            "docs/index.html",
        ),
        |route| route.name("docs.index"),
    );
}
