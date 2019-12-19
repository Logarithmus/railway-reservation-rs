mod responders;

#[macro_use]
extern crate actix_web;

use actix_files::{self, NamedFile};
use actix_web::{middleware, web, App, HttpServer};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::BufReader;

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
    let sys = actix_rt::System::new("railway");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(actix_files::Files::new("/static", "static"))
            .service(actix_files::Files::new("/", "templates").index_file("index.html"))
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
