pub mod credentials;
pub mod friendship_procedures;

include!(concat!(
    env!("OUT_DIR"),
    "/decentraland.social.friendships.rs"
));

pub struct MyExampleContext {
    pub hardcoded_database: Vec<User>,
}
