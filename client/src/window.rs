use byteorder::{ByteOrder, LittleEndian};
use minifb::{Key, MouseMode, Scale, Window, WindowOptions};
use tokio_tungstenite::tungstenite::Message;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

pub struct DrawingWindow {
    buffer: Vec<u32>,
    window: Window,
}

impl DrawingWindow {
    pub fn new() -> Result<Self, minifb::Error> {
        let buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
        let mut window = Window::new(
            "Wiggle Mouse to Draw - Press ESC to exit",
            WIDTH,
            HEIGHT,
            WindowOptions {
                scale: Scale::X2,
                ..WindowOptions::default()
            },
        )?;

        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
        Ok(DrawingWindow { buffer, window })
    }
    pub fn is_exited(&self) -> bool {
        !self.window.is_open() && self.window.is_key_down(Key::Escape)
    }

    pub fn get_mouse_pos(&self) -> Option<(f32, f32)> {
        self.window.get_mouse_pos(MouseMode::Pass)
    }
    fn update_with_buffer(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
    pub fn process_message(&mut self, msg: Message) {
        match msg {
            Message::Binary(bytes) => {
                if bytes.len() == self.buffer.len() * 4 {
                    LittleEndian::read_u32_into(&bytes, &mut self.buffer);
                } else {
                    println!("Unexpected byte size for the framebuffer");
                }
                self.update_with_buffer();
            }
            _ => {
                println!("other");
            }
        }
    }
}
