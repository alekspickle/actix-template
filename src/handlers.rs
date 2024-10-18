use std::{collections::HashMap, path::PathBuf};

use actix_files::NamedFile;
use actix_web::{
    http::StatusCode,
    web::{self, Html},
    HttpRequest, Responder, Result,
};
use askama::Template;

#[derive(Debug, Clone, Default)]
struct Post {
    id: String,
    title: String,
    content: String,
}

pub async fn home() -> Result<impl Responder> {
    let html = templates::Home {
        title: "Home".into(),
    }
    .render()
    .expect("template broken");

    Ok(Html::new(html))
}

pub async fn files(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

pub async fn hello(query: web::Query<HashMap<String, String>>) -> Result<impl Responder> {
    let name = query.get("name").map_or("stranger", |l| l);
    let html = templates::Hello {
        name: name.to_string(),
        title: "Hello".into(),
    }
    .render()
    .expect("template broken");

    Ok(Html::new(html))
}

pub async fn posts() -> Result<impl Responder> {
    let html = templates::Posts {
        title: "Home".into(),
        posts: Default::default(),
    }
    .render()
    .expect("template broken");

    Ok(Html::new(html))
}

/// Basically all templates handling
pub mod templates {
    use super::*;

    #[derive(Template)]
    #[template(path = "main.html")]
    pub struct Home {
        pub title: String,
    }

    #[derive(Template)]
    #[template(path = "hello.html")]
    pub struct Hello {
        pub title: String,
        pub name: String,
    }

    #[derive(Template)]
    #[template(path = "posts.html")]
    pub struct Posts {
        pub title: String,
        pub posts: Vec<Post>,
    }

    #[derive(Template)]
    #[template(path = "404.html")]
    pub struct NotFound {
        pub title: String,
        pub uri: String,
    }
}
