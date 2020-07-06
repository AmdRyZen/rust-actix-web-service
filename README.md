<br>

<p align="center">
<img src="https://actix.rs/img/logo-large.png" alt="Rust">
</p>


# RUST API
rust-actix-web-service

## 实现说明
此实现是为了学习Rust和actix-web框架而写的
rust-actix-web-service

## 环境要求

需要 Rust1.39+, actix_web2.0

## 安装运行

```shell
systemfd --no-pid -s http::8000 -- cargo watch -x run
cargo run
cargo update
cargo build --release
sudo nohup ./target/release/rust-actix-web-service &


Linux 交叉  CROSS_COMPILE=x86_64-linux-musl- cargo build --release --target x86_64-unknown-linux-musl
```
服务器启动默认端口为 8000

## 使用文档

[文档地址]

## License

Apache License Version 2.0, http://www.apache.org/licenses/