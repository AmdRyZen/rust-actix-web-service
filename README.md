<br>

<p align="center">
<img src="https://picb.zhimg.com/v2-cb1db68b184ed26bc6e2ff0b3108a827_1440w.jpg?source=172ae18b" alt="Rust">
</p>

<p align="center">高性能 • 轻量级 • 命令行 • Tokio异步IO</p>

# RUST
rust-actix-web-service

## 实现说明

基于 actix-web actixactor框架和Tokio 异步IO系统之上构建的高级Web框架部分 以及对其他组件的集成， 包括但不限于(MySQL Redis elasticsearch Middleware  jsonwebtoken等)

## 环境要求

需要 Rust1.42+, actix_web3.0


## 框架定位

绝对性能优先 基于Tokio异步IO的无栈协成

## 安装运行

```
sudo sh start.sh [dev|prod|build|linux]
```
服务器启动默认端口为 8000


## 性能测试
One of the fastest web frameworks available according to the TechEmpower Framework Benchmark.


## 使用文档

[文档地址](https://actix.rs/docs/)

##如果一切顺利，运行到最后你将看到如下的输出：
```
============2020-08-22 00:20:12===========
~> socket http://127.0.0.1:8000/
[Running 'cargo run']
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/rust-actix-web-service`
```


## License

Apache License Version 2.0, http://www.apache.org/licenses/