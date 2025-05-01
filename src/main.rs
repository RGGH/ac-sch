use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Import the whole api module
mod api;
// Import the specific function for OpenAPI documentation
use api::hello;

#[derive(OpenApi)]
#[openapi(
    paths(api::hello),
    components(schemas(api::MyResponse))
)]
pub struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let openapi = ApiDoc::openapi();
    
    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", openapi.clone())
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
