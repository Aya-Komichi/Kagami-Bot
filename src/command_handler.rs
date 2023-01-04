use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::commands::core::{ping::ping, shutdown::shutdown};
use crate::commands::information::help::help;
use crate::commands::command::Command;

use {
    async_trait::async_trait,
    ruvolt::{models::{Message, User}, models::events::ReadyEvent, Context, EventHandler, error::Error},
    dotenv::dotenv,
    std::env
};

#[async_trait]
pub trait CommandHandler {
    fn prefix(&self) -> String {
        env::var("PREFIX").unwrap()
    }

    fn load_commands(&self, _cmds: Vec<&dyn Command>) {}

    async fn process(&self, ctx: Context, msg: Message) {
        let prefix = self.prefix();
        let content = msg.content.to_string();
        let some_author = ctx.cache.user(&msg.author_id).await;

        if let Some(author) = some_author {
            if content.starts_with(&prefix[..]) && !author.is_bot() {
                let msg_copy = msg.clone();
                let command = content[prefix.len()..content.len()].trim().to_lowercase();
                let cmd_str = command.as_str();
                let mut is_cmd = true;
                let result = match cmd_str {
                    "help" => help(&ctx, msg).await,
                    "ping" => ping(&ctx, msg).await,
                    "shutdown" => shutdown(&ctx, msg).await,
                    _ => {
                        is_cmd = false;
                        Ok(msg)
                    },
                };
                Self::error_handler(author, cmd_str, is_cmd, result, msg_copy).await;
            }
        }
    }

    async fn error_handler(author: User, cmd: &str, is_cmd: bool, result: Result<Message, Error>, msg: Message) {
        if result.is_err() {
            let error: Error = result.err().unwrap();
            if is_cmd {
                println!("An error ocurred when executing the `{}` command.", cmd);
                println!("Message content: {}", msg.content);
                println!("Author: {} `{}`", author.username, author.id);
                eprintln!("Error:\n{}", error);
            }
        }
    }
}

pub struct Handler<'a> {
    commands: Arc<Mutex<HashMap<&'a str, &'a dyn Command>>>
}

impl CommandHandler for Handler<'_> {
    fn load_commands(&self,_cmds:Vec<&dyn Command>) {
        
    }
}

#[async_trait]
impl EventHandler for Handler<'_> {
    async fn ready(&self, _ctx: Context, _data: ReadyEvent) {
        dotenv().ok();
        println!("I'm ready!");
    }

    async fn message(&self, ctx: Context, msg: Message) {
        self.process(ctx, msg).await;
    }
}