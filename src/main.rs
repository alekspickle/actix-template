//! ![actix-template](https://github.com/alekspickle/actix-template.git)
//!
//! ## Overview
//! Template to have something to get-go in some situations
//!
//! This template provides:
//! - [x] Axum server(with middleware)
//! - [x] Templates
//! - [x] Containerization
//! - [ ] NATS integration setup
//!

use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};

mod error;
mod handlers;
//mod middleware;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let addr = ("0.0.0.0", 7777);

    log::info!("listening on {}:{}", addr.0, addr.1);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(handlers::home)))
            .service(Files::new("/", "./static/"))
            .service(web::resource("/posts").route(web::get().to(handlers::posts)))
            .service(web::resource("/hello").route(web::get().to(handlers::hello)))
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}
