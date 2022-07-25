#[derive(rocket::serde::Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Source {
    pub cb_url: String,
}