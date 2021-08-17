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
        seats_state -> Unsigned<Bigint>,
        seat_price -> Unsigned<Integer>,
    }
}

table! {
    carriage_type (id) {
        id -> Unsigned<Tinyint>,
        name -> Varchar,
        seats_count -> Unsigned<Tinyint>,
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
        carriage_id -> Unsigned<Integer>,
        from_station_id -> Unsigned<Integer>,
        to_station_id -> Unsigned<Integer>,
        seat_num -> Unsigned<Tinyint>,
        price -> Unsigned<Integer>,
        sell_datetime -> Datetime,
        cancel_datetime -> Nullable<Datetime>,
    }
}

table! {
    train (id) {
        id -> Unsigned<Integer>,
        num -> Unsigned<Integer>,
        train_type_id -> Unsigned<Tinyint>,
    }
}

table! {
    train_station (id) {
        id -> Unsigned<Integer>,
        train_id -> Unsigned<Integer>,
        station_id -> Unsigned<Integer>,
        arrival_time_relative -> Nullable<Time>,
        departure_time_relative -> Nullable<Time>,
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
        pass -> Varchar,
        is_admin -> Bool,
        is_active -> Bool,
    }
}

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, Copy)]
pub enum CarriageNumStart {
    Head,
    Tail,
}

table! {
    use diesel::sql_types::{Tinyint, Unsigned, Integer, Datetime, Time, Nullable};
    use super::CarriageNumStartMapping;
    voyage (id) {
        id -> Unsigned<Integer>,
        train_id -> Unsigned<Integer>,
        departure_datetime_absolute -> Datetime,
        late_by -> Time,
        carriage_num_start -> Nullable<CarriageNumStartMapping>,
        track_num -> Nullable<Unsigned<Tinyint>>,
        platform_num -> Nullable<Unsigned<Tinyint>>,
    }
}

joinable!(carriage_station -> carriage (carriage_id));
joinable!(carriage_station -> station (station_id));
joinable!(ticket -> user (user_id));
joinable!(train_station -> station (station_id));
joinable!(train_station -> train (train_id));
joinable!(voyage -> train (train_id));

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
