use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use tungstenite::protocol::Message;
use tungstenite::WebSocket;

//window dimensions (SD)
const WIDTH: usize = 640;
const HEIGHT: usize = 480;

type WsSink = SplitSink<WebSocket<tokio::net::TcpStream>, Message>;
type WsStream = SplitStream<WebSocket<tokio::net::TcpStream>>;

fn main() {
    println!("Hello, world!");
}
