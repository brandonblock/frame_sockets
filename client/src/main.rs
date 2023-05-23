mod window;
use crate::window::DrawingWindow;

use futures::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::time::sleep;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    //all setup steps panic on error

    // arrange window
    let mut window = DrawingWindow::new().unwrap();

    // create socket connection
    let url = url::Url::parse("ws://localhost:9001").unwrap();
    let (socket, response) = connect_async(url).await.unwrap();
    let (mut write, mut read) = socket.split();

    println!(
        "Connected to the server, status code: {}",
        response.status()
    );

    // handle mouse and render events in the main loop
    while !window.is_exited() {
        //total forced-sync hack to get this stood up using a read timeout on the async socket
        sleep(std::time::Duration::from_nanos(1)).await;

        //read framebuffers from the socket for a very small amount of time, if any part isn't Ok or Some, keep on truckin'
        if let Ok(Some(Ok(msg))) = timeout(Duration::from_nanos(1), read.next()).await {
            window.process_message(msg);
        }
        // send our mouse position to the server
        if let Some((x, y)) = window.get_mouse_pos() {
            let msg = format!("click {} {}", x as usize, y as usize);
            match write.send(Message::Text(msg)).await {
                Ok(_) => {}
                Err(e) => {
                    println!("Failed to send message: {}", e);
                }
            }
        }
    }
}
