use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue, USER_AGENT},
};
use visdom::Vis;
use regex::Regex;
use urlencoding::encode;

use super::{FeaturedContent, FeaturedContentInfo};

pub async fn new() -> anyhow::Result<FeaturedContent, anyhow::Error> {

    let mut new_headers = HeaderMap::new();
    new_headers.insert(USER_AGENT, HeaderValue::from_str("PostmanRuntime/7.53.0")?);

    let client = Client::new();
    let res = client.get("https://simkl.com/movies/")
        .headers(new_headers)
        .send()
        .await?;
    
    let html = res.text().await?;

    let mut new_featured_content = FeaturedContent(vec![]);


    let vis = Vis::load(html)
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;

    for script in vis.find("script") {
        let text: String = script.html();

        if text.contains("var artData =") {
            let re = Regex::new(r"(?s)var\s+artData\s*=\s*(\[.*?\]);")?;
            if let Some(cap) = re.captures(&text) {
                let array_str = cap.get(1)
                    .ok_or("array_str not found")
                    .map_err(|e| anyhow::Error::msg(e.to_string()))?
                    .as_str();

                let items: Vec<Vec<String>> = json5::from_str(&array_str)?;

                for item in items {
                    let id = encode(&item[8]).to_string();

                    let mut new_contextual: Vec<String> = vec![
                        String::from("Movies"),
                        format!("Rating: {}", item[7]),
                    ];
                    new_contextual.retain(|i| !i.is_empty());

                    new_featured_content.0.push(FeaturedContentInfo {
                        id: id.to_string(),
                        title: item[1].clone(),
                        contextual: new_contextual,
                        short_description: item[3].clone(),
                        banner_url: format!("https://wsrv.nl/?url=https://simkl.in/fanart/{}_medium.webp", item[9]),
                    });
                }
            }
        }
    }


    return Ok(new_featured_content);
}