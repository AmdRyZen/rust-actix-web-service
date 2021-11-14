echo "============`date +%F' '%T`==========="

dev()
{
  systemfd --no-pid -s http::8000 -- cargo watch -x run
}
prod()
{
  sudo nohup ./target/release/rust-actix-web-service
}
build()
{
  RUSTFLAGS="-C target-cpu=native" cargo build --release
}
linux()
{
  CROSS_COMPILE=x86_64-linux-musl- cargo build --release --target x86_64-unknown-linux-musl
}


case "$1" in
  dev)
    dev
    ;;
  prod)
    prod
    ;;
  build)
    build
    ;;
  linux)
    linux
    ;;
esac