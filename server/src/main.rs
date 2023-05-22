use byteorder::{ByteOrder, LittleEndian};
use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use futures_util::SinkExt;
use futures_util::StreamExt;
use std::println;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio_tungstenite::WebSocketStream;
use tungstenite::protocol::Message;

//window dimensions (SD)
const WIDTH: usize = 640;
const HEIGHT: usize = 360;

type WsSink = SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>;
type WsStream = SplitStream<WebSocketStream<tokio::net::TcpStream>>;

async fn handle_client(mut sink: WsSink, mut stream: WsStream, framebuffer: Arc<Mutex<Vec<u32>>>) {
    while let Some(Ok(msg)) = stream.next().await {
        if let Ok(msg) = msg.to_text() {
            println!("{}", msg);
            let mut parts = msg.split_whitespace();
            let cmd = parts.next();
            if let Some("click") = cmd {
                println!("click");
                let x = parts.next().unwrap_or("").parse::<usize>().unwrap_or(0);
                let y = parts.next().unwrap_or("").parse::<usize>().unwrap_or(0);
                if x < WIDTH && y < HEIGHT {
                    let mut buffer = framebuffer.lock().unwrap();
                    buffer[y * WIDTH + x] = 0xFFFFFFFF;
                }
            } else {
                continue;
            }
        }
        // assign bytes in this scope to drop the framebuffer lock as soon as possible
        let bytes = {
            let framebuffer = framebuffer.lock().unwrap();
            let mut bytes = vec![0; framebuffer.len() * 4];
            LittleEndian::write_u32_into(&framebuffer, &mut bytes);
            bytes
        };
        let msg = Message::binary(bytes);
        if (sink.send(msg).await)
            .map_err(|e| println!("Failed to send message: {}", e))
            .is_err()
        {
            break;
        }
    }
}

#[tokio::main]
async fn main() {
    // letting this unwrap() because if it fails, there's no point in continuing
    let listener = {
        match TcpListener::bind("localhost:9001").await {
            Ok(listener) => listener,
            Err(e) => panic!("Failed to bind: {}", e),
        }
    };

    let buffer = Arc::new(Mutex::new(vec![0; WIDTH * HEIGHT]));

    loop {
        if let Ok((stream, _)) = listener.accept().await {
            //TODO: handle unwrap
            let ws_stream = tokio_tungstenite::accept_async(stream).await.unwrap();
            let (sink, stream) = ws_stream.split();
            tokio::spawn(handle_client(sink, stream, Arc::clone(&buffer)));
        }
    }
}
