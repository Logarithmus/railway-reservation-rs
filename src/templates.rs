use crate::models::{TicketInfo, User, VoyageInfo};
use yarte::Template;

#[derive(Template)]
#[template(path = "timetable/choose_route.html")]
pub struct ChooseRoute {
    pub stations: Vec<String>,
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Template)]
#[template(path = "timetable/voyages.html")]
pub struct Voyages {
    pub voyages: Vec<VoyageInfo>,
}

#[derive(Template)]
#[template(path = "buy/carriages.html")]
pub struct Carriages {
    pub carriages: Vec<String>,
}

#[derive(Template)]
#[template(path = "buy/ticket.html")]
pub struct Ticket {
    pub ticket: TicketInfo,
}

#[derive(Template)]
#[template(path = "board/choose_station.html")]
pub struct ChooseStation {
    pub stations: Vec<String>,
}

#[derive(Template)]
#[template(path = "board/board.html")]
pub struct Board {
    pub voyages: Vec<VoyageInfo>,
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct Register {
    pub is_user_exists: bool,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct Login {
    pub is_wrong_login: bool,
    pub is_wrong_pass: bool,
}

#[derive(Template)]
#[template(path = "account.html")]
pub struct Account {
    pub user: User,
}

#[derive(Template)]
#[template(path = "admin.html")]
pub struct Admin {
    pub stations: Vec<String>,
}
