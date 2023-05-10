include!(concat!(
    env!("OUT_DIR"),
    "/decentraland.social.friendships.rs"
));

pub struct MyExampleContext {
    pub hardcoded_database: Vec<User>,
}
