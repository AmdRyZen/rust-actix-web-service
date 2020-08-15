use crate::http::{response};
use actix_web::{web, Responder};
use elasticsearch::*;
extern crate serde_json;
use serde_json::{json,Value};
use elasticsearch::{Elasticsearch, SearchParts};
use serde::{Deserialize, Serialize};

pub async fn index(
    es_client: web::Data<Elasticsearch>
) -> impl Responder {
    let ret = es_client
        .index(IndexParts::IndexId("tweets", "2"))
        .body(json!({
        "id": 2,
        "user": "kimchy",
        "post_date": "2009-11-15T00:00:00Z",
        "message": "Trying out Elasticsearch, so far so good?"
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
    _id: String,
    _index: String,
}
pub async fn search(
    es_client: web::Data<Elasticsearch>
) -> impl Responder {
    let ret = es_client
        .search(SearchParts::Index(&["tweets"]))
        .from(0)
        .size(10)
        .body(json!({
        "query": {
            "match": {
                "message": "Elasticsearch rust"
            }
        }
    }))
        .send()
        .await;

    let response_body = match ret {
        Ok(ret) => ret.json::<Value>().await,
        Err(err) => Err(err.into()),
    };

    let mut data :Vec<SearchList> = vec![];
    for i in &response_body {
        for d in i["hits"]["hits"].as_array().unwrap() {
            let p = SearchList {
                _id: d["_id"].to_string(),
                _index: d["_index"].to_string(),
            };
            data.push(p);
        }
    }
    //println!("{:?}", data);

    web::Json(response::Success {
        code: response::HTTP_OK,
        message: response::HTTP_MSG.to_string(),
        result: data,
    })
}
