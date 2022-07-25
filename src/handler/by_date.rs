use chrono::{
    Utc,
    Date
};
use rocket::serde::json::Json;
use diesel::prelude::*;

use crate::{
    form_param::date::NaiveDateForm,
    model::currency::Currency,
    PgConn,
    schema::currency
};

#[get("/by_date/<date>")]
pub async fn by_date(connection: PgConn, date: NaiveDateForm) -> Json<Vec<Currency>> {
    let today = Date::<Utc>::from_utc(date.0, Utc).and_hms(0, 0, 0);
    connection
        .run(move |conn| currency::table.filter(currency::date.eq(today)).load::<Currency>(conn))
        .await
        .map(Json)
        .expect("Failed to fetch currency")
}