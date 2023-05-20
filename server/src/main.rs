use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use futures_util::StreamExt;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio_tungstenite::WebSocketStream;
use tungstenite::protocol::Message;

//window dimensions (SD)
const WIDTH: usize = 640;
const HEIGHT: usize = 480;

type WsSink = SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>;
type WsStream = SplitStream<WebSocketStream<tokio::net::TcpStream>>;

async fn handle_client(mut sink: WsSink, mut stream: WsStream, framebuffer: Arc<Mutex<Vec<u32>>>) {
    todo!();
}

#[tokio::main]
async fn main() {
    // letting this unwrap() because if it fails, there's no point in continuing
    let listener = TcpListener::bind("127.0.0.1:9001").await.unwrap();

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let buffer = Arc::new(Mutex::new(buffer));

    loop {
        if let Ok((stream, _)) = listener.accept().await {
            //TODO: handle unwrap
            let ws_stream = tokio_tungstenite::accept_async(stream).await.unwrap();
            let (sink, stream) = ws_stream.split();
            tokio::spawn(handle_client(sink, stream, Arc::clone(&buffer)));
        }
    }
}
