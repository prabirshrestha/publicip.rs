extern crate publicip;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("IP v4: {}", publicip::v4().await?);
    println!("IP v6: {}", publicip::v6().await?);
    Ok(())
}
