use ruvolt::{models::Message, Context, Result, error::Error};

pub async fn ping(cx: &Context, msg: Message) -> Result<Message, Error> {
    msg.reply(cx, "Hello! I'm here!", true).await
}