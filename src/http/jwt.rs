use crate::http::response;
use actix_web::{web, HttpRequest, Responder};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::prelude::*;
extern crate chrono;
use std::str;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    login: String,         // 可选。听众
    exp: i64,          // 必须。(validate_exp 在验证中默认为真值)。截止时间 (UTC 时间戳)
    iat: i64,          // 可选。发布时间 (UTC 时间戳)
    iss: String,         // 可选。发布者
    sub: String,         // 可选。标题 (令牌指向的人)
}

#[derive(Debug, Serialize, Deserialize)]
struct Jwt {
    jwt: String,
}

pub async fn signing() -> impl Responder {
    let dt = Local::now();
    let exp  = dt.timestamp() + 86400;

    let my_claims = Claims {
        login: "login".to_owned(),
        exp: exp,
        iat: dt.timestamp(),
        iss: "huzhichao".to_owned(),
        sub: "jwt".to_owned()
    };

    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()));
    let jwt = match token {
        Ok(token) => { token },
        Err(_e) => "".to_string(),
    };

    web::Json(response::Success {
        code: response::HTTP_OK,
        message: response::HTTP_MSG.to_string(),
        result: jwt,
    })
}

pub async fn verification(_req: HttpRequest) -> impl Responder {
    let headers = _req.headers();
    let token = headers.get("Authorization").unwrap();
    let jwt = token.to_str().unwrap_or("");

    let decode_token = decode::<Claims>(&jwt, &DecodingKey::from_secret("secret".as_ref()), &Validation::default());
    match decode_token {
        Ok(c) => {
            return web::Json(response::Success {
                code: response::HTTP_OK,
                message: response::HTTP_MSG.to_string(),
                result: c.claims,
            });
        },
        _ => {
            return web::Json(response::Success {
                code: response::HTTP_OK,
                message: response::HTTP_MSG.to_string(),
                result: Claims {
                    login: "err".to_owned(),
                    exp: 0,
                    iat: 0,
                    iss: "".to_owned(),
                    sub: "".to_owned()
                },
            });
        },
    };
}
/*use actix_web::{HttpRequest};

pub trait Check {
    fn ckeck(_req :HttpRequest);
}
pub struct CheckLogin;

impl Check for CheckLogin
{
    fn ckeck(_req: HttpRequest) {
        let headers = _req.headers();
        let token = headers.get("Authorization").unwrap();
        let jwt = token.to_str().unwrap_or("");
        println!("{:#?}", jwt);
        ckeck_jwt();
    }
}
fn ckeck_jwt() {
    /*let headers = _req.headers();
    let token = headers.get("Authorization").unwrap();
    let jwt = token.to_str().unwrap_or("");*/

    let is_ok = false; // 作为返回值 => 必须使用 () 括起来，并不能写 ;
    println!("{:#?}", is_ok);
}*/
