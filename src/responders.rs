use crate::controllers;
use actix_web::{web, Either, HttpResponse, Responder};
use core::fmt::Debug;
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
}

pub fn timetable(
    route: web::Query<Route>,
    pool: web::Data<Pool>,
) -> Either<impl Responder, impl Responder> {
    match (&route.from, &route.to) {
        (Some(from), Some(to)) => Either::A(controllers::timetable::voyages(pool, from, to)),
        _ => Either::B(controllers::timetable::choose_route(
            pool,
            route.from.as_deref(),
            route.to.as_deref(),
        )),
    }
}

pub fn buy() -> impl Responder {
    unimplemented!()
}
pub fn board() -> impl Responder {
    controllers::board::choose_station()
}

pub fn login() -> impl Responder {
    controllers::login::login()
}

pub fn register() -> impl Responder {
    controllers::register::register()
}

pub fn account(pool: web::Data<Pool>) -> impl Responder {
    controllers::account::account(pool)
}

pub fn admin() -> impl Responder {
    unimplemented!()
}

pub fn error404() -> impl Responder {
    HttpResponse::NotFound().body("404 Not Found")
}
