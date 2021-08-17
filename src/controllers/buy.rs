use crate::models::Carriage;
use crate::responders::{BuyParams, Pool};
use crate::templates;
use actix_identity::Identity;
use actix_web::{web, Responder};
use diesel::prelude::*;

pub fn carriages(pool: web::Data<Pool>, mail: String, params: BuyParams) -> impl Responder {
    //let carriages = diesel::sql_query().expect("Failed to retrieve carriages");
    templates::Carriages {
        username: Some(mail),
        carriages: Vec::<Carriage>::new(),
    }
}

pub fn ticket() -> impl Responder {
    "".to_string()
}
