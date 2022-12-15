use crate::commands::command::Command;

use {
    async_trait::async_trait,
    ruvolt::{models::Message, Context, Result, error::Error}
};

pub struct Ping {}

#[async_trait]
impl Command for Ping {
    fn name(&self) ->  &str {
        return "ping";
    }

    async fn run(&self, _ctx: Context, _msg: Message) -> Result<Message, Error> {
        _msg.reply(&_ctx, "Hello! I'm here!", true).await
    }
}

pub async fn ping(cx: &Context, msg: Message) -> Result<Message, Error> {
    msg.reply(cx, "Hello! I'm here!", true).await
}