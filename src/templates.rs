use crate::models::*;
use yarte::Template;

#[derive(Template)]
#[template(path = "timetable/choose_route.html")]
pub struct ChooseRoute {
    pub username: Option<String>,
    pub stations: Vec<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub date: Option<String>,
}

#[derive(Template)]
#[template(path = "timetable/voyages.html")]
pub struct Voyages {
    pub username: Option<String>,
    pub date: String,
    pub from: String,
    pub to: String,
    pub voyages: Vec<TimetableVoyage>,
}

#[derive(Template)]
#[template(path = "buy/carriages.html")]
pub struct Carriages {
    pub username: Option<String>,
    pub carriages: Vec<String>,
}

#[derive(Template)]
#[template(path = "buy/ticket.html")]
pub struct Ticket {
    pub username: Option<String>,
    pub ticket: TicketInfo,
}

#[derive(Template)]
#[template(path = "board/choose_station.html")]
pub struct ChooseStation {
    pub username: Option<String>,
    pub stations: Vec<String>,
    pub date: Option<String>,
}

#[derive(Template)]
#[template(path = "board/board.html")]
pub struct Board {
    pub username: Option<String>,
    pub station: String,
    pub date: String,
    pub voyages: Vec<BoardVoyage>,
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct Register {
    pub is_user_exists: bool,
    pub is_registered: bool,
    pub is_not_registered: bool,
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
    pub user: IdUser,
}

#[derive(Template)]
#[template(path = "admin.html")]
pub struct Admin {
    pub username: Option<String>,
    pub stations: Vec<String>,
}
