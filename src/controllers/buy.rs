use crate::templates;
use actix_identity::Identity;
use actix_web::Responder;

pub fn carriages(identity: Identity) -> impl Responder {
    templates::Carriages {
        username: identity.identity(),
        carriages: Vec::new(),
    }
}

pub fn ticket() -> impl Responder {
    "".to_string()
}
