
use chrono::{
    Utc,
    Date,
    Datelike
};
use rocket::{
    serde::json::Json,
    State
};
use diesel::prelude::*;

use crate::{
    dto::cb_response::ValCurs,
    form_param::date::NaiveDateForm,
    model::currency::Currency,
    configuration::source::Source,
    PgConn,
    schema::currency
};

#[post("/sync_by_date/<date>")]
pub async fn sync_by_date(connection: PgConn, source: &State<Source>, date: NaiveDateForm) -> Json<u32> {
    let date = Date::<Utc>::from_utc(date.0, Utc).and_hms(0, 0, 0);
    let currency_count: i64 = connection
        .run(move |conn| currency::table.filter(currency::date.eq(date)).count().get_result::<i64>(conn))
        .await
        .expect("Cant check currency existence");

    if currency_count > 0 {
        return Json(currency_count as u32);
    }

    let request_url = format!("{}/XML_daily.asp?date_req={:02}/{:02}/{}", source.cb_url, date.day(), date.month(), date.year());
        
    let response = reqwest::get(request_url).await.expect("Cb request failed").text().await.expect("Empty response").replace(",", ".");

    let response: ValCurs = serde_xml_rs::from_str(&*response).expect("Deserialization fail");

    let valutes_count = response.Valute.len() as u32;

    let currency = response.Valute.into_iter().map(|x|Currency::from_valute(x, date)).collect::<Vec<_>>();

    connection
        .run(move |conn| {
            diesel::insert_into(currency::table)
                .values(&currency)
                .execute(conn)
                .expect("Connection is broken");
        })
        .await;

    return Json(valutes_count)
    
    //let single_currency_info = SingleCurrencyInfo { test: source.cb_url.clone() };
    //Json(single_currency_info)
}