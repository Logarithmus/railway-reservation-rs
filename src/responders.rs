use crate::controllers;
use crate::models::{IdUser, User};
use actix_identity::Identity;
use actix_web::{web, Either, HttpResponse, Responder};
use core::fmt::Debug;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use serde::Deserialize;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

fn empty_string_as_none<'de, D>(de: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(Option::<String>::deserialize(de)
        .ok()
        .and_then(|x| match x.as_deref() {
            None | Some("") => None,
            _ => x,
        }))
}

#[derive(Debug, Deserialize)]
pub struct Route {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    from: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    to: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    date: Option<String>,
}

pub fn timetable(
    route: web::Query<Route>,
    pool: web::Data<Pool>,
    identity: Identity,
) -> Either<impl Responder, impl Responder> {
    match (&route.from, &route.to, &route.date) {
        (Some(from), Some(to), Some(date)) => Either::A(controllers::timetable::voyages(
            pool, identity, from, to, date,
        )),
        _ => Either::B(controllers::timetable::choose_route(
            pool,
            identity,
            route.from.as_deref(),
            route.to.as_deref(),
            route.date.as_deref(),
        )),
    }
}

pub fn buy() -> impl Responder {
    unimplemented!()
}

#[derive(Debug, Deserialize)]
pub struct BoardParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    station: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    date: Option<String>,
}

pub fn board(
    params: web::Query<BoardParams>,
    pool: web::Data<Pool>,
    identity: Identity,
) -> Either<impl Responder, impl Responder> {
    match (&params.station, &params.date) {
        (Some(station), Some(date)) => {
            Either::A(controllers::board::board(pool, identity, station, date))
        }
        _ => Either::B(controllers::board::choose_station(
            pool,
            identity,
            params.station.as_deref(),
            params.date.as_deref(),
        )),
    }
}

#[derive(Debug, Deserialize)]
pub struct RegisterParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    email: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    password: Option<String>,
}

pub fn register_get() -> impl Responder {
    controllers::register::enter_credentials(false)
}

pub fn register(
    params: web::Form<RegisterParams>,
    pool: web::Data<Pool>,
    identity: Identity,
) -> Either<impl Responder, impl Responder> {
    println!("{:?}", params);
    match (&params.email, &params.password, &identity.identity()) {
        (_, _, Some(username)) => {
            identity.forget();
            Either::B(controllers::register::enter_credentials(false))
        }
        (Some(mail), Some(password), None) => {
            use crate::schema::user::dsl::*;
            let existing_user = user
                .filter(email.eq(mail))
                .first::<IdUser>(&pool.get().unwrap());
            match existing_user {
                Ok(_) => Either::B(controllers::register::enter_credentials(true)),
                Err(_) => Either::A(controllers::register::register(pool, mail, password)),
            }
        }

        _ => Either::B(controllers::register::enter_credentials(false)),
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    email: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    password: Option<String>,
    #[serde(default)]
    remember: bool,
}

pub fn login_get() -> impl Responder {
    controllers::login::enter_credentials(false, false)
}

pub fn login(
    params: web::Form<LoginParams>,
    identity: Identity,
    pool: web::Data<Pool>,
) -> Either<impl Responder, impl Responder> {
    println!("{:?}", params);
    match (&params.email, &params.password, &identity.identity()) {
        (_, _, Some(username)) => {
            identity.forget();
            Either::B(controllers::login::enter_credentials(false, false))
        }
        (Some(mail), Some(password), None) => {
            use crate::schema::user::dsl::*;
            let existing_user = user
                .filter(email.eq(mail))
                .first::<IdUser>(&pool.get().unwrap());
            match existing_user {
                Ok(u) => {
                    let secret_key =
                        std::env::var("SECRET_KEY").expect("Argonautica SECRET_KEY not found");
                    let is_correct = argonautica::Verifier::default()
                        .with_hash(&u.pass)
                        .with_password(password)
                        .with_secret_key(secret_key.as_str())
                        .verify()
                        .unwrap();
                    match is_correct {
                        true => {
                            identity.remember(u.email.clone());
                            Either::A(controllers::account::account(u))
                        }
                        false => Either::B(controllers::login::enter_credentials(false, true)),
                    }
                }
                Err(_) => Either::B(controllers::login::enter_credentials(true, false)),
            }
        }
        _ => Either::B(controllers::login::enter_credentials(false, false)),
    }
}

pub fn account(
    pool: web::Data<Pool>,
    identity: Identity,
) -> Either<impl Responder, impl Responder> {
    use crate::schema::user::dsl::*;
    let current_user = identity.identity().and_then(|mail| {
        user.filter(email.eq(mail))
            .first::<IdUser>(&pool.get().unwrap())
            .ok()
    });
    match current_user {
        Some(u) => Either::A(controllers::account::account(u)),
        None => Either::B(controllers::login::enter_credentials(false, false)),
    }
}

pub fn admin() -> impl Responder {
    unimplemented!()
}

pub fn logout(identity: Identity) -> impl Responder {
    identity.forget();
    HttpResponse::Found()
        .header(actix_web::http::header::LOCATION, "/")
        .finish()
        .into_body()
}

pub fn error404() -> impl Responder {
    HttpResponse::NotFound().body("404 Not Found")
}
