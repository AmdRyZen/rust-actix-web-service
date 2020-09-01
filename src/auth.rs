use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::HeaderValue;
use actix_web::{http, Error, HttpResponse};
use futures::future::{ok, Either, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

pub struct CheckLogin;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub login: String, // 可选。听众
    pub exp: i64,      // 必须。(validate_exp 在验证中默认为真值)。截止时间 (UTC 时间戳)
    pub iat: i64,      // 可选。发布时间 (UTC 时间戳)
    pub iss: String,   // 可选。发布者
    pub sub: String,   // 可选。标题 (令牌指向的人)
}

impl<S, B> Transform<S> for CheckLogin
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckLoginMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckLoginMiddleware { service })
    }
}
pub struct CheckLoginMiddleware<S> {
    service: S,
}

impl<S, B> Service for CheckLoginMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        // We only need to hook into the `start` for this middleware.
        // let is_logged_in = true; // Change this to see the change in outcome in the browser

        //println!("Hi from start. You requested: {}", req.path());
        let val = HeaderValue::from_static("");
        let token: &HeaderValue = req.headers().get("authorization").unwrap_or(&val);
        let jwt = token.to_str().unwrap();
        let decode_token = decode::<Claims>(
            &jwt,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        );

        let is_logged_in = match decode_token {
            Ok(_c) => true,
            _ => false,
        };

        if is_logged_in {
            Either::Left(self.service.call(req))
        } else {
            // Don't forward to /login if we are already on /login
            if req.path() != "/jwt/verification" {
                Either::Left(self.service.call(req))
            } else {
                //println!("Hi from start. You requested: {}", is_logged_in);
                Either::Right(ok(req.into_response(
                    HttpResponse::Found()
                        .header(http::header::LOCATION, "/jwt/render_401")
                        .finish()
                        .into_body(),
                )))
            }
        }
    }
}
