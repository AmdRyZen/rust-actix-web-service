use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
extern crate serde_json;
use crate::http::response;
use mobc_redis::RedisConnectionManager;
use mobc_redis::{redis, Connection};
use mysql::prelude::*;
use mysql::*;
use std::str;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct List {
    id: i32,
    status: i32,
    name: String,
    pull_url: String,
    server_name: String,
    created_at: String,
    _type: i32,
}

pub async fn insert(pool: web::Data<mysql::Pool>, _req: HttpRequest) -> impl Responder {
    let list = vec![List {
        id: 4,
        status: 1,
        name: "name".to_string(),
        pull_url: "pull_url".to_string(),
        server_name: "server_name".to_string(),
        created_at: "2020-07-23 00:00:00".to_string(),
        _type: 1,
    }];

    let mut conn = pool.get_conn().unwrap();
    let result = conn.exec_batch(
        r"INSERT INTO t_media_screenshot (id, status, name, pull_url, server_name, created_at, type)
          VALUES (:id, :status, :name, :pull_url, :server_name, :created_at, :type)",
        list.iter().map(|p| {
            params! {
                "id" => p.id,
                "status" => p.status,
                "name" => &p.name,
                "pull_url" => &p.pull_url,
                "server_name" => &p.server_name,
                "created_at" => &p.created_at,
                "type" => &p._type,
            }
        }),
    );

    let ret = match result {
        Ok(_result) => true,
        Err(_e) => false,
    };

    web::Json(response::Success {
        code: response::HTTP_OK,
        message: response::HTTP_MSG.to_string(),
        result: ret,
    })
}

pub async fn update(pool: web::Data<mysql::Pool>, _req: HttpRequest) -> impl Responder {
    let list = vec![
        List {
            id: 10,
            status: 1,
            name: "name_update".to_string(),
            pull_url: "pull_url".to_string(),
            server_name: "server_name".to_string(),
            created_at: "2020-07-24 00:00:00".to_string(),
            _type: 1,
        },
        List {
            id: 9,
            status: 1,
            name: "9".to_string(),
            pull_url: "pull_url".to_string(),
            server_name: "server_name".to_string(),
            created_at: "2020-07-24 00:00:00".to_string(),
            _type: 1,
        },
    ];

    let mut conn = pool.get_conn().unwrap();
    let result = conn.exec_batch(
        r"UPDATE t_media_screenshot
        SET status = :status, name = :name, pull_url = :pull_url, server_name = :server_name, created_at = :created_at, type = :type
        WHERE id = :id",
        list.iter().map(|p| params! {
            "status" => p.status,
            "name" => &p.name,
            "pull_url" => &p.pull_url,
            "server_name" => &p.server_name,
            "created_at" => &p.created_at,
            "type" => &p._type,
            "id" => p.id,
        })
    );

    let ret = match result {
        Ok(_result) => true,
        Err(_e) => false,
    };

    web::Json(response::Success {
        code: response::HTTP_OK,
        message: response::HTTP_MSG.to_string(),
        result: ret,
    })
}

pub async fn list(
    pool: web::Data<mysql::Pool>,
    _req: HttpRequest,
    //_info: web::Json<Value>,
) -> impl Responder {
    //let id = _info.get("id").unwrap();
    let id = _req.match_info().get("id").unwrap_or("1");

    let mut sql = String::from("");
    let mut sql_where = String::from("");
    let mut sql_count = String::from("");

    sql_where.push_str(" where id >= ");
    sql_where.push_str(&id.to_string());

    sql.push_str(
        "select id, status, name, pull_url, server_name, created_at, type from t_media_screenshot ",
    );
    sql.push_str(&sql_where);
    sql.push_str(" order by id desc");

    sql_count.push_str("select count(1) as total from t_media_screenshot ");
    sql_count.push_str(&sql_where);

    let mut conn = pool.get_conn().unwrap();

    let total: Result<Option<u32>> = conn.query_first(sql_count);
    let count: u32 = match total {
        Ok(total) => total.unwrap(),
        Err(_e) => 0,
    };

    let result = conn.query_map(
        sql,
        |(id, status, name, pull_url, server_name, created_at, _type)| List {
            id,
            status,
            name,
            pull_url,
            server_name,
            created_at,
            _type,
        },
    );

    let list = match result {
        Ok(result) => result,
        Err(_e) => vec![],
    };

    web::Json(response::Success {
        code: response::HTTP_OK,
        message: response::HTTP_MSG.to_string(),
        result: response::Result {
            page: 1,
            size: 10,
            count: count,
            list: list,
        },
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
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Redislist {
    match_name: String,
    home_team: String,
    away_team: String,
    sport_name: String,
    region_name: String,
}
pub async fn get_list(
    redis_pool: web::Data<mobc::Pool<RedisConnectionManager>>,
    req: HttpRequest,
) -> impl Responder {
    let league_id = req.match_info().get("league_id").unwrap_or("0");
    let page = req.match_info().get("page").unwrap_or("1");
    let size = req.match_info().get("size").unwrap_or("20");
    let sport_id = req.match_info().get("sport_id").unwrap_or("0");

    let mut set_key = String::from("match_list_by_league_id_key_pre_set_");
    set_key.push_str(&sport_id.to_string());
    set_key.push_str("_");
    set_key.push_str(&league_id.to_string());
    set_key.push_str("_");
    set_key.push_str("cn");
    let limit_s: u8 = (page.parse::<u8>().unwrap() - 1) * size.parse::<u8>().unwrap();
    let limit_e: u8 = (limit_s + size.parse::<u8>().unwrap()) - 1;
    let hash_key = String::from("match_list_key_pre_hash_cn");

    let mut conn = redis_pool.get().await.unwrap();
    let s: Vec<String> = redis::cmd("zrange")
        .arg(&set_key)
        .arg(limit_s)
        .arg(limit_e)
        .query_async(&mut conn as &mut Connection)
        .await
        .unwrap_or(vec![]);

    let m: Vec<String> = redis::cmd("hmget")
        .arg(hash_key)
        .arg(s)
        .query_async(&mut conn as &mut Connection)
        .await
        .unwrap_or(vec![]);

    let count = redis::cmd("zcard")
        .arg(&set_key)
        .query_async(&mut conn as &mut Connection)
        .await
        .unwrap_or(0u32);

    let mut list: Vec<Redislist> = vec![];
    for i in &m {
        //println!("{:?}", i);
        let p: Redislist = serde_json::from_str(i).unwrap();
        list.push(p);
    }

    web::Json(response::Success {
        code: response::HTTP_OK,
        message: response::HTTP_MSG.to_string(),
        result: response::Result {
            page: page.parse::<i32>().unwrap(),
            size: size.parse::<i32>().unwrap(),
            count: count,
            list: list,
        },
    })
}

pub async fn get(
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

    web::Json(response::Success {
        code: response::HTTP_OK,
        message: response::HTTP_MSG.to_string(),
        result: s,
    })
}

pub async fn set(
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

    web::Json(response::Success {
        code: response::HTTP_OK,
        message: response::HTTP_MSG.to_string(),
        result: s,
    })
}

#[get("/test")]
pub async fn test() -> HttpResponse {
    loop {
        println!("loop");
    }
    //HttpResponse::Ok().body("hello!")
}
