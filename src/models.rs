use crate::schema::{passenger, station, ticket, user};
use chrono::naive::NaiveDateTime;
use diesel::{Insertable, Queryable};

#[derive(Queryable)]
pub struct Voyage {
    pub train_num: String,
    pub from: String,
    pub to: String,
    pub departure_datetime: NaiveDateTime,
    pub arrival_datetime: NaiveDateTime,
    pub on_the_way: NaiveDateTime,
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
            passport_num: other.passport_num,
            first_name: other.first_name,
            last_name: other.last_name,
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
        Station { name: other.name }
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
    pub departure_station_id: u32,
    pub arrival_station_id: u32,
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
    pub departure_station_id: u32,
    pub arrival_station_id: u32,
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
            departure_station_id: other.departure_station_id,
            arrival_station_id: other.arrival_station_id,
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
    pub pass: Vec<u8>,
    pub is_admin: u8,
}

#[derive(Queryable)]
pub struct IdUser {
    pub id: u32,
    pub email: String,
    pub pass: Vec<u8>,
    pub is_admin: u8,
}
