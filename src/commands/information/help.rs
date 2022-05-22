use ruvolt::{
    models::Message,
    builders::{CreateMessage, CreateEmbed},
    Context,
    Result,
    error::Error,
};
use {
    dotenv::dotenv,
    std::env
};

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