use async_trait::async_trait;
use framework::action::{text, Action, Responsable};
use framework::app::App;
use framework::http::error::HttpError;
use framework::http::request::HttpRequest;
use markdown::to_html;
use minijinja::context;
use serde::Serialize;
use std::fs::read_to_string;

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
        let result = app.template(&self.template, self);

        match result {
            Ok(template) => text(template),
            Err(error) => Err(HttpError::new(500, error.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct DocsPage;

#[async_trait]
impl Action for DocsPage {
    async fn handle(
        &self,
        app: &App,
        request: HttpRequest,
    ) -> Result<Box<dyn Responsable>, HttpError> {
        // First load and parse md file
        let doc = request.var("slug").unwrap();
        let md = read_to_string(format!("resource/template/docs/md/{doc}.md")).unwrap();
        let html = to_html(md.as_str());
        let result = app.template("docs/show.html", context!(content => html));

        match result {
            Ok(rendered) => text(rendered),
            Err(error) => Err(HttpError::new(500, error.to_string())),
        }
    }
}
