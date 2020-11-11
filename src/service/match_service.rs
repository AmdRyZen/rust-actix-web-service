use actix_web::{web, HttpRequest};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
extern crate serde_json;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Match {
    pub id: i32,
    pub status: i32,
    pub name: String,
    pub pull_url: String,
    pub server_name: String,
    pub created_at: String,
    pub _type: i32,
}
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub age: i64,
    pub email: String,
}

impl Match {
    pub async fn list(_pool: web::Data<mysql::Pool>, _req: HttpRequest) -> (u64, Vec<Match>) {
        let id = _req.match_info().get("id").unwrap_or("1");

        let mut sql = String::from("");
        let mut sql_where = String::from("");
        let mut sql_count = String::from("");

        sql_where.push_str(" where id >= ");
        sql_where.push_str(&id.to_string());

        sql.push_str(
             "select id, status, name, pull_url, server_name, created_at, type from t_media_screenshot ",
         );
        sql.push_str(&sql_where);
        sql.push_str(" order by id desc");

        sql_count.push_str("select count(1) as total from t_media_screenshot ");
        sql_count.push_str(&sql_where);

        let mut conn = _pool.get_conn().unwrap();

        let total: Result<Option<u64>> = conn.query_first(sql_count);
        let count: u64 = match total {
            Ok(total) => total.unwrap(),
            Err(_e) => 0,
        };

        let result = conn.query_map(
            sql,
            |(id, status, name, pull_url, server_name, created_at, _type)| Match {
                id,
                status,
                name,
                pull_url,
                server_name,
                created_at,
                _type,
            },
        );

        let list = match result {
            Ok(result) => result,
            Err(_e) => vec![],
        };

        (count, list)
    }

    pub async fn curl() -> (u32, Vec<User>) {
        let client = reqwest::Client::new();
        let response = client.get("http://127.0.0.1:8000/list")
            .body("the exact body that is sent")
            .send()
            .await
            .unwrap();
        //println!("client: {:?}", client);
        //println!("response: {:?}", response);

        let code = response.status().as_u16();
        let mut list: Vec<User> = vec![];
        if code == 200 {
            let data = response.json::<serde_json::value::Value>().await.unwrap();
            if data["code"] == 1 {
                let data_array = data["result"]["list"].as_array().unwrap();
                for val in data_array {  //开始迭代
                    let _id = val.get(&"id").unwrap().as_i64().unwrap();
                    let _val_name = val.get("name").unwrap().as_str().unwrap();
                    let _age = val.get(&"age").unwrap().as_i64().unwrap();
                    let _email = val.get("email").unwrap().as_str().unwrap();

                    let p: User = User {
                        id: _id,
                        name: _val_name.to_string(),
                        age: _age,
                        email: _email.to_string()
                    };
                    list.push(p);
                }
            }
        }
        (0, list)
    }
}
