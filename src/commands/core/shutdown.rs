use crate::commands::command::Command;

use {
    async_trait::async_trait,
    ruvolt::{models::Message, Context, Result, error::Error}
};

pub struct Shutdown {}

#[async_trait]
impl Command for Shutdown {
    fn name(&self) -> &str {
        return "shutdown"
    }

    async fn run(&self, _ctx: Context, _msg: Message) -> Result<Message, Error> {
        _msg.reply(&_ctx, "Shutting down. See you later!", true).await.ok();
        std::process::exit(0)
    }
}

pub async fn shutdown(cx: &Context, msg: Message) -> Result<Message, Error> {
    msg.reply(cx, "Shutting down. See you later!", true).await.ok();
    std::process::exit(0)
}