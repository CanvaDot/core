#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::match_like_matches_macro)]

use std::io::Error as IoError;

use actix_web::{App, HttpServer, main};
use routes::test_route::test_route;
use thiserror::Error;

mod routes;

#[derive(Error, Debug)]
enum AppError {
    #[error("Error starting the server.")]
    Server(#[from] IoError),
}

#[main]
#[cfg(not(feature = "no_coverage"))]
async fn main() -> Result<(), AppError> {
    HttpServer::new(|| App::new().service(test_route))
        .bind(("0.0.0.0", 8081))?
        .run()
        .await?;

    Ok(())
}

#[cfg(feature = "no_coverage")]
pub fn main() {}

#[cfg(feature = "no_coverage")]
mod tests {
    use crate::main;

    #[test]
    fn main_coverage() {
        main()
    }
}
