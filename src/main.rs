const URL: &str = "https://www.gesetze-im-internet.de/gii-toc.xml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get(URL).await?.text().await?;

    let res = quick_xml::de::from_str(&body);

    println!("{:#?}", res);

    Ok(())
}
