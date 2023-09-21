use mysql::{params, Params};
use mysql::prelude::Queryable;
use rocket::futures::StreamExt;
use rocket::serde::json::Json;
use crate::db::get_connect;
use crate::structures::base_responser::{BaseResponse, Message};
use crate::structures::user_body::{UserBody, UserPost};
use crate::utils::md5::md5;

#[get("/")]
pub fn index() -> Json<Vec<UserBody>> {
    let mut conn = get_connect();
    let results: Vec<UserBody> = conn.query::<UserBody, String>("SELECT * FROM `user`".to_string())
        .expect("query user list failure")
        .iter()
        .map(|row| UserBody { id: row.id, name: row.name.clone(), password: row.password.clone(), root: row.root })
        .collect();
    // format!("{:?}", results)
    Json(results)
}

#[post("/", data="<body>")]
pub fn login(body: Json<UserPost>) -> Json<Vec<UserBody>> {
    let name = body.name.clone();
    let password = md5(body.password.clone());

    let mut conn = get_connect();
    let sql = String::from(
       concat!(
       "SELECT * FROM `user` ",
       "WHERE ",
       "name=:name AND password=:password;"
       )
    );
    let results: Vec<UserBody> = conn.exec::<UserBody, String, Params>(sql, params! { "name" => name, "password" => password })
        .expect("Get user info failure..")
        .iter()
        .map(|row| {
            UserBody { id: row.id, name: row.name.clone(), password: row.password.clone(), root: row.root }
        }).collect();

    // format!("{:?}", results)
    Json(results)
}

#[put("/", data="<body>")]
pub fn add(body: Json<UserPost>) -> Json<BaseResponse> {
    let name = body.name.clone();
    let password = md5(body.password.clone());

    let mut conn = get_connect();
    let sql = String::from(
      concat!(
      "INSERT INTO `user`(`name`, `password`) ",
      "VALUE ",
      "(:name, :password)"
      )
    );
    let mut ok = true;
    let _results = conn.exec::<UserBody, String, Params>(sql, params! { "name" => name, "password" => password })
        .unwrap_or_else(|_| { ok = false; vec![] });

    if ok {
        Json(BaseResponse { ok: true, msg: Message::None })
    } else {
        Json(BaseResponse { ok: false, msg: Message::MSG(String::from("add failure")) })
    }
}

#[delete("/<id>")]
pub fn delete(id: i32) -> Json<BaseResponse> {
    let mut conn = get_connect();
    let sql = String::from(
      concat!(
      "DELETE FROM `user` ",
      "WHERE `id`=:id;"
      )
    );
    match conn.exec_drop(sql, params! { "id" => id }) {
        Ok(_) => Json(BaseResponse { ok: true, msg: Message::None }),
        _ => Json(BaseResponse { ok: false, msg: Message::MSG(String::from("delete failure")) })
    }
}