use crate::commands::core::{ping::ping, shutdown::shutdown};

use {
    async_trait::async_trait,
    ruvolt::{models::Message, models::events::ReadyEvent, Context, EventHandler, error::Error},
    dotenv::dotenv,
    std::env
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _cx: Context, _data: ReadyEvent) {
        dotenv().ok();
        println!("I'm ready!");
    }

    async fn message(&self, cx: Context, msg: Message) {
        let prefix = env::var("PREFIX").unwrap();
        let content = msg.content.to_string();

        if content.starts_with(&prefix[..]) {
            let command = content[prefix.len()..content.len()].trim().to_lowercase();
            let result = match command.as_str() {
                "ping" => ping(&cx, msg).await,
                "shutdown" => shutdown(&cx, msg).await,
                _ => Ok(msg),
            };
            handle_error(result, content).await;
        }
    }
}

async fn handle_error(result: Result<Message, Error>, msg_content: String) {
    if result.is_err() {
        let error: Error = result.err().unwrap();
        println!("An error ocurred when executing a command.");
        println!("Message content: {}", msg_content);
        eprintln!("{}", error);
    }
}