use crate::controller::response;
use actix_web::{web, Responder};
use futures::{future::FutureExt, pin_mut, select};
//use tokio::runtime::Runtime;
use std::io::Result;
use std::thread;
use std::time::Duration;

pub async fn select() -> impl Responder {
    let _trest = async_main();
    println!("Hello, world!");

    web::Json(response::Success {
        code: response::HTTP_OK,
        message: response::HTTP_MSG.to_string(),
        result: response::HTTP_MSG.to_string(),
    })
}

async fn async_main() {
    let f1 = function1().fuse();
    let f2 = function2().fuse();

    pin_mut!(f1, f2);

    select! {
        _ = f1 => println!("task one completed first"),
        _ = f2 => println!("task two completed first"),
    }
}

async fn function1() -> Result<()> {
    thread::sleep(Duration::from_secs(2));
    println!("function1 ++++ ");
    Ok(())
}

async fn function2() -> Result<()> {
    println!("function2 ++++ ");
    Ok(())
}
