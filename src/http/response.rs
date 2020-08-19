use serde::Serialize;

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
    pub page: i64,
    pub size: i64,
    pub count: u64,
    pub list: T,
}
