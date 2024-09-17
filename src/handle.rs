
use actix_web::{error, web, App, HttpResponse, HttpServer};
use macro_lib::generate_services;
// use crate::handler;

#[actix_web::main]
pub async fn main(port: u16) -> std::io::Result<()> {
    log::info!("Server is running on port {}", port);
    generate_services!()
    .bind(("0.0.0.0", port))?
    .run()
    .await
}