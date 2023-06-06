pub mod benchmark_args;
pub mod benchmark_client;
pub mod credentials;
pub mod friendship_procedures;
pub mod benchmark_simulation;

include!(concat!(
    env!("OUT_DIR"),
    "/decentraland.social.friendships.rs"
));

pub struct MyExampleContext {
    pub hardcoded_database: Vec<User>,
}
