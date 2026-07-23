use crate::action::pages::{DocIndexPage, Seo};
use crate::state::AppState;
use sturdy::routing::router::Router;

pub fn register_routes(state: &AppState, router: &mut Router) {
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
}
