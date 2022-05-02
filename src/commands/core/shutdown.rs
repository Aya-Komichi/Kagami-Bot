use ruvolt::{models::Message, Context, Result, error::Error};

pub async fn shutdown(cx: &Context, msg: Message) -> Result<Message, Error> {
    msg.reply(cx, "This command is still unfinished.", true).await
    //std::process::exit(0)
}