use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
// serde_json::Result;
use mobc_redis::RedisConnectionManager;
use mobc_redis::{redis, Connection};
use mysql::prelude::*;
use mysql::*;
use std::str;
use urlqstring::QueryParams;

#[derive(Serialize)]
struct Success<T> {
    code: i32,
    message: String,
    list: T,
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
}

#[get("/list")]
pub(crate) async fn list(pool: web::Data<mysql::Pool>, req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let _id = query.value("id");
    //println!("{:?}", id);

    let sql = "select id, status, name from t_media_screenshot";
    let sql = sql.to_string() + " order by id desc";

    let mut conn = pool.get_conn().unwrap();
    let result = conn.query_map(
        sql,
        |(id, status, name)| List { id, status, name },
    );

    let list = match result {
        Ok(result) => result,
        Err(_e) => vec![],
    };

    web::Json(Success {
        code: 1,
        message: "success".to_string(),
        list: list,
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
        list: List {
            id: 99,
            status: 1,
            name: s,
        },
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
        list: List {
            id: 99,
            status: 1,
            name: s,
        },
    })
}

#[get("/test")]
pub(crate) async fn test() -> HttpResponse {
    loop {
        println!("loop");
    }
    //HttpResponse::Ok().body("hello!")
}
