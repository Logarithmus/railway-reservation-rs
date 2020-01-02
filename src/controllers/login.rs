use crate::templates;
use actix_identity::Identity;

use actix_web::Responder;

pub fn enter_credentials(is_wrong_login: bool, is_wrong_pass: bool) -> impl Responder {
    templates::Login {
        is_wrong_login,
        is_wrong_pass,
    }
}
