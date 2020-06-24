use actix_web::{HttpRequest, Responder,web, HttpMessage, HttpResponse, get};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use mysql::*;
use mysql::prelude::*;
use serde_json::json;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct List {
    id: i32,
    status: i32,
    name: String,
}

#[get("/list")]
pub(crate) async fn list(
    pool: web::Data<mysql::Pool>,
) -> HttpResponse {
    let mut conn = pool.get_conn().unwrap();
    let result = conn
        .query_map(
            "SELECT id, status, name from t_media_screenshot",
            |(id, status, name)| {
                List { id, status, name }
            },
        );
    match result {
        // HttpResponse::Ok().content_type("application/json").json(serde_json::to_string(&result).unwrap()),
        Ok(result) => HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&result).unwrap()),
        Err(e) => HttpResponse::InternalServerError().body("whatever"),
    }
    //println!("result: {:?}", result);
}