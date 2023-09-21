use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(crate="rocket::serde")]
pub struct BaseResponse {
    pub ok: bool,
    pub msg: Message
}

#[derive(Serialize)]
#[serde(crate="rocket::serde")]
pub enum Message {
    None,
    MSG(String)
}