use crate::templates;
use actix_web::Responder;

pub fn carriages() -> impl Responder {
    templates::Carriages {
        carriages: Vec::new(),
    }
}

pub fn ticket() -> impl Responder {
    "".to_string()
}
