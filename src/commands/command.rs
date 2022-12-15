use {
    async_trait::async_trait,
    ruvolt::{models::Message, Context, Result, error::Error}
};

#[async_trait]
pub trait Command {
    fn name(&self) -> &str {
        return "";
    }

    async fn run(&self, _ctx: Context, _msg: Message) -> Result<Message, Error> {
        return Err(Error::Unknown("".to_string()));
    }
}