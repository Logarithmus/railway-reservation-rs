use crate::models::IdUser;
use crate::responders::Pool;
use crate::templates;
use actix_identity::Identity;
use actix_web::{web, Responder};

pub fn account(user: IdUser) -> impl Responder {
    templates::Account { user }
}
