# frame_sockets

Small prototype of a simple remote graphics rendering server using [minifb](https://crates.io/crates/minifb) and websockets/[tokio-tungstenite](https://crates.io/crates/tokio-tungstenite).

## Server:
Generates frames and sends them to the client via websockets. Also receives mouse click events from the client and draws them on the frame.

## Client:
Receives frames from the server and displays them. Also sends mouse click events to the server.


## Feature Checklist:
- [x] Server generates frames and sends them to the client
- [x] Client receives frames and displays them in a window
- [x] Client sends mouse movement events to the server
- [x] Server receives mouse movementos events from the client and draws them on the frame
- [x] Client actions happens in the right order (latency and stability could be increased by using a thread-safe window library or decoupling the mouse events from the window display state)

## Future Goals:
- [ ] stream an arbitrary application (rusty-roguelike?)
- [ ] encode stream using a video codec (AV1?)
- [ ] use more robust stream type (WebRTC/VNC?)
- [ ] run client in browser w/ WASM (and ffmpeg?)
- [ ] explore feasibility of running multiple threads each serving up a framebuffer stream to a different client
- [ ] encrypt the framebuffers and mouse events indepedently of the enclosing stream

## To Run
Ensure there's a working Rust toolchain installed. Then run `cargo run` in each of the `server` and `client` directories (server first). Or build it if you like. Wiggle your mouse in the window that pops up and be amazed by the latency and jitter. See minifb docs for info on running in an environment other than MacOS.
