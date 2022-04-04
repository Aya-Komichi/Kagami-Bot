mod command_handler;

use {
    ruvolt::{Client, Result},
    std::env
};

#[tokio::main]
async fn main() -> Result {
    let token = env::var("TOKEN").unwrap();
    let mut client = Client::new(command_handler::Handler, token).await?;

    client.listen().await
}
