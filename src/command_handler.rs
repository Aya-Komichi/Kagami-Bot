use {
    async_trait::async_trait,
    ruvolt::{models::Message, models::events::ReadyEvent, Context, EventHandler},
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
            let command = content[prefix.len()..content.len()].trim().to_lowercase();
            match command.as_str() {
                "ping" => msg.reply(&cx, "Hello! I'm here!", true).await.ok(),
                "shutdown" => {
                    msg.reply(&cx, "Shutting down. See you later!", true).await.ok();
                    std::process::exit(0)
                },
                _ => None,
            };
        }
    }
}