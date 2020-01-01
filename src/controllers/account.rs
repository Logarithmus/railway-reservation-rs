use crate::models::{IdUser, User};
use crate::responders::Pool;
use crate::templates;
use actix_web::{web, Responder};
use diesel::prelude::*;

pub fn account(pool: web::Data<Pool>) -> impl Responder {
    use crate::schema::user::dsl::*;
    let id_user: IdUser = user
        .find(1)
        .first(&pool.get().unwrap())
        .expect("Failed to get current user from database");
    templates::Account {
        user: User::from(&id_user),
    }
}
