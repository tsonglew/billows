use actix_web::HttpServer;

mod app;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let server_host = dotenv::var("SERVER_HOST").unwrap_or("127.0.0.1".to_string());
    let server_port = dotenv::var("SERVER_PORT").unwrap_or("8080".to_string());
    let server_location = format!("{}:{}", server_host, server_port);

    HttpServer::new(|| app::app())
        .bind(server_location)?
        .run()
        .await
}
