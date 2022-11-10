use async_trait::async_trait;

#[async_trait]
pub trait Command {
    fn name(&self) {}

    async fn run(&self, cx: Context, msg: Message) {}
}