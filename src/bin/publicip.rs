extern crate publicip;

use anyhow::Result;

#[async_std::main]
async fn main() -> Result<()> {
    println!("IP v4: {}", publicip::v4().await?);
    println!("IP v6: {}", publicip::v6().await?);
    Ok(())
}
