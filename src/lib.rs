use anyhow::Result;

pub async fn v4() -> Result<String> {
    // let mut res = surf::get("https://api.ipify.org/").await?;
    // Ok(res.body_string().await?)
    Ok(String::from(""))
}

pub async fn v6() -> Result<String> {
    // let mut res = surf::get("https://api64.ipify.org/").await?;
    // Ok(res.body_string().await?)
    Ok(String::from(""))
}
