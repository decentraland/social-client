use std::io::{self, Write};

pub async fn get_input(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    io::stdout().flush()?; // Ensure the prompt is displayed before read_line

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim_end().to_owned())
}

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub address: String,
    pub token: String,
}

pub async fn load_users() -> [AuthUser; 3] {
    // Read token from file
    match std::fs::read_to_string("credentials.zone.json") {
        Ok(it) => {
            let users = serde_json::from_str::<serde_json::Value>(&it).unwrap();
            let users = &users["users"];

            let user_a = extract_user(&users[0], "A").await;
            let user_b = extract_user(&users[1], "B").await;
            let user_c = extract_user(&users[2], "C").await;

            [user_a, user_b, user_c]
        }
        Err(_) => {
            // If missing read from stdin
            let token_user_a = get_input("Enter Token for User A: ").await.unwrap();
            let token_user_b = get_input("Enter Token for User B: ").await.unwrap();
            let token_user_c = get_input("Enter Token for User C: ").await.unwrap();
            let user_a_address = get_input("Enter Address for User A: ").await.unwrap();
            let user_b_address = get_input("Enter Address for User B: ").await.unwrap();
            let user_c_address = get_input("Enter Address for User C: ").await.unwrap();

            [
                AuthUser {
                    address: user_a_address,
                    token: token_user_a,
                },
                AuthUser {
                    address: user_b_address,
                    token: token_user_b,
                },
                AuthUser {
                    address: user_c_address,
                    token: token_user_c,
                },
            ]
        }
    }
}

pub async fn extract_user(user: &serde_json::Value, user_id: &str) -> AuthUser {
    let address = match user["social_user_id"].as_str() {
        Some(address) => address.to_string(),
        None => {
            let message = format!("Enter address for User {user_id}");
            get_input(&message).await.unwrap()
        }
    };
    let token = match user["access_token"].as_str() {
        Some(token) => token.to_string(),
        None => {
            let message = format!("Enter token for User {user_id}");
            get_input(&message).await.unwrap()
        }
    };
    AuthUser { address, token }
}
