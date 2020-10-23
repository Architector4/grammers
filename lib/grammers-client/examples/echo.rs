//! Example to echo user text messages.
//!
//! The `TG_ID` and `TG_HASH` environment variables must be set (learn how to do it for
//! [Windows](https://ss64.com/nt/set.html) or [Linux](https://ss64.com/bash/export.html))
//! to Telegram's API ID and API hash respectively.
//!
//! Then, run it as:
//!
//! ```sh
//! cargo run --example echo -- BOT_TOKEN
//! ```

use grammers_client::ext::{MessageExt, UpdateExt};
use grammers_client::{AuthorizationError, Client, Config};
use grammers_session::Session;
use log;
use simple_logger;
use std::env;
use tokio::runtime;

async fn async_main() -> Result<(), AuthorizationError> {
    simple_logger::init_with_level(log::Level::Debug).expect("failed to setup logging");

    let api_id = env!("TG_ID").parse().expect("TG_ID invalid");
    let api_hash = env!("TG_HASH").to_string();
    let token = env::args().skip(1).next().expect("token missing");

    println!("Connecting to Telegram...");
    let mut client = Client::connect(Config {
        session: Session::load_or_create("echo.session")?,
        api_id,
        api_hash: api_hash.clone(),
        params: Default::default(),
    })
    .await?;
    println!("Connected!");

    if !client.is_authorized().await? {
        println!("Signing in...");
        client.bot_sign_in(&token, api_id, &api_hash).await?;
        println!("Signed in!");
    }

    println!("Waiting for messages...");
    while let Some((updates, entity_set)) = client.next_updates().await {
        for update in updates {
            if let Some(message) = update.message() {
                let peer = entity_set
                    .get(&message.chat())
                    .expect("failed to find entity");

                println!("Responding to {}", peer.name());
                client
                    .send_message(peer.to_input_peer(), message.message.as_str().into())
                    .await?;
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), AuthorizationError> {
    runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}
