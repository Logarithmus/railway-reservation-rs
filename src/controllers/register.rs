use crate::diesel::RunQueryDsl;
use crate::models::User;
use crate::responders::Pool;
use crate::templates;
use actix_web::web;
use actix_web::Responder;
use diesel::prelude::*;

pub fn enter_credentials(is_user_exists: bool) -> impl Responder {
    templates::Register {
        is_user_exists,
        is_registered: false,
        is_not_registered: false,
    }
}

pub fn register(pool: web::Data<Pool>, mail: &str, password: &str) -> impl Responder {
    let secret_key = std::env::var("SECRET_KEY").expect("Argonautica SECRET_KEY not found");
    let hashed_pass = argonautica::Hasher::default()
        .with_password(password)
        .with_secret_key(secret_key.as_str())
        .configure_iterations(64)
        .hash()
        .unwrap();
    println!("{}, len = {}", hashed_pass, hashed_pass.len());
    let new_user = User {
        email: mail.to_string(),
        pass: hashed_pass,
        is_active: true,
        is_admin: false,
    };
    use crate::schema::user::dsl::*;
    let res = diesel::insert_into(user)
        .values(&new_user)
        .execute(&pool.get().unwrap());
    let is_ok = res.map_err(|e| println!("{:?}", e)).is_ok();
    templates::Register {
        is_user_exists: false,
        is_registered: is_ok,
        is_not_registered: !is_ok,
    }
}
