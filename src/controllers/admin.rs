use crate::templates;
use actix_identity::Identity;
use actix_web::Responder;

pub fn admin(identity: Identity) -> impl Responder {
    templates::Admin {
        username: identity.identity(),
        stations: Vec::new(),
    }
}
