use crate::models::{IdUser, User};
use crate::responders::Pool;
use crate::templates;
use actix_web::{web, Responder};
use diesel::prelude::*;

pub fn account(pool: web::Data<Pool>) -> impl Responder {
    use crate::schema::user::dsl::*;
    let current_user: IdUser = user
        .find(1)
        .first(&pool.get().unwrap())
        .expect("Failed to get current user from database");
    let current_user = User {
        email: current_user.email,
        pass: current_user.pass,
        is_admin: current_user.is_admin,
    };
    templates::Account { user: current_user }
}
