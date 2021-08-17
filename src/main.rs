mod controllers;
mod models;
mod responders;
mod schema;
mod templates;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::BufReader;

#[macro_use]
extern crate diesel;

fn setup_rustls() -> rustls::ServerConfig {
    let mut cert_file = BufReader::new(File::open("tls/cert.pem").unwrap());
    let mut key_file = BufReader::new(File::open("tls/key.pem").unwrap());
    let cert_chain = certs(&mut cert_file).unwrap();
    let mut keys = pkcs8_private_keys(&mut key_file).unwrap();
    let mut config = rustls::ServerConfig::new(rustls::NoClientAuth::new());
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();
    config
}

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let sys = actix_rt::System::new("railway");
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY not found");
	println!("Creating database connection manager...");
    let manager = ConnectionManager::<MysqlConnection>::new(connspec);
	println!("Creating database connection pool...");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
	println!("Starting server...");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Compress::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(secret_key.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain("localhost")
                    .max_age_time(chrono::Duration::days(1))
                    .secure(true),
            ))
            .service(actix_files::Files::new("static", "static"))
            .route("account", web::get().to(responders::account))
            .route("admin", web::get().to(responders::admin))
            .route("board", web::get().to(responders::board))
            .route("buy", web::get().to(responders::buy))
            .route("/", web::get().to(responders::timetable))
            .route("timetable", web::get().to(responders::timetable))
            .route("logout", web::get().to(responders::logout))
            .service(
                web::resource("login")
                    .route(web::get().to(responders::login_get))
                    .route(web::post().to(responders::login)),
            )
            .service(
                web::resource("register")
                    .route(web::get().to(responders::register_get))
                    .route(web::post().to(responders::register)),
            )
            .default_service(
                web::resource("/")
                    .route(web::get().to(responders::error404))
                    .route(web::to(|| "Error 400")),
            )
    })
    .bind_rustls("0.0.0.0:8443", setup_rustls())?
    .start();

    sys.run()
}
