use mini_redis::{client, Result};

#[tokio::main]
async fn main()->Result<()> {

    let mut client=client::connect("127.0.0.1:6379").await?;
    client.set("hello", "world2".into()).await?;
    println!("set hello world success!");
    let result= client.get("hello").await.unwrap();
    if let Some(b)=result{
        let str= String::from_utf8(b.to_vec()).unwrap();
        println!("{str}");
    }
    Ok(())
}