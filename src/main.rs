mod handler;
mod dto;
mod configuration;
mod model;
pub mod schema;
mod form_param;

use configuration::source::Source;
use rocket::fairing::AdHoc;
use rocket_sync_db_pools::database;

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

#[database("db")]
pub struct PgConn(diesel::PgConnection);


#[rocket::launch]
async fn rocket() -> _ /* Result<()> */ {
    rocket::build()
        .attach(PgConn::fairing())
        .attach(AdHoc::config::<Source>())
        .mount("/", routes![
            ping,
            handler::by_date::by_date,
            handler::today::today,
            handler::sync::sync,
            handler::sync_by_date::sync_by_date
        ])
}

