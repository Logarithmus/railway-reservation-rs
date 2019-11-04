mod responders;

use actix_web::{http::header::ContentEncoding, App, HttpServer};

fn main() {
    //let mut tls_config = rustls::ServerConfig::new(Arc::new(rustls::NoClientAuth));
    //tls_config.set_single_cert(cert_chain: Vec<key::Certificate>, key_der: key::PrivateKey);

    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .route("/hello", actix_web::web::get().to(responders::index))
            .route("/again", actix_web::web::get().to(responders::index2))
            .service(actix_files::Files::new("/", "templates").show_files_listing())
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap();
}
