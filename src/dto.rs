use diesel::mysql::types::Datetime;
use diesel::sql_types::Decimal;
use diesel::Queryable;

#[derive(Debug, Queryable)]
pub struct User {
    pub email: String,
    pub password: String,
    pub is_admin: bool,
}

#[derive(Debug, Queryable)]
pub struct Station {
    pub id: u32,
    pub name: String,
}

impl Station {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Queryable)]
pub struct Voyage {
    pub train_num: String,
    pub departure_absolute: Datetime,
    pub late_by: Datetime,
}

impl Voyage {
    pub fn new(train_num: &str, departure_absolute: Datetime) -> Self {
        Self {
            train_num: train_num.to_string(),
            departure_absolute,
            late_by: Datetime::default(),
        }
    }
}

#[derive(Debug, Queryable)]
pub struct Ticket {
    pub user_id: u32,
    pub passenger_id: u32,
    pub carriage_station_id: u32,
    pub seat_num: u8,
    pub price: Decimal,
    pub sell_datetime: Datetime,
    pub cancel_datetime: Option<Datetime>,
}

#[derive(Debug, Queryable)]
pub struct Passenger {
    pub passport_num: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Queryable)]
pub struct Carriage {
    pub voyage_id: u32,
    pub carriage_type_id: u32,
    pub number: u8,
}
