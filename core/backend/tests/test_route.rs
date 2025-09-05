use actix_web::body::MessageBody;
use actix_web::test::{call_service, init_service, TestRequest};
use actix_web::{test, App};

use backend::routes::test_route::test_route;

#[test]
async fn test_route_test() {
    let app = init_service(App::new().service(test_route)).await;
    let req = TestRequest::default().to_request();
    let resp = call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 200u16);

    let body_bytes = resp.into_body()
        .try_into_bytes()
        .expect("Body to be bytes.")
        .into_iter()
        .map(|c| c as char)
        .collect::<String>();

    assert_eq!(body_bytes, "Hello World!");
}
