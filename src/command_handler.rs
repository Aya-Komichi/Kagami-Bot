use {
    async_trait::async_trait,
    ruvolt::{models::Message, Context, EventHandler},
    std::env
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, cx: Context, msg: Message) {
        let prefix = env::var("PREFIX").unwrap();
        let content = msg.content.to_string();

        if content.starts_with(&prefix) {
            let command = content[prefix.len()..content.len() - 1].trim();
            //msg.reply(&cx, "Pong!", true).await.ok();
        }
    }
}