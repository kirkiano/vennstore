use tokio::net::TcpListener;

use vennstore_web_server::routes;


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, routes()).await.unwrap();
}