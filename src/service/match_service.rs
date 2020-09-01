use serde::{Deserialize, Serialize};
use actix_web::{web, HttpRequest};
use mysql::prelude::*;
use mysql::*;
use curl::easy::Easy;
use std::io::{stdout, Write};

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

impl Match
{
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

    pub async fn curl() -> (u32, ()) {
        let mut handle = Easy::new();
        handle.url("127.0.0.1:8000/match/list").unwrap();
        handle.write_function(|data| {
           stdout().write_all(data).unwrap();
           Ok(data.len())
        }).unwrap();
        handle.perform().ok().unwrap();
        let _code = handle.response_code().unwrap();
        let _err = handle.perform().err();

        //println!("{:#?}", _code);
        (_code, ())
    }
}