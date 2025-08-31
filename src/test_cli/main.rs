use anyhow::anyhow;
use std::env::var;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let user = var("USERNAME");
    let password = var("PASSWORD");
    let ref_token = var("TOKEN");

    let mut client = tankille::client::Client::new()?;

    if let Ok(token) = ref_token {
        client.set_refresh_token(&token);
    } else if let (Ok(user), Ok(pass)) = (user, password) {
        let login = tankille::client::LoginOptions::new(
            &user,
            &pass,
        );

	client.login(login).await?;
    } else {
	return Err(anyhow!("Supply either a refresh token as $TOKEN, or a $USERNAME and $PASSWORD"));
    }

    println!("{client:#?}");

    client.refresh_token().await?;
    println!("{client:#?}");

    Ok(())
}
