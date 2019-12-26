use crate::controllers::*;

use actix_web::{web, Either, HttpResponse, Responder};
use core::fmt::Debug;

use serde::{Deserialize, Deserializer};

use serde::de::IntoDeserializer;

#[derive(Debug, Deserialize)]
pub struct Route {
    from: Option<String>,
    to: Option<String>,
}

pub fn timetable(route: web::Query<Route>) -> Either<impl Responder, impl Responder> {
    println!("{:?}, {:?}", route.from, route.to);
    let from = route
        .from
        .as_ref()
        .and_then(|x| if x == "" { None } else { Some(x) });
    let to = route
        .to
        .as_ref()
        .and_then(|x| if x == "" { None } else { Some(x) });
    match (&from, &to) {
        (Some(from), Some(to)) if from != to => Either::A(timetable::voyages(from, to)),
        _ => Either::B(timetable::choose_route(
            route.from.as_deref(),
            route.to.as_deref(),
        )),
    }
}

pub fn buy() -> impl Responder {
    unimplemented!()
}

pub fn board() -> impl Responder {
    unimplemented!()
}

pub fn login() -> impl Responder {
    unimplemented!()
}

pub fn register() -> impl Responder {
    unimplemented!()
}

pub fn account() -> impl Responder {
    unimplemented!()
}

pub fn admin() -> impl Responder {
    unimplemented!()
}

pub fn error404() -> impl Responder {
    HttpResponse::NotFound().body("404 Not Found")
}
