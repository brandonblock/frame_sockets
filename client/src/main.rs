use byteorder::{ByteOrder, LittleEndian};
use futures::{SinkExt, StreamExt};
use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};
use std::time::Duration;
use tokio::time::sleep;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

#[tokio::main]
async fn main() {
    // create a window
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = match Window::new(
        "Mouse Draw - Press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    ) {
        Ok(window) => window,
        Err(e) => {
            println!("Failed to create a window: {}", e);
            return;
        }
    };
    match window.update_with_buffer(&buffer, WIDTH, HEIGHT) {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to create initial window: {}", e);
            return;
        }
    }

    let url = url::Url::parse("ws://localhost:9001").unwrap();

    let (socket, response) = connect_async(url).await.unwrap();

    let (mut write, mut read) = socket.split();

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    // TODO: spawn thread (don;t forget 'move' to read and send mouse movements, check for rx.next in main while loop for framebuffers

    while window.is_open() && !window.is_key_down(Key::Escape) {
        match write.send(Message::Text("client tick".to_string())).await {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to send client tick: {}", e);
            }
        }

        sleep(std::time::Duration::from_millis(2)).await;

        // // assign bytes in this scope to drop the framebuffer lock as soon as possible
        // assign bytes in this scope to drop the framebuffer lock as soon as possible
        if let Ok(Some(Ok(msg))) = timeout(Duration::from_nanos(1), read.next()).await {
            match msg {
                Message::Binary(bytes) => {
                    println!("binary");
                    // Assuming buffer is Vec<u32>
                    if bytes.len() == buffer.len() * 4 {
                        LittleEndian::read_u32_into(&bytes, &mut buffer);
                    } else {
                        println!("Unexpected byte size for the framebuffer");
                    }
                    match window.update_with_buffer(&buffer, WIDTH, HEIGHT) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Failed to update window: {}", e);
                        }
                    }
                }
                Message::Text(text) => {
                    // handle text message
                    println!("{}", text);
                }
                // handle other types of messages as necessary
                _ => {
                    println!("other");
                }
            }
        }
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            println!("got mouse position");
            // if window.get_mouse_down(MouseButton::Left) {
            println!("clicked");
            // convert the coordinates to the format you want, then send it
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
