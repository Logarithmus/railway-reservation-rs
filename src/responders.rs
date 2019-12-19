use actix_files::NamedFile;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct UrlParam {
    filename: String,
}

pub fn template(params: web::Path<UrlParam>) -> actix_web::Result<NamedFile> {
    println!("{}", &params.filename);
    NamedFile::open(&format!("templates/{}.html", &params.filename)).map_err(|e| e.into())
}

pub fn error404() -> impl Responder {
    HttpResponse::NotFound().body("404 Not Found")
}
