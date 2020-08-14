use crate::http::response;
use actix_web::{web, Responder};
use std::process::Command;

pub async fn execute() -> impl Responder {
    /* let _output = Command::new("ls")
    .arg("-l")
    .arg("-a")
    .spawn()
    .expect("ls command failed to start");*/

    /*let pwd = Command::new("pwd")
    .spawn()
    .expect("ls command failed to start");*/

    let _ffmpeg = Command::new("ffmpeg")
        .arg("-version")
        .spawn()
        .expect("ls command failed to start");
    println!("{:?}", _ffmpeg);

    web::Json(response::Success {
        code: response::HTTP_OK,
        message: response::HTTP_MSG.to_string(),
        result: response::HTTP_MSG.to_string(),
    })
}
