
use chrono::{
    Utc,
    Datelike
};
use rocket::{
    serde::json::Json,
    State
};
use diesel::prelude::*;

use crate::{
    dto::cb_response::ValCurs,
    model::currency::Currency,
    configuration::source::Source,
    PgConn,
    schema::currency
};



#[post("/sync")]
pub async fn sync(connection: PgConn, source: &State<Source>) -> Json<u32> {
    let today = Utc::today().and_hms(0, 0, 0);
    let currency_count: i64 = connection
        .run(move |conn| currency::table.filter(currency::date.eq(today)).count().get_result::<i64>(conn))
        .await
        .expect("Cant check currency existence");

    if currency_count > 0 {
        return Json(currency_count as u32);
    }

    let request_url = format!("{}/XML_daily.asp?date_req={:02}/{:02}/{}", source.cb_url, today.day(), today.month(), today.year());
        
    let response = reqwest::get(request_url).await.expect("Cb request failed").text().await.expect("Empty response").replace(",", ".");

    let response: ValCurs = serde_xml_rs::from_str(&*response).expect("Deserialization fail");

    let valutes_count = response.Valute.len() as u32;

    let currency = response.Valute.into_iter().map(|x|Currency::from_valute(x, today)).collect::<Vec<_>>();

    connection
        .run(move |conn| {
            diesel::insert_into(currency::table)
                .values(&currency)
                .execute(conn)
                .expect("Connection is broken");
        })
        .await;

    Json(valutes_count)
}