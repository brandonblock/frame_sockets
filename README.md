# frame_sockets

Small prototype of a remote graphical environment server.

## Server:
Generates frames and sends them to the client via websockets. Also receives mouse click events from the client and draws them on the frame.

## Client:
Receives frames from the server and displays them. Also sends mouse click events to the server.


## Feature Checklist:
- [x] Server generates frames and sends them to the client
- [x] Client receives frames and displays them in a window
- [x] Client sends mouse click events to the server
- [x] Server receives mouse click events from the client and draws them on the frame
- [ ] Client actions happens in the right order (appropriate threads, locks, and async fns)

## Future Goals:
- [ ] stream an arbitrary application (rusty-roguelike?)
- [ ] encode stream using a video codec (AV1?)
- [ ] use more robust stream type (WebRTC?)
- [ ] run client in browser w/ WASM (and ffmpeg?)

## To Run
`cargo run` in each of the `server` and `client` directories (server first). Wiggle your mouse in the window that pops up and be amazed by the latency.
