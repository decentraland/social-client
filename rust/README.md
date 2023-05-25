# Websocket Rust Client

A Websocket client implemented in Rust, using [dcl-rpc](https://crates.io/crates/dcl-rpc) to autogenerate the code from the Proto file located in [Protocol Repository](https://github.com/decentraland/protocol/blob/main/proto/decentraland/social/friendships/friendships.proto)

## Run clients

To avoid entering the credentials every time you run the script you can add the credentials.json file (a template is at credentials.example.json)

### Client A: Executor

This is the executor of the actions, gets all friends and requests and then performs actions based on the specified flow.

There are 4 available flows:

- `flow1`: User A sends a friendship request to B, and then A cancels it.
- `flow2`: User A sends a friendship request to B, User B accepts the friendship, and then User A deletes it.
- `flow3`: User A sends a friendship request to B, and then User B rejects it.
- `flow4`: User A sends a friendship request to B, and User B accepts the friendship. Then User B deletes it.

You can specify the flow to execute as a command-line argument. Here's how to run the client with each flow:

`cargo run --bin a-client-ws -- flow{{number}}`

If no flow is specified, the program will panic with a message indicating that a flow must be provided.

### Client B: Listener

This client only subscribes to new friendship event updates and receives the updates from Client A.
`cargo run --bin b-client-ws`

---

## Accessing Social-Service Instances:

How to access different instances of the social-service by creating tunnels through the bastion.

### Step 1: Obtain Instance IPs

1. Connect to the bastions.

2. Once connected, run the following `dig` command to get the IP addresses of the social-service instances:

`dig +noall +answer social-service.internal-ns`

### Step 2: Create Tunnels to the Instances

Now, you're ready to create tunnels from your local machine to the service instances:

1. Disconnect from the bastion host (if you're still connected).

2. Run the following command to create the tunnels. Replace {ip} with the IPs you obtained from the previous step.

`ssh ubuntu@b.decentraland.io -L 5000:{ip}:8085 -L 5001:{ip}:8085`

(The format is `ssh ubuntu@{bastion_host} -L {localport}:{ip}:{serviceport}`)

### Step 3: Access the Services

With the tunnels established, you can access the social-service instances via localhost:5000 and localhost:5001 while the SSH connection is active.

Note that if you terminate the SSH connection or if it's lost, the tunnels will close and you will need to repeat the steps to re-establish them.
