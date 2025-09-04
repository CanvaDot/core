#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::match_like_matches_macro)]

use std::io::Error as IoError;

use actix_web::web::get;
use actix_web::{App, HttpResponse, HttpServer, main};
use thiserror::Error;

#[cfg(test)]
mod tests;

#[derive(Error, Debug)]
enum AppError {
    #[error("Error starting the server.")]
    Server(#[from] IoError),
}

#[main]
async fn main() -> Result<(), AppError> {
    HttpServer::new(|| {
        App::new().route("/", get().to(async || HttpResponse::Ok().body("Hello World!")))
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await?;

    Ok(())
}
