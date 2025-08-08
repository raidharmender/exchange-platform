use actix_web::{web, App, HttpServer, middleware, HttpResponse, get};
use actix_cors::Cors;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod models;
mod handlers;
mod services;
mod errors;

use config::Config;
use services::order_service::OrderService;
use services::order_book_service::OrderBookService;

// Simple OpenAPI specification
const OPENAPI_SPEC: &str = include_str!("../openapi.json");

#[get("/swagger-ui")]
async fn swagger_ui() -> HttpResponse {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Exchange API - Swagger UI</title>
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@5.3.1/swagger-ui.css" />
    <style>
        html {
            box-sizing: border-box;
            overflow: -moz-scrollbars-vertical;
            overflow-y: scroll;
        }
        *, *:before, *:after {
            box-sizing: inherit;
        }
        body {
            margin:0;
            background: #fafafa;
        }
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5.3.1/swagger-ui-bundle.js"></script>
    <script src="https://unpkg.com/swagger-ui-dist@5.3.1/swagger-ui-standalone-preset.js"></script>
    <script>
        window.onload = function() {
            const ui = SwaggerUIBundle({
                url: '/api-docs/openapi.json',
                dom_id: '#swagger-ui',
                deepLinking: true,
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIStandalonePreset
                ],
                plugins: [
                    SwaggerUIBundle.plugins.DownloadUrl
                ],
                layout: "StandaloneLayout"
            });
        };
    </script>
</body>
</html>
    "#;
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

#[get("/api-docs/openapi.json")]
async fn openapi_spec() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(OPENAPI_SPEC)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting Exchange API server...");

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");

    // Create services
    let order_book = OrderBookService::new();
    
    #[cfg(feature = "database")]
    let order_service = {
        use sqlx::PgPool;
        let pool = PgPool::connect(&config.database.url)
            .await
            .expect("Failed to connect to database");
        OrderService::new(pool, order_book)
    };

    #[cfg(not(feature = "database"))]
    let order_service = OrderService::new(order_book);

    // Create HTTP server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .app_data(web::Data::new(order_service.clone()))
            .service(swagger_ui)
            .service(openapi_spec)
            .service(
                web::scope("/api/v1")
                    .service(handlers::health::health_check)
                    .configure(handlers::orders::configure)
            )
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run();

    info!("Server running on {}:{}", config.server.host, config.server.port);
    info!("Swagger UI available at: http://{}:{}/swagger-ui", config.server.host, config.server.port);

    server.await
} 