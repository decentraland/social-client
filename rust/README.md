# Websocket Rust Client 
A Websocket client implemented in Rust, using [dcl-rpc](https://crates.io/crates/dcl-rpc) to autogenerate the code from the Proto file located in [Protocol Repository](https://github.com/decentraland/protocol/blob/main/proto/decentraland/social/friendships/friendships.proto)

## Run clients

### Client A: Executor
This is the executor of the actions, gets all friends and requests and then performs actions based on the specified flow. 

There are two available flows:

- `flow1`: User A sends a friendship request to B, and then A cancels it.
- `flow2`: User A sends a friendship request to B, User B accepts the friendship, and then User A deletes it.
- `flow3`: User A sends a friendship request to B, and then User B rejects it.
- `flow4`: User A sends a friendship request to B, and User B accepts the friendship. Then User B deletes it.

You can specify the flow to execute as a command-line argument. Here's how to run the client with each flow:

For flow1: `cargo run --bin a-client-ws -- flow1`
For flow2: `cargo run --bin a-client-ws -- flow2`
For flow3: `cargo run --bin a-client-ws -- flow3`

If no flow is specified, the program will panic with a message indicating that a flow must be provided.

### Client B: Listener
This client only subscribes to new friendship event updates and receives the updates from Client A.
`cargo run --bin b-client-ws`
