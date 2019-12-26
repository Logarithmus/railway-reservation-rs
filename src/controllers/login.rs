use crate::templates;
use actix_web::Responder;

pub fn login() -> impl Responder {
    templates::Login {
        is_wrong_login: false,
        is_wrong_pass: false,
    }
}
