use crate::templates;
use actix_web::Responder;

pub fn register() -> impl Responder {
    templates::Register {
        is_user_exists: false,
    }
}
