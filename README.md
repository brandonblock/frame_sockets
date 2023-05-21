# frame_sockets

Small prototype of a remote graphical environment server.

## Server:
Generates frames and sends them to the client via websockets. Also receives mouse click events from the client and draws them on the frame.

## Client:
Receives frames from the server and displays them. Also sends mouse click events to the server.


## Feature Checklist:
- [x] Server generates frames and sends them to the client
- [ ] Client receives frames and displays them in a window
- [ ] Client sends mouse click events to the server
- [ ] Server receives mouse click events from the client and draws them on the frame

## Future Goals:
- [ ] stream an arbitrary application
- [ ] encode stream using AV1
- [ ] use [webrtc](https://crates.io/crates/webrtc) instead of [tungstenite](https://crates.io/crates/tungstenite)
- [ ] run client in browser w/ WASM and ffmpeg
