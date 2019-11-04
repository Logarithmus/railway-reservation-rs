use actix_web::{HttpResponse, Responder};

pub fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}
