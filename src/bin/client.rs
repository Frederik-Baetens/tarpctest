use futures::StreamExt;
use futures::stream;
use std::net::SocketAddr;
use tarpc::{client, context, tokio_serde::formats::Json};
use tarpctest::MyProtocolClient;

#[tokio::main]
async fn main() {
    let server_addr = std::env::args()
        .nth(1)
        .expect("pass a valid server socketaddr")
        .parse()
        .unwrap();
    let client = get_client(server_addr).await;
    client.ping(context::current()).await.unwrap();
    stream::iter(0..1000)
        .for_each_concurrent(50, |_| async {
            client.ping(context::current()).await.unwrap();
        })
        .await;
}

async fn get_client(server_addr: SocketAddr) -> MyProtocolClient {
    let mut transport = tarpc::serde_transport::tcp::connect(server_addr, Json::default);
    transport.config_mut().max_frame_length(4294967296);
    MyProtocolClient::new(client::Config::default(), transport.await.unwrap()).spawn()
}
