use discord_flows::{
    get_client, listen_to_event,
    model::{Message, MessageActivityKind},
};
use dotenv::dotenv;
use std::env;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv().ok();
    let discord_token = env::var("discord_token").expect("Expected a bot token.");

    listen_to_event(discord_token.clone(), move |msg| handle(msg, discord_token)).await;
}

async fn handle(msg: Message, token: String) {
    let client = get_client(token);
    // let channel_id = msg.channel_id;
    // let content = msg.content;
    // let discord_server = env::var("discord_server").unwrap_or("Vivian Hu's server".to_string());
    let discord_channel = env::var("discord_channel").unwrap_or("general".to_string());

    if msg.author.bot {
        return;
    }

    if msg.activity.unwrap().kind == MessageActivityKind::JOIN {
        let body = format!("Welcome {} to {}!", msg.author.to_string(), discord_channel);

        _ = client
            .send_message(
                msg.channel_id.into(),
                &serde_json::json!({
                    "content": body,
                }),
            )
            .await;
    }
}
