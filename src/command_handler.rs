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
            let msg_copy = msg.clone();
            let command = content[prefix.len()..content.len()].trim().to_lowercase();
            let cmd_str = command.as_str();
            let mut is_cmd = true;
            let result = match cmd_str {
                "ping" => ping(&cx, msg).await,
                "shutdown" => shutdown(&cx, msg).await,
                _ => {
                    is_cmd = false;
                    Ok(msg)
                },
            };
            error_handler(cx, cmd_str, is_cmd, result, msg_copy).await;
        }
    }
}

async fn error_handler(context: Context, cmd_str: &str, is_cmd: bool, result: Result<Message, Error>, msg: Message) {
    if result.is_err() {
        let error: Error = result.err().unwrap();
        if is_cmd {
            let author = context.cache.user(&msg.author_id).await.unwrap();
            println!("An error ocurred when executing the `{}` command.", cmd_str);
            println!("Message content: {}", msg.content);
            println!("Author: {} `{}`", author.username, author.id);
            eprintln!("```{}```", error);
        }
    }
}