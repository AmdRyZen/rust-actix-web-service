use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{dev, http, Result};
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;
use structopt::StructOpt;
//use crate::music_api::*;
use crate::auth;
use crate::http::*;
use elasticsearch::{
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    Elasticsearch,
};
use futures::executor;
use mobc::Pool;
use mobc_redis::redis;
use mobc_redis::RedisConnectionManager;
use std::{sync::mpsc, thread};
use url::Url;

#[derive(Debug, StructOpt)]
#[structopt(name = "rust-actix-web-service", about = "rust-actix-web-service")]
pub(crate) struct Opt {
    #[structopt(long, default_value = "0.0.0.0")]
    ip: String,

    #[structopt(short, long, default_value = "8000")]
    port: i32,
}

#[get("/hello")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("hello!")
}

#[get("/stop")]
async fn stop(stopper: web::Data<mpsc::Sender<()>>) -> HttpResponse {
    // make request that sends message through the Sender
    stopper.send(()).unwrap();

    HttpResponse::NoContent().finish()
}

fn render_500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}

// systemfd --no-pid -s http::8000 -- cargo watch -x run
// cargo run
// cargo update
// cargo build --release
// sudo nohup ./target/release/rust-actix-web-service &
pub(crate) async fn start_server(opt: &Opt) -> std::io::Result<()> {
    dotenv().ok();

    //std::env::set_var("RUST_LOG", "actix_web=trace,actix_redis=trace");
    //std::env::set_var("RUST_LOG", "actix_server=debug,actix_web=debug");
    //env_logger::init();

    // create a channel
    let (tx, rx) = mpsc::channel::<()>();

    // mysql
    let url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let pool = mysql::Pool::new(url).unwrap();

    // redis
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL is not set in .env file");
    let client = redis::Client::open(redis_url).unwrap();
    let manager = RedisConnectionManager::new(client);
    let redis_pool = Pool::builder().max_open(100).build(manager);

    // es
    let es_url = env::var("ES_URL").expect("ES_URL is not set in .env file");
    let es_uri = Url::parse(&es_url).unwrap();
    let conn_pool = SingleNodeConnectionPool::new(es_uri);
    let transport = TransportBuilder::new(conn_pool)
        .disable_proxy()
        .build()
        .unwrap();
    let es_client = Elasticsearch::new(transport);
    //println!("{:#?}", client);

    // start server as normal but don't .await after .run() yet
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .data(tx.clone())
            .wrap(middleware::Logger::default())
            .wrap(auth::CheckLogin)
            .wrap(ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, render_500))
            .data(pool.clone())
            .data(redis_pool.clone())
            .data(es_client.clone())
            .service(test)
            .service(hello)
            .service(stop)
            .service(web::resource("/get/{name}").route(web::get().to(get)))
            .service(web::resource("/set/{name}").route(web::get().to(set)))
            .service(web::resource("/list").route(web::get().to(list)))
            .service(web::resource("/insert").route(web::get().to(insert)))
            .service(web::resource("/update").route(web::get().to(update)))
            .service(web::resource("/redis-list").route(web::get().to(get_list)))
            .service(web::resource("/execute").route(web::get().to(execute)))
            .service(web::resource("/es/index").route(web::get().to(index)))
            .service(web::resource("/es/search").route(web::get().to(search)))
            .service(web::resource("/select").route(web::get().to(select)))
            .service(web::resource("/jwt/signing").route(web::get().to(signing)))
            .service(web::resource("/jwt/verification").route(web::get().to(verification)))
            .service(web::resource("/jwt/render_401").route(web::get().to(render_401)))
            .service(web::resource("/match/list").route(web::get().to(match_list)))
            .service(web::resource("/match/curl").route(web::get().to(curl)))
            .service(web::resource("/hashmap").route(web::get().to(hashmap)))
            .service(web::resource("/queue").route(web::get().to(queue)))
            .service(web::resource("/lpush").route(web::get().to(lpush)))
    });

    let env = env::var("ENV").expect("ENV is not set in .env file");
    if env == "dev" {
        server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
            server.listen(l)?
        } else {
            server.bind(format!("{}:{}", opt.ip, opt.port))?
        };
        server.run().await
    } else {
        let server = server.bind(format!("{}:{}", opt.ip, opt.port))?.run();
        // clone the Server handle
        let srv = server.clone();
        thread::spawn(move || {
            // wait for shutdown signal
            rx.recv().unwrap();

            // stop server gracefully
            executor::block_on(srv.stop(true))
        });
        // run server
        server.await
    }
}
