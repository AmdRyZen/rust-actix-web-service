use actix_web::{HttpRequest, web, Error as AWError, HttpResponse, Result, get};
use serde::{Deserialize, Serialize};
// serde_json::Result;
use mysql::*;
use mysql::prelude::*;
//use serde_json::json;
use actix::prelude::*;
use urlqstring::QueryParams;
use actix_redis::{Command, RedisActor};
use redis_async::resp::RespValue;
use futures::future::join_all;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct List {
    id: i32,
    status: i32,
    name: String,
}

#[derive(Deserialize)]
pub struct CacheInfo {
    key: String,
}

#[get("/list")]
pub(crate) async fn list(
    pool: web::Data<mysql::Pool>,
    req: HttpRequest,
) -> HttpResponse {
    let query = QueryParams::from(req.query_string());
    let id = query.value("id");
    println!("{:?}",id);

    let mut conn = pool.get_conn().unwrap();
    let result = conn
        .query_map(
            "SELECT id, status, name from t_media_screenshot where id= 2",
            |(id, status, name)| {
                List { id, status, name }
            },
        );
    match result {
        // HttpResponse::Ok().content_type("application/json").json(serde_json::to_string(&result).unwrap()),
        Ok(result) => HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&result).unwrap()),
        Err(_e) => HttpResponse::InternalServerError().body("err"),
    }
}

pub(crate) async fn set(
    info: web::Json<CacheInfo>,
    redis: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, AWError> {
    let info = info.into_inner();
    let result=redis.send(Command(resp_array!["set","name",info.key]));
    let res: Vec<Result<RespValue, AWError>> =
        join_all(vec![result].into_iter())
            .await
            .into_iter()
            .map(|item| {
                item.map_err(AWError::from)
                    .and_then(|res| res.map_err(AWError::from))
            })
            .collect();

    // successful operations return "OK", so confirm that all returned as so
    if !res.iter().all(|res| match res {
        Ok(RespValue::SimpleString(x)) if x == "OK" => true,
        _ => false,
    }) {
        Ok(HttpResponse::InternalServerError().finish())
    } else {
        Ok(HttpResponse::Ok().body("successfully cached values"))
    }
}



pub(crate) async fn get(
    redis: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, AWError> {
    let result=redis.send(Command(resp_array!["get","name"])).await?;
    match result{
        Ok(RespValue::BulkString(s)) =>{
            Ok(HttpResponse::Ok().body(s))
        }
        _ => {
            println!("---->{:?}", result);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }

}

#[get("/test")]
pub(crate) async fn test(
) -> HttpResponse {
    loop {
        println!("loop");
    }
    //HttpResponse::Ok().body("hello!")
}