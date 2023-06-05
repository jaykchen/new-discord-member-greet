use discord_flows::{
    get_client, listen_to_event,
    model::{Message, MessageActivityKind},
    Bot::Default,
};
use dotenv::dotenv;
use std::env;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv().ok();

    listen_to_event(Default, |msg| async  {
        handle(msg).await;
    })
    .await;
}

async fn handle(msg: Message) {
    let client = get_client(Default);
    // let channel_id = msg.channel_id;
    // let content = msg.content;
    // let discord_server = env::var("discord_server").unwrap_or("Vivian Hu's server".to_string());
    // let discord_channel = env::var("discord_channel").unwrap_or("general".to_string());

    if msg.author.bot {
        return;
    }

    if msg.activity.unwrap().kind == MessageActivityKind::JOIN {
        let body = format!("Welcome {} to {}!", msg.author.to_string(), discord_channel);

        _ = client
            .send_message(
                1112553551789572167,
                &serde_json::json!({
                    "content": body,
                }),
            )
            .await;
    }
}
