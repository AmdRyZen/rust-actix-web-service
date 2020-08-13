use actix_web::{get, HttpResponse};
use std::process::Command;

#[get("/execute")]
pub(crate) async fn execute() -> HttpResponse {
   /* let _output = Command::new("ls")
        .arg("-l")
        .arg("-a")
        .spawn()
        .expect("ls command failed to start");*/


    /*let pwd = Command::new("pwd")
        .spawn()
        .expect("ls command failed to start");*/


    let ffmpeg = Command::new("ffmpeg")
        .arg("-version")
        .spawn()
        .expect("ls command failed to start");
    println!("{:?}", ffmpeg);

    HttpResponse::Ok().body("hello!")
}