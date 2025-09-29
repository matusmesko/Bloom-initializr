use bloom_web::{application};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    application::run().enable_swagger().await
}
