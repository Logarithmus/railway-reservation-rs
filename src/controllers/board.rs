use crate::templates;
use actix_web::Responder;

pub fn board() -> impl Responder {
    templates::Board {
        stations: Vec::new(),
    }
}
