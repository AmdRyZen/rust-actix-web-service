use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
// serde_json::Result;
use mobc_redis::RedisConnectionManager;
use mobc_redis::{redis, Connection};
use mysql::prelude::*;
use mysql::*;
use std::str;
use serde_json::Value;

#[derive(Serialize)]
struct Success<T> {
    code: i32,
    message: String,
    result: T,
}

#[derive(Serialize)]
struct Failed {
    code: i32,
    message: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct List {
    id: i32,
    status: i32,
    name: String,
    pull_url: String,
    server_name: String,
    created_at: String,
}

pub(crate) async fn list(pool: web::Data<mysql::Pool>, _req: HttpRequest, _info: web::Json<Value>) -> impl Responder {
    let id = _info.get("id").unwrap();

    let mut sql = String::from("");
    sql.push_str("select id, status, name, pull_url, server_name, created_at  from t_media_screenshot ");
    if id.is_string() {
        sql.push_str(" where id = ");
        sql.push_str(&id.to_string());
    }
    sql.push_str(" order by id desc");

    let mut conn = pool.get_conn().unwrap();
    let result = conn.query_map(
        sql,
        |(id, status, name, pull_url, server_name, created_at)| List { id, status, name, pull_url, server_name, created_at },
    );

    let list = match result {
        Ok(result) => result,
        Err(_e) => vec![],
    };

    web::Json(Success {
        code: 1,
        message: "success".to_string(),
        result: list,
    })
}

/*let count : i32 = con.get("my_counter")?;
let count = con.get("my_counter").unwrap_or(0i32);
let k : Option<String> = con.get("missing_key")?;
let name : String = con.get("my_name")?;
let bin : Vec<u8> = con.get("my_binary")?;
let map : HashMap<String, i32> = con.hgetall("my_hash")?;
let keys : Vec<String> = con.hkeys("my_hash")?;
let mems : HashSet<i32> = con.smembers("my_set")?;
let (k1, k2) : (String, String) = con.get(&["k1", "k2"])?;*/
pub(crate) async fn get(
    redis_pool: web::Data<mobc::Pool<RedisConnectionManager>>,
    req: HttpRequest,
) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("name");
    let mut conn = redis_pool.get().await.unwrap();
    // let s: String = redis::cmd("SET").arg("a").arg(1).query_async(&mut conn as &mut Connection).await.unwrap();
    let s: String = redis::cmd("GET")
        .arg(name)
        .query_async(&mut conn as &mut Connection)
        .await
        .unwrap_or("".to_string());

    web::Json(Success {
        code: 1,
        message: "success".to_string(),
        result: s,
    })
}

pub(crate) async fn set(
    redis_pool: web::Data<mobc::Pool<RedisConnectionManager>>,
    req: HttpRequest,
) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("name");
    let mut conn = redis_pool.get().await.unwrap();
    let s: String = redis::cmd("SET")
        .arg(name)
        .arg(name)
        .query_async(&mut conn as &mut Connection)
        .await
        .unwrap();

    web::Json(Success {
        code: 1,
        message: "success".to_string(),
        result: s,
    })
}

#[get("/test")]
pub(crate) async fn test() -> HttpResponse {
    loop {
        println!("loop");
    }
    //HttpResponse::Ok().body("hello!")
}
