#[macro_use] extern crate rocket;

pub mod structures;
pub mod db;
pub mod routes;
pub mod utils;

use crate::db::init;
use crate::routes::user::{ index, login, add, delete };

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    init();
    let _rocket = rocket::build()
        .mount("/user", routes![index, login, add, delete])
        .launch()
        .await?;

    Ok(())
}

