use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use bytes::BytesMut;
use futures::future::{ok, Future, Ready};
use futures::stream::StreamExt;
use actix_web::{web, Responder};

use jsonwebtoken::{decode, Validation, DecodingKey};
use serde::{Serialize, Deserialize};
use crate::http::response;

pub struct CheckLogin;

impl<S: 'static, B> Transform<S> for CheckLogin
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckLoginMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckLoginMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    login: String,         // 可选。听众
    exp: i64,          // 必须。(validate_exp 在验证中默认为真值)。截止时间 (UTC 时间戳)
    iat: i64,          // 可选。发布时间 (UTC 时间戳)
    iss: String,         // 可选。发布者
    sub: String,         // 可选。标题 (令牌指向的人)
}

pub struct CheckLoginMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for CheckLoginMiddleware<S>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>
        + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let mut svc = self.service.clone();

        Box::pin(async move {
            let mut body = BytesMut::new();
            let mut stream = req.take_payload();
            while let Some(chunk) = stream.next().await {
                body.extend_from_slice(&chunk?);
            }
            //println!("Hi from start. You requested: {}", req.path());
            let token = req.headers().get("authorization").unwrap();
            let jwt = token.to_str().unwrap();
            let decode_token = decode::<Claims>(&jwt, &DecodingKey::from_secret("secret".as_ref()), &Validation::default());

            let is_ok = match decode_token {
                Ok(_c) => true,
                _ => false
            };
            let res = svc.call(req).await?;
            if is_ok {
                Ok(res)
            } else {
                println!("Hi from start. You requested: {}", "authorization err");
                Ok(res)
            }
        })
    }
}

fn _verification() -> impl Responder {
    return web::Json(response::Success {
        code: response::_HTTP_NO_LOGIN,
        message: response::_HTTP_MSG_NO_LOGIN.to_string(),
        result: false
    });
}
