table! {
    carriage (id) {
        id -> Unsigned<Integer>,
        voyage_id -> Unsigned<Integer>,
        carriage_type_id -> Unsigned<Integer>,
        number -> Unsigned<Tinyint>,
    }
}

table! {
    carriage_station (id) {
        id -> Unsigned<Integer>,
        station_id -> Unsigned<Integer>,
        carriage_id -> Unsigned<Integer>,
        seats_state -> Binary,
        seat_price -> Nullable<Decimal>,
    }
}

table! {
    carriage_type (id) {
        id -> Unsigned<Tinyint>,
        name -> Varchar,
        seats_count -> Tinyint,
        seat_price -> Decimal,
    }
}

table! {
    passenger (id) {
        id -> Unsigned<Integer>,
        passport_num -> Char,
        first_name -> Varchar,
        last_name -> Varchar,
    }
}

table! {
    station (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
    }
}

table! {
    ticket (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        passenger_id -> Unsigned<Integer>,
        departure_station_id -> Unsigned<Integer>,
        arrival_station_id -> Unsigned<Integer>,
        carriage_id -> Unsigned<Integer>,
        seat_num -> Unsigned<Tinyint>,
        price -> Unsigned<Integer>,
        sell_datetime -> Datetime,
        cancel_datetime -> Nullable<Datetime>,
    }
}

table! {
    train (id) {
        id -> Unsigned<Integer>,
        num -> Varchar,
        train_type_id -> Unsigned<Tinyint>,
    }
}

table! {
    train_station (id) {
        id -> Unsigned<Integer>,
        train_id -> Unsigned<Integer>,
        station_id -> Unsigned<Integer>,
        arrival_datetime_relative -> Nullable<Datetime>,
        departure_datetime_relative -> Nullable<Datetime>,
    }
}

table! {
    train_type (id) {
        id -> Unsigned<Tinyint>,
        name -> Varchar,
    }
}

table! {
    user (id) {
        id -> Unsigned<Integer>,
        email -> Varchar,
        pass -> Binary,
        is_admin -> Unsigned<Tinyint>,
    }
}

#[derive(diesel_derive_enum::DbEnum, Debug)]
pub enum TrainEnd {
    Head,
    Tail,
}

table! {
    use super::TrainEndMapping;
    use diesel::sql_types::{Unsigned, Integer, Tinyint, Datetime, Nullable};
    voyage (id) {
        id -> Unsigned<Integer>,
        train_id -> Unsigned<Integer>,
        departure_datetime_absolute -> Datetime,
        late_by -> Nullable<Datetime>,
        carriage_num_start -> Nullable<TrainEndMapping>,
        track_num -> Nullable<Tinyint>,
        platform_num -> Nullable<Tinyint>,
    }
}

joinable!(carriage_station -> carriage (carriage_id));
joinable!(carriage_station -> station (station_id));
joinable!(ticket -> carriage (carriage_id));
joinable!(ticket -> user (user_id));
joinable!(train_station -> station (station_id));
joinable!(train_station -> train (train_id));
joinable!(voyage -> train (train_id));
joinable!(train -> train_type (train_type_id));

allow_tables_to_appear_in_same_query!(
    carriage,
    carriage_station,
    carriage_type,
    passenger,
    station,
    ticket,
    train,
    train_station,
    train_type,
    user,
    voyage,
);
