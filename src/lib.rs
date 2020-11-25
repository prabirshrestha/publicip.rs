pub async fn v4() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut res = surf::get("https://api.ipify.org/").await?;
    Ok(res.body_string().await?)
}

pub async fn v6() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut res = surf::get("https://api64.ipify.org/").await?;
    Ok(res.body_string().await?)
}
