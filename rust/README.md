# Websocket Rust Client 
A Websocket client implemented in Rust, using [dcl-rpc](https://crates.io/crates/dcl-rpc) to autogenerate the code from the Proto file located in [Protocol Repository](https://github.com/decentraland/protocol/blob/main/proto/decentraland/social/friendships/friendships.proto)

### Run clients

#### Client A: Executor
This is the executor of the actions, gets all friends and requests and then sends a friendship request and a cancel.
`cargo run --bin a-client-ws`

### Client B: Listener
This client only subscribes to new friendship event updates and receives the request and cancel from Client A.
`cargo run --bin b-client-ws`
