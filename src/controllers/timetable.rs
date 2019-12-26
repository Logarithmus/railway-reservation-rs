use crate::dto::Station;
use crate::dto::Voyage;
use crate::templates;
use actix_web::Responder;
use diesel::mysql::types::Datetime;

pub fn choose_route(from: Option<&str>, to: Option<&str>) -> impl Responder {
    let stations = vec![
        "Brest".to_string(),
        "Vitebsk".to_string(),
        "Gomel".to_string(),
        "Grodno".to_string(),
        "Minsk".to_string(),
        "Mogilev".to_string(),
    ];
    templates::ChooseRoute {
        from: from.map(|s| s.to_string()),
        to: to.map(|s| s.to_string()),
        stations,
    }
}

pub fn voyages(from: &str, to: &str) -> impl Responder {
    templates::Voyages {
        voyages: vec![
            Voyage::new(from, Datetime::default()),
            Voyage::new(from, Datetime::default()),
            Voyage::new(to, Datetime::default()),
            Voyage::new(to, Datetime::default()),
        ],
    }
}
