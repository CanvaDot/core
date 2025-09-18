#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::match_like_matches_macro)]

#[cfg(not(feature = "coverage"))]
use std::io::Error as IoError;

#[cfg(not(feature = "coverage"))]
use ::{
    actix_web::{App, HttpServer, main},
    thiserror::Error,
};
#[cfg(not(feature = "coverage"))]
use routes::test_route::test_route;

mod routes;

#[derive(Error, Debug)]
#[cfg(not(feature = "coverage"))]
enum AppError {
    #[error("Error starting the server.")]
    Server(#[from] IoError),
}

#[main]
#[cfg(not(feature = "coverage"))]
async fn main() -> Result<(), AppError> {
    HttpServer::new(|| App::new().service(test_route))
        .bind(("0.0.0.0", 8081))?
        .run()
        .await?;

    Ok(())
}

#[cfg(feature = "coverage")]
pub fn main() {}

#[cfg(feature = "coverage")]
#[cfg(test)]
mod tests {
    use crate::main;

    #[test]
    fn main_coverage() {
        main()
    }
}
