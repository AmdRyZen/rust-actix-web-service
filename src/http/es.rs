use crate::http::response;
use actix_web::{web, HttpRequest, Responder};
use elasticsearch::*;
extern crate serde_json;
use elasticsearch::{Elasticsearch, SearchParts};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub async fn index(es_client: web::Data<Elasticsearch>) -> impl Responder {
    let ret = es_client
        .index(IndexParts::IndexId("tweets", "3"))
        .body(json!({
            "id": 3,
            "user": "user1",
            "post_date": "2009-11-15T00:00:00Z",
            "message": "Trying out Elasticsearch rust, aa?"
        }))
        .send()
        .await;

    let successful: bool = ret.is_ok();
    //println!("{:?}", successful);

    web::Json(response::Success {
        code: response::HTTP_OK,
        message: response::HTTP_MSG.to_string(),
        result: successful,
    })
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct SearchList {
    id: String,
    user: String,
    post_date: String,
    message: String,
    www: String,
}
pub async fn search(es_client: web::Data<Elasticsearch>, req: HttpRequest) -> impl Responder {
    let page = req.match_info().get("page").unwrap_or("1");
    let size = req.match_info().get("size").unwrap_or("20");
    let message = req
        .match_info()
        .get("message")
        .unwrap_or("Elasticsearch rust");

    let limit_s: i64 = (page.parse::<i64>().unwrap() - 1) * size.parse::<i64>().unwrap();
    let limit_e: i64 = (limit_s + size.parse::<i64>().unwrap()) - 1;

    let ret = es_client
        .search(SearchParts::Index(&["tweets"]))
        .from(limit_s)
        .size(limit_e)
        .body(json!({
            "query": {
                "match": {
                    "message": message.to_string()
                }
            }
        }))
        .send()
        .await;

    let response_body = match ret {
        Ok(ret) => ret.json::<Value>().await,
        Err(err) => Err(err.into()),
    };
    let mut list: Vec<SearchList> = vec![];
    for i in &response_body {
        for d in i["hits"]["hits"].as_array().unwrap() {
            let p = SearchList {
                id: d["_source"]["id"].to_string(),
                user: serde_json::from_str(&d["_source"]["user"].to_string()).unwrap(),
                post_date: serde_json::from_str(&d["_source"]["post_date"].to_string()).unwrap(),
                message: serde_json::from_str(&d["_source"]["message"].to_string()).unwrap(),
                www: serde_json::from_str(&d["_source"]["www"].to_string())
                    .unwrap_or("".to_string()),
            };
            list.push(p);
        }
    }

    web::Json(response::Success {
        code: response::HTTP_OK,
        message: response::HTTP_MSG.to_string(),
        result: response::Result {
            page: page.parse::<i32>().unwrap(),
            size: size.parse::<i32>().unwrap(),
            count: 0,
            list: list,
        },
    })
}
