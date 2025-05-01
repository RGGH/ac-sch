// tests/api_test.rs
use actix_web::{test, App};
use actix_web::http::StatusCode;
use ac_sch::api::{hello, MyResponse};  // Update with your crate name

#[actix_web::test]
async fn test_hello_endpoint() {
    // Create test application
    let app = test::init_service(
        App::new()
            .service(hello)
    ).await;

    // Create test request
    let req = test::TestRequest::get().uri("/hello").to_request();
    
    // Execute request and verify response
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    
    // Extract and check response body
    let body: MyResponse = test::read_body_json(resp).await;
    assert_eq!(body.message, "Hello from module!");
}
