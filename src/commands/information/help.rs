use crate::commands::command::Command;

use ruvolt::{
    models::Message,
    builders::{CreateMessage, CreateEmbed},
    Context,
    Result,
    error::Error,
};
use {
    async_trait::async_trait,
    dotenv::dotenv,
    std::env
};

pub struct Help {}

#[async_trait]
impl Command for Help {
    fn name(&self) -> &str {
        return "help"
    }

    async fn run(&self, _ctx: Context, _msg: Message) -> Result<Message, Error> {
        dotenv().ok();
        let prefix = env::var("PREFIX").unwrap();
        let message = CreateMessage::new(" ").embed(|embed: CreateEmbed| -> CreateEmbed {
            embed.title("Command's List")
                .color("#d02bff")
                .description(format!("Don't forget my prefix is `{}`.\n\n`help`, `ping`, `shutdown`", prefix))
        });
        _msg.reply(&_ctx, message, true).await
    }
}

pub async fn help(cx: &Context, msg: Message) -> Result<Message, Error> {
    dotenv().ok();
    let prefix = env::var("PREFIX").unwrap();
    let message = CreateMessage::new(" ").embed(|embed: CreateEmbed| -> CreateEmbed {
        embed.title("Command's List")
            .color("#d02bff")
            .description(format!("Don't forget my prefix is `{}`.\n\n`help`, `ping`, `shutdown`", prefix))
    });
    msg.reply(cx, message, true).await
}