use async_trait::async_trait;
use framework::action::{text, Action, Responsable};
use framework::app::App;
use framework::http::error::HttpError;
use framework::http::request::HttpRequest;
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
        _request: HttpRequest,
    ) -> Result<Box<dyn Responsable>, HttpError> {
        let guard = app.template();
        let result = guard.get_template(&self.template);
        framework::log!("test");

        match result {
            Ok(template) => text(template.render(self).unwrap()),
            Err(_error) => Err(HttpError::new(500, "whoops".to_owned())),
        }
    }
}

#[derive(Debug)]
pub struct _DocsPage(&'static str);

#[async_trait]
impl Action for _DocsPage {
    async fn handle(
        &self,
        _app: &App,
        _request: HttpRequest,
    ) -> Result<Box<dyn Responsable>, HttpError> {
        todo!()
    }
}
