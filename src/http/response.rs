use serde::{Serialize};

pub const HTTP_OK: i32 = 1;
//pub const HTTP_ERROR: i32 = 0;
pub const HTTP_MSG: &str = "success";

#[derive(Serialize)]
pub struct Success<T> {
    pub code: i32,
    pub message: String,
    pub result: T,
}

#[derive(Serialize)]
pub struct Failed {
    pub code: i32,
    pub message: String,
}

#[derive(Serialize)]
pub struct Result<T> {
    pub page: i32,
    pub size: i32,
    pub count: u32,
    pub list: T,
}