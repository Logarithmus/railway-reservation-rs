use crate::schema::*;
use chrono::naive::{NaiveDateTime, NaiveTime};
use diesel::sql_types::{Integer, Nullable, Time, Tinyint, Unsigned, Varchar};
use diesel::{Insertable, QueryableByName};

#[derive(QueryableByName, Debug)]
pub struct TimetableVoyage {
    #[sql_type = "Unsigned<Integer>"]
    pub voyage_id: u32,
    #[sql_type = "Unsigned<Integer>"]
    pub train_num: u32,
    #[sql_type = "Varchar"]
    pub train_type: String,
    #[sql_type = "Varchar"]
    pub first_station: String,
    #[sql_type = "Varchar"]
    pub last_station: String,
    #[sql_type = "Varchar"]
    pub from_station: String,
    #[sql_type = "Varchar"]
    pub to_station: String,
    #[sql_type = "Time"]
    pub depart_time: NaiveTime,
    #[sql_type = "Time"]
    pub arrival_time: NaiveTime,
    #[sql_type = "Time"]
    pub on_the_way_time: NaiveTime,
}

#[derive(QueryableByName, Debug)]
pub struct BoardVoyage {
    #[sql_type = "Unsigned<Integer>"]
    pub train_num: u32,
    #[sql_type = "Varchar"]
    pub train_type: String,
    #[sql_type = "Varchar"]
    pub first_station: String,
    #[sql_type = "Varchar"]
    pub last_station: String,
    #[sql_type = "Nullable<Time>"]
    pub arrival_time: Option<NaiveTime>,
    #[sql_type = "Nullable<Time>"]
    pub depart_time: Option<NaiveTime>,
    #[sql_type = "Nullable<Unsigned<Tinyint>>"]
    pub platform_num: Option<u8>,
    #[sql_type = "Nullable<Unsigned<Tinyint>>"]
    pub track_num: Option<u8>,
    #[sql_type = "Nullable<CarriageNumStartMapping>"]
    pub carriage_num_start: Option<CarriageNumStart>,
}

#[derive(Insertable)]
#[table_name = "passenger"]
pub struct Passenger {
    pub passport_num: String,
    pub first_name: String,
    pub last_name: String,
}

impl From<&IdPassenger> for Passenger {
    fn from(other: &IdPassenger) -> Passenger {
        Passenger {
            passport_num: other.passport_num.clone(),
            first_name: other.first_name.clone(),
            last_name: other.last_name.clone(),
        }
    }
}

#[derive(Queryable)]
pub struct IdPassenger {
    pub id: u32,
    pub passport_num: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Insertable)]
#[table_name = "station"]
pub struct Station {
    pub name: String,
}

impl From<&IdStation> for Station {
    fn from(other: &IdStation) -> Station {
        Station {
            name: other.name.clone(),
        }
    }
}

#[derive(Queryable)]
pub struct IdStation {
    pub id: u32,
    pub name: String,
}

#[derive(Queryable)]
pub struct TicketInfo {
    pub id: u32,
    pub passenger: Passenger,
    pub from: Station,
    pub to: Station,
    pub departure_time: NaiveDateTime,
    pub arrival_time: NaiveDateTime,
    pub on_the_road: NaiveDateTime,
}

#[derive(Queryable)]
pub struct IdTicket {
    pub id: u32,
    pub user_id: u32,
    pub passenger_id: u32,
    pub from_station_id: u32,
    pub to_station_id: u32,
    pub carriage_id: u32,
    pub seat_num: u8,
    pub price: u32,
    pub sell_datetime: NaiveDateTime,
    pub cancel_datetime: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "ticket"]
pub struct Ticket {
    pub user_id: u32,
    pub passenger_id: u32,
    pub from_station_id: u32,
    pub to_station_id: u32,
    pub carriage_id: u32,
    pub seat_num: u8,
    pub price: u32,
    pub sell_datetime: NaiveDateTime,
    pub cancel_datetime: Option<NaiveDateTime>,
}

impl From<&IdTicket> for Ticket {
    fn from(other: &IdTicket) -> Ticket {
        Ticket {
            user_id: other.user_id,
            passenger_id: other.passenger_id,
            from_station_id: other.from_station_id,
            to_station_id: other.to_station_id,
            carriage_id: other.carriage_id,
            seat_num: other.seat_num,
            price: other.price,
            sell_datetime: other.sell_datetime,
            cancel_datetime: other.cancel_datetime,
        }
    }
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct User {
    pub email: String,
    pub pass: String,
    pub is_admin: bool,
    pub is_active: bool,
}

impl From<&IdUser> for User {
    fn from(other: &IdUser) -> User {
        User {
            email: other.email.clone(),
            pass: other.pass.clone(),
            is_admin: other.is_admin,
            is_active: other.is_active,
        }
    }
}

#[derive(Queryable)]
pub struct IdUser {
    pub id: u32,
    pub email: String,
    pub pass: String,
    pub is_admin: bool,
    pub is_active: bool,
}

#[derive(Insertable)]
#[table_name = "carriage_type"]
pub struct CarriageType {
    pub name: String,
    pub seats_count: u8,
}

impl From<&IdCarriageType> for CarriageType {
    fn from(other: &IdCarriageType) -> Self {
        Self {
            name: other.name.clone(),
            seats_count: other.seats_count,
        }
    }
}

#[derive(Queryable)]
pub struct IdCarriageType {
    pub id: u8,
    pub name: String,
    pub seats_count: u8,
}

#[derive(Insertable)]
#[table_name = "carriage"]
pub struct Carriage {
    pub voyage_id: u32,
    pub carriage_type_id: u32,
    pub number: u8,
}

#[derive(QueryableByName, Debug)]
pub struct CarriageInfo {
    #[sql_type = "Unsigned<Integer>"]
    pub id: u32,
    #[sql_type = "Nullable<Integer>"]
    pub voyage_id: u32,
    #[sql_type = "Varchar"]
    pub carriage_type_name: String,
    #[sql_type = "Tinyint"]
    pub number: u8,
}
