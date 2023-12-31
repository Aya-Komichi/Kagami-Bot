pub mod commands;
mod command_handler;

use {
    ruvolt::{Client, Result},
    dotenv::dotenv,
    std::env
};

#[tokio::main]
async fn main() -> Result {
    dotenv().ok();
    let token = env::var("TOKEN").unwrap();
    let mut client = Client::new(command_handler::Handler, token).await?;

    client.listen().await
}
