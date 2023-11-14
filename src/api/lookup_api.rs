use reqwest::Error;

/* #[get("/search/<upc>")]
pub async fn lookup(upc: String) -> Result<String, Error> {
    let url = format!("https://api.upcitemdb.com/prod/trial/lookup?upc={}", upc);
    let response = reqwest::get(&url).await?;

    let text = response.text().await?;

    Ok(text)
} */
