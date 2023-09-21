use mysql::prelude::FromRow;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, FromRow)]
#[serde(crate="rocket::serde")]
pub struct UserBody {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub root: i32,
}

#[derive(Deserialize, Debug, Clone )]
#[serde(crate="rocket::serde")]
pub struct UserPost {
    pub name: String,
    pub password: String
}