use crate::AppState;
use async_trait::async_trait;
use framework::action::{text, Action, Responsable};
use framework::app::App;
use framework::http::error::HttpError;
use framework::http::request::HttpRequest;
use markdown::{to_html_with_options, CompileOptions, Options};
use minijinja::context;
use serde::Serialize;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
pub struct StandardPage {
    pub path: &'static str,
    pub route_name: &'static str,
    pub template: &'static str,
    pub seo: Seo,
}

impl StandardPage {
    pub fn new(
        path: &'static str,
        route_name: &'static str,
        template: &'static str,
        seo: Seo,
    ) -> StandardPage {
        StandardPage {
            path,
            route_name,
            template,
            seo,
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
        let result = app.template(
            &self.template,
            context! {
                // seo_title => self.seo.0,
                seo_description => self.seo.1,
                // template => self.template,
            },
        );

        match result {
            Ok(template) => text(template),
            Err(error) => Err(HttpError::new(500, error.to_string())),
        }
    }
}


#[derive(Debug, Clone)]
pub struct DocIndexPage {
    pub seo: Seo,
}

impl DocIndexPage {
    pub fn new(seo: Seo) -> DocIndexPage {
        DocIndexPage { seo }
    }
}

#[async_trait]
impl Action for DocIndexPage {
    async fn handle(
        &self,
        app: &App,
        _request: HttpRequest,
    ) -> Result<Box<dyn Responsable>, HttpError> {
        let mut doc_pages = &app.state::<AppState>().doc_pages_vec();
        dbg!(&doc_pages);
        let result = app.template(
            "docs/index.html",
            context! {
                doc_pages,
                seo_title => self.seo.0,
                seo_description => self.seo.1,
            },
        );

        match result {
            Ok(template) => text(template),
            Err(error) => Err(HttpError::new(500, error.to_string())),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DocPage {
    pub title: &'static str,
    pub description: &'static str,
    pub md_template: &'static str,
    pub route_name: &'static str,
    pub index: u8,
}

impl DocPage {
    pub fn new(
        title: &'static str,
        description: &'static str,
        md_template: &'static str,
        route_name: &'static str,
        index: u8,
    ) -> DocPage {
        DocPage {
            title,
            description,
            md_template,
            route_name,
            index,
        }
    }
}

#[async_trait]
impl Action for DocPage {
    async fn handle(
        &self,
        app: &App,
        _request: HttpRequest,
    ) -> Result<Box<dyn Responsable>, HttpError> {
        let mut doc_pages = &app.state::<AppState>().doc_pages_vec();
        let md = read_to_string(format!("resource/template/docs/md/{}", self.md_template));
        //
        match md {
            Ok(md) => {
                let html = to_html_with_options(
                    md.as_str(),
                    &Options {
                        compile: CompileOptions {
                            allow_dangerous_html: true,
                            ..CompileOptions::default()
                        },
                        ..Options::default()
                    },
                )
                .unwrap();
                let result = app.template(
                    "docs/show.html",
                    context!(
                        doc_pages,
                        content => html,
                    ),
                );

                match result {
                    Ok(rendered) => text(rendered),
                    Err(error) => Err(HttpError::new(500, error.to_string())),
                }
            }
            Err(error) => Err(HttpError::new(404, error.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Seo(pub &'static str, pub &'static str);
