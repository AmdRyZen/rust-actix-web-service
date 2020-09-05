use crate::http::*;
use crate::service::*;
use actix_web::{web, HttpRequest, Responder};

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
