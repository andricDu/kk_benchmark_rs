use keycloak::{KeycloakAdmin, KeycloakAdminToken};
use reqwest;
use std::time::SystemTime;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("KEYCLOAK_ADDR")
        .unwrap_or_else(|_| "---".into());
    let user = std::env::var("KEYCLOAK_USER").unwrap_or_else(|_| "---".into());
    let password = std::env::var("KEYCLOAK_PASSWORD").unwrap_or_else(|_| "---".into());
    let client = reqwest::Client::new();
    let admin_token = KeycloakAdminToken::acquire(&url, &user, &password, &client).await?;

    //eprintln!("{:?}", admin_token);

    let admin = KeycloakAdmin::new(&url, admin_token, client);

    let start = SystemTime::now();
    let users = admin
        .realm_users_get(
            "pilot", None, None, None, None, None, None, None, None, None, None, None, None, None,
            None,
        )
        .await?;
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    eprintln!("Time to get all users: {:?}", duration.as_millis());

    //eprintln!("{:?}", users);

    Ok(())
}
