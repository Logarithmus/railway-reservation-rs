use crate::models::{CarriageType, IdCarriageType, IdStation, Station};
use crate::responders::AdminParams;
use crate::responders::Pool;
use crate::templates;
use actix_identity::Identity;
use actix_web::web;
use actix_web::Responder;
use diesel::prelude::*;

fn get_all_stations(pool: &web::Data<Pool>) -> Result<Vec<String>, diesel::result::Error> {
    use crate::schema::station::dsl::*;
    let id_stations = station.order(name).load::<IdStation>(&pool.get().unwrap());
    id_stations.map(|st| st.iter().map(|s| s.name.clone()).collect())
}

fn get_all_carriage_types(pool: &web::Data<Pool>) -> Result<Vec<String>, diesel::result::Error> {
    use crate::schema::carriage_type::dsl::*;
    let id_carriage_types = carriage_type
        .order(name)
        .load::<IdCarriageType>(&pool.get().unwrap());
    id_carriage_types.map(|st| st.iter().map(|s| s.name.clone()).collect())
}

pub fn admin(pool: web::Data<Pool>, identity: Identity, params: AdminParams) -> impl Responder {
    let (is_station_success, is_station_error) = {
        use crate::schema::station::dsl::*;
        let station_params = (
            params.old_station_name.as_ref(),
            params.new_station_name.as_ref(),
            params.station_action.as_ref(),
        );
        match station_params {
            (None, None, None) => (false, false),
            (Some(old), _, Some(action)) if action == "create" => {
                let result = diesel::insert_into(station)
                    .values(Station {
                        name: old.to_string(),
                    })
                    .execute(&pool.get().unwrap());
                match result {
                    Ok(_) => (true, false),
                    Err(_) => (false, true),
                }
            }
            (Some(old), _, Some(action)) if action == "delete" => {
                let result =
                    diesel::delete(station.filter(name.eq(old))).execute(&pool.get().unwrap());
                match result {
                    Ok(_) => (true, false),
                    Err(_) => (false, true),
                }
            }
            (Some(old), Some(new), Some(action)) if action == "rename" => {
                let result = diesel::update(station.filter(name.eq(old)))
                    .set(name.eq(new))
                    .execute(&pool.get().unwrap());
                match result {
                    Ok(_) => (true, false),
                    Err(_) => (false, true),
                }
            }
            _ => (false, true),
        }
    };
    let (is_carriage_type_success, is_carriage_type_error) = {
        use crate::schema::carriage_type::dsl::*;
        let carriage_type_params = (
            params.old_carriage_type.as_ref(),
            params.new_carriage_type.as_ref(),
            params.carriage_type_action.as_ref(),
        );
        match carriage_type_params {
            (None, None, None) => (false, false),
            (Some(old), _, Some(action)) if action == "create" => {
                let result = diesel::insert_into(carriage_type)
                    .values(CarriageType {
                        name: old.to_string(),
                        seats_count: 56,
                    })
                    .execute(&pool.get().unwrap());
                match result {
                    Ok(_) => (true, false),
                    Err(_) => (false, true),
                }
            }
            (Some(old), _, Some(action)) if action == "delete" => {
                let result = diesel::delete(carriage_type.filter(name.eq(old)))
                    .execute(&pool.get().unwrap());
                match result {
                    Ok(_) => (true, false),
                    Err(_) => (false, true),
                }
            }
            (Some(old), Some(new), Some(action)) if action == "rename" => {
                let result = diesel::update(carriage_type.filter(name.eq(old)))
                    .set(name.eq(new))
                    .execute(&pool.get().unwrap());
                match result {
                    Ok(_) => (true, false),
                    Err(_) => (false, true),
                }
            }
            _ => (false, true),
        }
    };

    let stations = get_all_stations(&pool).unwrap_or_default();
    let carriage_types = get_all_carriage_types(&pool).unwrap_or_default();
    templates::Admin {
        username: identity.identity(),
        stations,
        carriage_types,
        is_station_success,
        is_station_error,
        is_carriage_type_success,
        is_carriage_type_error,
    }
}
