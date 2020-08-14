use crate::http::response;
use actix_web::{web, Responder};
use elasticsearch::*;
use serde_json::{json,Value};

pub async fn index(
    es_client: web::Data<Elasticsearch>
) -> impl Responder {
    let ret = es_client
        .index(IndexParts::IndexId("tweets", "1"))
        .body(json!({
        "id": 1,
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
    let response_body = ret.is_ok();
    // let took = response_body["took"].as_i64().unwrap();
    // for hit in response_body["hits"]["hits"].as_array().unwrap() {
    //     // print the source document
    //     println!("{:?}", hit["_source"]);
    // }

    println!("{:?}", response_body);

    web::Json(response::Success {
        code: response::HTTP_OK,
        message: response::HTTP_MSG.to_string(),
        result: 1,
    })
}
