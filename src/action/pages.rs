use async_trait::async_trait;
use framework::action::{text, Action, Responsable};
use framework::app::App;
use framework::error::HttpError;
use hyper::body::Incoming;
use hyper::Request;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct StandardPage {
    title: &'static str,
    description: &'static str,
    template: &'static str,
}

impl StandardPage {
    pub fn new(
        title: &'static str,
        description: &'static str,
        template: &'static str,
    ) -> StandardPage {
        StandardPage {
            title,
            description,
            template,
        }
    }
}

#[async_trait]
impl Action for StandardPage {
    async fn handle(
        &self,
        app: &App,
        _request: Request<Incoming>,
    ) -> Result<Box<dyn Responsable>, HttpError> {
        let guard = app.template();
        let result = guard.get_template(&self.template);

        match result {
            Ok(template) => text(template.render(self).unwrap()),
            Err(error) => Err(HttpError::new(500, "whoops".to_owned())),
        }
    }
}
