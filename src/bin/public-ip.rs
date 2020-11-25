extern crate public_ip;

use anyhow::Result;

#[async_std::main]
async fn main() -> Result<()> {
    println!("IP v4: {}", public_ip::v4().await?);
    println!("IP v6: {}", public_ip::v6().await?);
    Ok(())
}
