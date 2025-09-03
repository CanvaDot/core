use std::io::Error as IoError;

use actix_web::{main, web::get, App, HttpResponse, HttpServer};
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("Error starting the server.")]
    Server(#[from] IoError)
}

#[main]
async fn main() -> Result<(), AppError> {
    HttpServer::new(|| {
        App::new()
            .route("/", get().to(async || HttpResponse::Ok().body("Hello World!")))
    })
        .bind(("0.0.0.0", 8081))?
        .run()
        .await?;

    Ok(())
}
