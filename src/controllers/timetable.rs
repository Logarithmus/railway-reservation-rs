use crate::models::{IdStation, TimetableVoyage};
use crate::responders::Pool;
use crate::templates;
use actix_web::web;
use actix_web::Responder;

use diesel::prelude::*;

fn get_all_stations(pool: web::Data<Pool>) -> Result<Vec<String>, diesel::result::Error> {
    use crate::schema::station::dsl::*;
    let id_stations = station.load::<IdStation>(&pool.get().unwrap());
    id_stations.map(|s| s.iter().map(|st| st.name.clone()).collect())
}

pub fn choose_route(
    pool: web::Data<Pool>,
    from: Option<&str>,
    to: Option<&str>,
    date: Option<&str>,
) -> impl Responder {
    templates::ChooseRoute {
        from: from.map(|s| s.to_string()),
        to: to.map(|s| s.to_string()),
        date: date.map(|s| s.to_string()),
        stations: get_all_stations(pool).unwrap_or_default(),
    }
}

fn get_voyages(pool: web::Data<Pool>, from: &str, to: &str, date: &str) -> Vec<TimetableVoyage> {
    let voyages: Vec<TimetableVoyage> = diesel::sql_query(format!(
        "SELECT train.num as train_num,
            train_type.name as train_type,
            first_st.name as first_station,
            last_st.name as last_station,
            from_st.name as from_station,
            to_st.name as to_station,
            TIME(ADDTIME(voyage.departure_datetime_absolute,
                    from_tr_st.departure_time_relative)) as depart_time,
            TIME(ADDTIME(voyage.departure_datetime_absolute,
                    to_tr_st.arrival_time_relative)) as arrival_time,
            TIMEDIFF(
                TIME(ADDTIME(voyage.departure_datetime_absolute,
                    to_tr_st.arrival_time_relative)),
                TIME(ADDTIME(voyage.departure_datetime_absolute,
                from_tr_st.departure_time_relative)) 
            ) as on_the_way_time
        FROM voyage
        JOIN train ON train.id = voyage.train_id
            AND DATE(voyage.departure_datetime_absolute) = '{date}'
        JOIN train_type ON train_type.id = train.train_type_id

        JOIN train_station first_tr_st ON first_tr_st.train_id = train.id
            AND first_tr_st.arrival_time_relative IS NULL
        JOIN station first_st ON first_st.id = first_tr_st.station_id

        JOIN train_station last_tr_st ON last_tr_st.train_id = train.id
            AND last_tr_st.departure_time_relative IS NULL
        JOIN station last_st ON last_st.id = last_tr_st.station_id

        JOIN train_station from_tr_st ON from_tr_st.train_id = train.id
            AND from_tr_st.departure_time_relative IS NOT NULL
        JOIN station from_st ON from_st.id = from_tr_st.station_id
            AND from_st.name = '{from}'

        JOIN train_station to_tr_st ON to_tr_st.train_id = train.id
            AND to_tr_st.arrival_time_relative IS NOT NULL
            AND to_tr_st.arrival_time_relative > from_tr_st.departure_time_relative
        JOIN station to_st ON to_st.id = to_tr_st.station_id
            AND to_st.name = '{to}'

        ORDER BY depart_time, arrival_time;",
        from = from,
        to = to,
        date = date
    ))
    .load(&pool.get().unwrap())
    .expect("Failed to get voyages");
    println!("{:#?}", voyages);
    voyages
}

pub fn voyages(pool: web::Data<Pool>, from: &str, to: &str, date: &str) -> impl Responder {
    templates::Voyages {
        date: date.to_string(),
        from: from.to_string(),
        to: to.to_string(),
        voyages: get_voyages(pool, from, to, date),
    }
}
