use actix_web::{get, HttpResponse};


#[get("/")]
pub async fn test_route() -> HttpResponse {
    HttpResponse::Ok()
        .body("Hello World!")
}
