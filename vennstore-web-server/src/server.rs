

    use tokio::net::TcpListener;

use biblia::route::build_router;


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, build_router()).await.unwrap();
}
}