use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use colored::Colorize;
use dotenv::dotenv;
use std::env::{self, var};
use std::io;

use crate::controller::application::application_config;
use crate::utility::application::get_application_key_from_headder;

mod app_data;
mod controller;
mod db;
mod model;
mod utility;

#[get("/")]
async fn index() -> web::Json<String> {
    web::Json("hello world!".to_owned())
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let ip = var("IP").unwrap_or("127.0.0.1".to_string());
    let port = var("PORT")
        .unwrap_or("7777".to_string())
        .parse::<u16>()
        .unwrap();

    println!(
        "{} {}",
        "Starting server on".green().bold(),
        format!("{}:{}", ip, port).cyan().bold()
    );

    let data = app_data::AppData {
        pg_conn: db::db_connection().await,
    };

    HttpServer::new(move || {
        let basic_middleware = HttpAuthentication::basic(get_application_key_from_headder);

        App::new()
            .app_data(web::Data::new(data.clone()))
            // .service(web::scope("/api").service(index))
            .service(
                web::scope("/api")
                    .wrap(basic_middleware)
                    .configure(application_config),
            )
            .wrap(Logger::default())
    })
    .bind((ip, port))?
    .run()
    .await
}
