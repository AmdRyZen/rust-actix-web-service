use crate::controller::*;
use crate::service::*;
use actix_web::{web, HttpRequest, Responder};
use mobc_redis::RedisConnectionManager;
use mobc_redis::{redis, Connection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
extern crate serde_json;

pub async fn match_list(_pool: web::Data<mysql::Pool>, _req: HttpRequest) -> impl Responder {
    let (count, list) = Match::list(_pool, _req).await;

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

pub async fn curl(_pool: web::Data<mysql::Pool>, _req: HttpRequest) -> impl Responder {
    let (_code, _ret) = Match::curl().await;

    let mut is_ok = false;
    if _code == 200 {
        is_ok = true;
    }

    web::Json(response::Success {
        code: response::HTTP_OK,
        message: response::HTTP_MSG.to_string(),
        result: is_ok,
    })
}

pub async fn hashmap() -> impl Responder {
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    // Takes a reference and returns Option<&V>
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", std_service::call(number)),
        _ => println!("Don't have Daniel's number."),
    }

    // `HashMap::insert()` returns `None`
    // if the inserted value is new, `Some(value)` otherwise
    contacts.insert("Daniel", "164-6743");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("Calling Ashley: {}", std_service::call(number)),
        _ => println!("Don't have Ashley's number."),
    }

    contacts.remove(&"Ashley");

    //println!("{:?}", contacts);

    // `HashMap::iter()` returns an iterator that yields
    // (&'a key, &'a value) pairs in arbitrary order.
    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, std_service::call(number));
    }

    web::Json(response::Success {
        code: response::HTTP_OK,
        message: response::HTTP_MSG.to_string(),
        result: contacts,
    })
}

pub async fn queue() -> impl Responder {
    let mut q = queue_service::Queue::new(10).await;
    for i in 0..50 {
        if let Err(error) = q.enqueue(i).await {
            println!("err: {:?}", error);
        }
    }
    println!("q: {:#?}", q);

    for _ in 0..11 {
        if let Some(data) = q.dequeue().await {
            println!("data: {:?}", data);
        } else {
            println!("data: None");
        }
    }

    let _size = q.size().await;
    //println!("size: {:#?}", q);

    web::Json(response::Success {
        code: response::HTTP_OK,
        message: response::HTTP_MSG.to_string(),
        result: _size,
    })
}

#[derive(Debug, Serialize, Deserialize)]
struct Pushdata {
    id: i64,
    name: String,
}
pub async fn lpush(_redis_pool: web::Data<mobc::Pool<RedisConnectionManager>>) -> impl Responder {
    let mut conn = _redis_pool.get().await.unwrap();
    let data = Pushdata {
        id: 10,
        name: "stefano".to_string(),
    };
    let push_data = serde_json::to_string(&data).unwrap();
    //println!("size: {:#?}", push_data);
    let s = redis::cmd("LPUSH")
        .arg("list")
        .arg(push_data)
        .query_async(&mut conn as &mut Connection)
        .await
        .unwrap_or(false);

    //println!("s: {:#?}", s);
    web::Json(response::Success {
        code: response::HTTP_OK,
        message: response::HTTP_MSG.to_string(),
        result: s,
    })
}