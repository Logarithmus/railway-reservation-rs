use crate::templates;
use actix_web::Responder;

pub fn admin() -> impl Responder {
    templates::Admin {
        stations: Vec::new(),
    }
}
