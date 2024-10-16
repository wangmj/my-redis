use my_redis::server;

#[tokio::main]
async fn main() {
    server::run(9000).await;
}