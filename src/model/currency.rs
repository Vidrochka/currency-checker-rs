use chrono::{DateTime, Utc};
use rocket::serde::Serialize;
use diesel::{Insertable, Queryable};

use crate::{schema::currency, dto::cb_response::Valute};

#[derive(Serialize, Queryable, Insertable, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "currency"]
pub struct Currency {
    pub id: uuid::Uuid,
    pub iso_num_code: i32,
    pub iso_char_code: String,
    pub nominal: i32,
    pub value: f64,
    pub name: String,
    pub date: DateTime<Utc>
}

impl Currency {
    pub fn from_valute(valute: Valute, date: DateTime<Utc>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            iso_num_code: valute.NumCode as i32,
            iso_char_code: valute.CharCode,
            nominal: valute.Nominal as i32,
            value: valute.Value,
            name: valute.Name,
            date: date,
        }
    }
}