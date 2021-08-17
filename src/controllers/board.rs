use crate::models::{BoardVoyage, IdStation};
use crate::responders::Pool;
use crate::templates;
use actix_identity::Identity;
use actix_web::web;
use actix_web::Responder;
use diesel::prelude::*;

fn get_all_stations(pool: &web::Data<Pool>) -> Result<Vec<String>, diesel::result::Error> {
    use crate::schema::station::dsl::*;
    let id_stations = station.order(name).load::<IdStation>(&pool.get().unwrap());
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
        stations: get_all_stations(&pool).unwrap_or_default(),
        date: date.map(|s| s.to_string()),
    }
}

pub fn board(
    pool: web::Data<Pool>,
    identity: Identity,
    station: &str,
    date: &str,
) -> impl Responder {
    let voyages: Vec<BoardVoyage> = diesel::sql_query(format!(
        "SELECT train.num as train_num,
            train_type.name as train_type,
            first_st.name as first_station,
            last_st.name as last_station,
            TIME(ADDTIME(voyage.departure_datetime_absolute,
                    tr_st.arrival_time_relative)) as arrival_time,
            TIME(ADDTIME(voyage.departure_datetime_absolute,
                    tr_st.departure_time_relative)) as depart_time,
            voyage.platform_num as platform_num,
            voyage.track_num as track_num,
            voyage.carriage_num_start as carriage_num_start
        FROM voyage
        JOIN train ON train.id = voyage.train_id
        JOIN train_type ON train_type.id = train.train_type_id

        JOIN train_station first_tr_st ON first_tr_st.train_id = train.id
            AND first_tr_st.arrival_time_relative IS NULL
        JOIN station first_st ON first_st.id = first_tr_st.station_id

        JOIN train_station last_tr_st ON last_tr_st.train_id = train.id
            AND last_tr_st.departure_time_relative IS NULL
        JOIN station last_st ON last_st.id = last_tr_st.station_id

        JOIN train_station tr_st ON tr_st.train_id = train.id
            AND (DATE(ADDTIME(voyage.departure_datetime_absolute,
                             tr_st.departure_time_relative)) = '{date}' OR

                DATE(ADDTIME(voyage.departure_datetime_absolute,
                             tr_st.arrival_time_relative)) = '{date}'
        )
        JOIN station st ON st.id = tr_st.station_id
            AND st.name = '{station}'

        ORDER BY arrival_time, depart_time;",
        station = station,
        date = date
    ))
    .load(&pool.get().unwrap())
    .expect("Failed to get voyages");

    templates::Board {
        username: identity.identity(),
        voyages,
        station: station.to_string(),
        date: date.to_string(),
    }
}
