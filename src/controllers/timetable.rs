use crate::models::{Station, Voyage};
use crate::responders::Pool;
use crate::templates;
use actix_web::web;
use actix_web::Responder;
use diesel::mysql::types::Datetime;
use diesel::prelude::*;

fn get_stations(pool: web::Data<Pool>) -> Result<Vec<Station>, diesel::result::Error> {
    use crate::schema::station::dsl::*;
    let id_stations = station.load(&pool.get().unwrap());
    let stations = id_stations.map(|s| s.iter().map(|st| Station::from(st)).collect());
    stations
}

pub fn choose_route(pool: web::Data<Pool>, from: Option<&str>, to: Option<&str>) -> impl Responder {
    let stations = get_stations(pool)
        .unwrap_or_default()
        .iter()
        .map(|st| st.name)
        .collect();
    templates::ChooseRoute {
        from: from.map(|s| s.to_string()),
        to: to.map(|s| s.to_string()),
        stations,
    }
}

fn get_voyages(pool: web::Data<Pool>, from: &str, to: &str) -> Result<Vec<Station>, diesel::result::Error> {
    use crate::schema::{train::dsl::*, voyage::dsl::*, train_type::dsl::*};
    let voyages = voyage.inner_join(train.inner_join(train_type)).load(&pool.get().unwrap());
    let stations = id_stations.map(|s| s.iter().map(|st| Station::from(st)).collect());
    stations
}

pub fn voyages(from: &str, to: &str) -> impl Responder {
    let voyages = 
    templates::Voyages {
        
    }
}
