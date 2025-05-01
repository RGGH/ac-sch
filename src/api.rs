use actix_web::{get, web, Responder};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use utoipa::{ToSchema};

#[derive(Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct MyResponse {
    pub message: String,
}

#[utoipa::path(
    get,  // HTTP method (GET)
    path = "/hello",  // The path for this route
    responses(
        (status = 200, description = "Successful greeting", body = MyResponse)  // Response details
    )
)]
#[get("/hello")]
pub async fn hello() -> impl Responder {
    web::Json(MyResponse {
        message: "Hello from module!".to_string(),
    })
}

