
#[allow(non_snake_case)]
#[derive(rocket::serde::Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ValCurs {
     pub Valute: Vec<Valute>,
}

#[allow(non_snake_case)]
#[derive(rocket::serde::Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Valute {
    pub NumCode: u32,
    pub CharCode: String,
    pub Nominal: u32,
    pub Name: String,
    pub Value: f64,
}