use byteorder::{ByteOrder, LittleEndian};
use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};
use std::println;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use tungstenite::connect;
use tungstenite::protocol::Message;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    // create a thread to listen for incoming connections and write them to a channel
    // the while window open loop will read from the channel and update the framebuffer

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

    let socket = {
        match connect("ws://localhost:9001") {
            Ok((socket, _)) => socket,
            Err(e) => panic!("Failed to connect: {}", e),
        }
    };
    let socket = Arc::new(Mutex::new(socket));
    let socket_clone = Arc::clone(&socket);

    // // create an mpsc channel to send messages from the websocket thread to the window thread
    // let (tx, rx) = mpsc::channel::<Vec<u32>>();
    // // create a thread to listen for incoming messages and write them to the channel
    // let _ = std::thread::spawn(move || {
    //     let mut buffer = vec![0; WIDTH * HEIGHT];
    //     loop {
    //         let msg = socket.lock().unwrap().read_message().unwrap();
    //         match msg {
    //             Message::Binary(bytes) => {
    //                 println!("binary");
    //                 LittleEndian::read_u32_into(&bytes, &mut buffer);
    //             }
    //             Message::Text(text) => {
    //                 // handle text message
    //                 println!("{}", text);
    //                 continue;
    //             }
    //             // handle other types of messages as necessary
    //             _ => {
    //                 println!("other");
    //                 continue;
    //             }
    //         }
    //         tx.send(buffer.clone()).unwrap();
    //     }
    // });
    // //TODO: while window is open, for every message on the channel update the framebuffer
    // while window.is_open() && !window.is_key_down(Key::Escape) {
    //     if rx.try_recv().is_ok() {
    //         for buffer in rx.iter() {
    //             match window.update_with_buffer(&buffer, WIDTH, HEIGHT) {
    //                 Ok(_) => {}
    //                 Err(e) => {
    //                     println!("Failed to update window: {}", e);
    //                     return;
    //                 }
    //             }
    //         }
    //     }

    //     // handle mouse click
    //     if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
    //         println!("mouse pos {} {}", x, y);
    //         if window.get_mouse_down(MouseButton::Left) {
    //             let msg = format!("click {} {}", x as usize, y as usize);
    //             let mut socket = socket_clone.lock().unwrap();
    //             let _ = socket.write_message(Message::Text(msg));
    //             println!("click");
    //         }
    //     }
    // }

    let (mut width, mut height) = (WIDTH, HEIGHT);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        {
            let (new_width, new_height) = window.get_size();
            if new_width != width || new_height != height {
                // Div by / 2 here as we use 2x scaling for the buffer
                // copy valid bits of old buffer to new buffer
                let mut new_buffer = vec![0; (new_width / 2) * (new_height / 2)];
                for y in 0..(height / 2).min(new_height / 2) {
                    for x in 0..(width / 2).min(new_width / 2) {
                        new_buffer[y * (new_width / 2) + x] = buffer[y * (width / 2) + x];
                    }
                }
                buffer = new_buffer;
                width = new_width;
                height = new_height;
            }
        }

        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            let screen_pos = ((y as usize) * (width / 2)) + x as usize;

            if window.get_mouse_down(MouseButton::Left) {
                //tick
                {
                    let mut s = socket_clone.lock().unwrap();
                    let _ = s.write_message(Message::Text("client tick".to_string()));
                    println!("click");
                }
                buffer[screen_pos] = 0x00ffffff;
            }
        }

        if let Some(scroll) = window.get_scroll_wheel() {
            println!("Scrolling {} - {}", scroll.0, scroll.1);
        }

        // We unwrap here as we want this code to exit if it fails
        window
            .update_with_buffer(&buffer, width / 2, height / 2)
            .unwrap();
    }
}
