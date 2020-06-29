use actix_web::{HttpRequest, web, HttpResponse, get};
use serde::{Deserialize, Serialize};
// serde_json::Result;
use mysql::*;
use mysql::prelude::*;
//use serde_json::json;
use urlqstring::QueryParams;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct List {
    id: i32,
    status: i32,
    name: String,
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

#[get("/test")]
pub(crate) async fn test(
) -> HttpResponse {
    loop {
        println!("loop");
    }
    //HttpResponse::Ok().body("hello!")
}