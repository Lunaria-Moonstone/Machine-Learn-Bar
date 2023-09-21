use std::env;
use log::info;
use dotenv::dotenv;
use mysql as sql;
use mysql::{Pool, PooledConn};
use once_cell::sync::OnceCell;
use crate::structures::user_body::UserBody;

pub static DB_POOL: OnceCell<Pool> = OnceCell::new();

pub fn init() {
    println!("hello world");
    dotenv().ok();
    init_mysql_pool(
        &format!(
            "mysql://{}:{}@{}:{}/{}",
            env::var("DB_USER").unwrap(),
            env::var("DB_PASSWORD").unwrap(),
            env::var("DB_HOST").unwrap(),
            env::var("DB_PORT").unwrap(),
            env::var("DB_NAME").unwrap()
        )
    );
}

fn init_mysql_pool(db_url: &str) {
    info!("init mysql pool start...");
    info!("{}", format!("the db url is: {}", db_url));
    DB_POOL.set(sql::Pool::new(db_url).expect(&format!("Error connect to {}", db_url)))
        .unwrap_or_else(|_| { info!("try insert pool cell failure!") });
    info!("init mysql pool end...");
}

pub fn get_connect() -> PooledConn {
    info!("get conn from pool...");
    let conn = DB_POOL.get()
        .expect("Error getting pool from OnceCell<Pool>")
        .get_conn()
        .expect("Error get_conn from db poll");
    info!("get conn end...");
    conn
}