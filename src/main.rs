mod failure;
mod handlers;
mod machines;
mod requests;
mod router;

use std::net::SocketAddr;

use tokio::net::TcpListener;

use crate::router::router;

#[tokio::main]
async fn main() {
    let app = router();

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(address).await.unwrap();
    println!(
        "bit-casino-slots-ms is listening on {}",
        address.to_string()
    );
    axum::serve(listener, app).await.unwrap();
}
