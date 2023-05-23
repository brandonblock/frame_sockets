use byteorder::{ByteOrder, LittleEndian};
use futures::{SinkExt, StreamExt};
use minifb::{Key, MouseMode, Scale, Window, WindowOptions};
use std::time::Duration;
use tokio::time::sleep;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

#[tokio::main]
async fn main() {
    // arrange window
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

    // create socket connection
    let url = url::Url::parse("ws://localhost:9001").unwrap();
    let (socket, response) = connect_async(url).await.unwrap();
    let (mut write, mut read) = socket.split();

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    // handle mouse and render events in the main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        //total forced-sync hack to get this stood up using a read timeout on the async socket
        sleep(std::time::Duration::from_nanos(1)).await;

        //read framebuffers from the socket for a very small amount of time
        if let Ok(Some(Ok(msg))) = timeout(Duration::from_nanos(1), read.next()).await {
            match msg {
                Message::Binary(bytes) => {
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
                _ => {
                    println!("other");
                }
            }
        }
        // send our mouse position to the server
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Pass) {
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
