use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {    
    let client_builder = reqwest::ClientBuilder::new();
    let client = client_builder.build()?;

    let mut url = reqwest::Url::parse("https://api.openalex.org/authors/https://orcid.org/0000-0002-9451-1957")?;
    url.set_query(Some("api_key=odrzvqHhJTTgzgrPRLrnZj"));

    let resp = client.get(url).send().await?;


    println!("Resp: {resp:#?}");
    let res = resp.text().await?;
    println!("{res:#?}");
    Ok(())
}