use mini_redis::{client, Result};


#[tokio::main]
async fn main() -> Result<()>{
    println!("Hello, world!");
    let host = "127.0.0.1:6379";
    let host = "127.0.0.1:5000";
    let mut client = client::connect(host).await?;

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
