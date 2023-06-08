# Websocket Rust Client

A Websocket client implemented in Rust, using [dcl-rpc](https://crates.io/crates/dcl-rpc) to autogenerate the code from the Proto file located in [Protocol Repository](https://github.com/decentraland/protocol/blob/main/proto/decentraland/social/friendships/friendships.proto)

## Run clients

To avoid entering the credentials every time you run the script you can add the credentials.json file (a template is at credentials.example.json)

### Friendship Procedures Executor

This application launches two clients, each client is associated with a different user and independently performs operations specific to that user based on the specified flow. Additionally, it will print the friends and pending friendship requests of each user.

There are 4 available flows:

- `flow1`: User A sends a friendship request to B, and then A cancels it.
- `flow2`: User A sends a friendship request to B, User B accepts the friendship, and then User A deletes it.
- `flow3`: User A sends a friendship request to B, and then User B rejects it.
- `flow4`: User A sends a friendship request to B, and User B accepts the friendship. Then User B deletes it.

You can specify the flow to execute as a command-line argument. Here's how to run the client with each flow:

`cargo run --bin friendship_procedures_executor -- flow{{number}}`

If no flow is specified, the program won't perform any flow operations.

### Friendship Events Listener

This application launches two clients, each subscribing to new friendship events updates. Each client is associated with a different user and independently receives updates specific to that user.

`cargo run --bin friendship_events_listener`

This command will start the event listeners for both users. They will connect to the server, authenticate with their respective user credentials, and then continually listen for and print any incoming updates related to friendship events.

### Client A & Client B

Each application launches a client, each client is associated with a different user and independently performs operations specific to that user interacting with the other user. It's similar to the Friendship Procedures Executor, but it's not automated and it's thought to be used for manual testing against the bastion.

Note that before running the clients, you need to revise the code to check the feseability of the operations you want to perform and calculate the order and time in which they need to be executed.

To run the each program, use the following commands (in separate terminals) in that order:

`cargo run --bin client_a`

`cargo run --bin client_b`

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
