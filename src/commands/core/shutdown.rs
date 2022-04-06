use ruvolt::{models::Message, Context, Result, error::Error};

pub async fn shutdown(cx: &Context, msg: Message) -> Result<Message, Error> {
    msg.reply(cx, "Shutting down. See you later!", true).await.ok();
    std::process::exit(0)
}