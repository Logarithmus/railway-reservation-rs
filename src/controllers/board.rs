use crate::models::IdStation;
use crate::responders::Pool;
use crate::templates;
use actix_identity::Identity;
use actix_web::web;
use actix_web::Responder;
use diesel::prelude::*;

fn get_all_stations(pool: web::Data<Pool>) -> Result<Vec<String>, diesel::result::Error> {
    use crate::schema::station::dsl::*;
    let id_stations = station.load::<IdStation>(&pool.get().unwrap());
    id_stations.map(|s| s.iter().map(|st| st.name.clone()).collect())
}

pub fn choose_station(
    pool: web::Data<Pool>,
    identity: Identity,
    station: Option<&str>,
    date: Option<&str>,
) -> impl Responder {
    templates::ChooseStation {
        username: identity.identity(),
        stations: Vec::new(),
        date: date.map(|s| s.to_string()),
    }
}

pub fn board(
    pool: web::Data<Pool>,
    identity: Identity,
    station: &str,
    date: &str,
) -> impl Responder {
    templates::Board {
        username: identity.identity(),
        voyages: Vec::new(),
        station: station.to_string(),
        date: date.to_string(),
    }
}
