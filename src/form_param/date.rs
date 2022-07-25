use chrono::{NaiveDate, ParseError};
use rocket::request::FromParam;

pub struct NaiveDateForm(pub NaiveDate);

impl<'a> FromParam<'a> for NaiveDateForm {
    type Error = ParseError;
  
    fn from_param(param: &'a str) -> Result<Self, Self::Error>{
        match NaiveDate::parse_from_str(&param, "%Y-%m-%d") {
            Ok(date)=> Ok(NaiveDateForm(date)),
            Err(e) =>Err(e),
        }
    }
}