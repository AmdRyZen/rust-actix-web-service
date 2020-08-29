use actix_web::{web, HttpRequest, Responder};
use crate::http::*;
use crate::service::*;


pub async fn match_list(_pool: web::Data<mysql::Pool>, _req: HttpRequest) -> impl Responder {

    let (count,list) = Match::list(_pool, _req).await;

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