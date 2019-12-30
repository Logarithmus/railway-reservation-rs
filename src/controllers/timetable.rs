use crate::models::{IdStation, Station, VoyageInfo};
use crate::responders::Pool;
use crate::templates;
use actix_web::web;
use actix_web::Responder;
use chrono::naive::NaiveDateTime;
use diesel::mysql::types::Datetime;
use diesel::prelude::*;

fn get_all_stations(pool: web::Data<Pool>) -> Result<Vec<String>, diesel::result::Error> {
    use crate::schema::station::dsl::*;
    let id_stations = station.load::<IdStation>(&pool.get().unwrap());
    id_stations.map(|s| s.iter().map(|st| st.name.clone()).collect())
}

pub fn choose_route(pool: web::Data<Pool>, from: Option<&str>, to: Option<&str>) -> impl Responder {
    templates::ChooseRoute {
        from: from.map(|s| s.to_string()),
        to: to.map(|s| s.to_string()),
        stations: get_all_stations(pool).unwrap_or_default(),
    }
}

fn get_voyages(pool: web::Data<Pool>, from: &str, to: &str) -> Vec<VoyageInfo> {
    use crate::schema::{
        station::dsl::*, train::dsl::*, train_station::dsl::*, train_type::dsl::*, voyage::dsl::*,
    };
    let voyages: Vec<VoyageInfo> = diesel::sql_query(format!(
        "SELECT train.num as train_num,
            train_type.name as train_type,
            first_st.name as first_station,
            last_st.name as last_station,
            from_st.name as depart_station,
            to_st.name as arrival_station,
            ADDTIME(voyage.departure_datetime_absolute,
                    from_tr_st.departure_datetime_relative) as departure_datetime,
            ADDTIME(voyage.departure_datetime_absolute,
                    to_tr_st.arrival_datetime_relative) as arrival_datetime
        FROM station from_st
        JOIN train_station from_tr_st ON from_st.name = '{from}'
            AND from_tr_st.station_id = from_st.id
            AND from_tr_st.departure_datetime_relative IS NOT NULL
        JOIN station to_st ON to_st.name = '{to}'
        JOIN train_station to_tr_st ON to_tr_st.station_id = to_st.id
            AND to_tr_st.arrival_datetime_relative IS NOT NULL
            AND to_tr_st.arrival_datetime_relative > from_tr_st.departure_datetime_relative

        JOIN train ON from_tr_st.train_id = train.id
            AND to_tr_st.train_id = train.id
        
        JOIN train_station first_tr_st ON first_tr_st.train_id = train.id
            AND first_tr_st.arrival_datetime_relative IS NULL
        JOIN station first_st ON first_tr_st.station_id = first_st.id

        JOIN train_station last_tr_st ON last_tr_st.train_id = train.id
            AND last_tr_st.departure_datetime_relative IS NULL
        JOIN station last_st ON last_tr_st.station_id = last_st.id
        JOIN train_type ON train_type.id = train.train_type_id
        JOIN voyage ON voyage.train_id = train.id
        ORDER BY departure_datetime, arrival_datetime;
",
        from = from,
        to = to
    ))
    .load(&pool.get().unwrap())
    .expect("super query failed!");
    println!("{:#?}", voyages);
    voyages
}

pub fn voyages(pool: web::Data<Pool>, from: &str, to: &str) -> impl Responder {
    templates::Voyages {
        voyages: get_voyages(pool, from, to),
    }
}
