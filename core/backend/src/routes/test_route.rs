use actix_web::{HttpResponse, get};

#[get("/")]
pub async fn test_route() -> HttpResponse {
    HttpResponse::Ok().body("Hello World!")
}
