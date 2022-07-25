use chrono::Utc;
use rocket::serde::json::Json;
use diesel::prelude::*;

use crate::{
    model::currency::Currency,
    PgConn,
    schema::currency
};

#[get("/today")]
pub async fn today(connection: PgConn) -> Json<Vec<Currency>> {
    let today = Utc::today().and_hms(0, 0, 0);
    connection
        .run(move |conn| currency::table.filter(currency::date.eq(today)).load::<Currency>(conn))
        .await
        .map(Json)
        .expect("Failed to fetch currency")
}