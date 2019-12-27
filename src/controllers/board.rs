use crate::templates;
use actix_web::Responder;

pub fn choose_station() -> impl Responder {
    templates::ChooseStation {
        stations: Vec::new(),
    }
}

pub fn board() -> impl Responder {
    templates::Board {
        voyages: Vec::new(),
    }
}
