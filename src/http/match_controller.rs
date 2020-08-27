use actix_web::{web, HttpRequest, Responder};
use crate::http::*;
use crate::service::*;


pub async fn match_list(_pool: web::Data<mysql::Pool>, _req: HttpRequest) -> impl Responder {

    let match_list = Match::list().await;

    web::Json(response::Success {
        code: response::HTTP_OK,
        message: response::HTTP_MSG.to_string(),
        result: match_list,
    })
}