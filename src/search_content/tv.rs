

use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue, USER_AGENT, ORIGIN, REFERER},
    multipart::{Form}
};
use fake_user_agent;
use serde_json::{Value};
use urlencoding::encode;

use super::{SearchContent, SearchContentInfo};
use crate::global_types::Source;

pub async fn new(
    source: &Source,
    search: &str, 
    sort: u64,
    page: u64
) -> anyhow::Result<SearchContent, anyhow::Error> {

    let mut new_headers = HeaderMap::new();
    new_headers.insert(USER_AGENT, HeaderValue::from_str(fake_user_agent::get_chrome_rua())?);
    new_headers.insert(ORIGIN, HeaderValue::from_str("https://simkl.com")?);
    new_headers.insert(REFERER, HeaderValue::from_str("https://simkl.com/")?);



    let form_data = Form::new()
        .text("s", search.to_string())
        .text("type", source.to_string())
        .text("sort", sort.to_string())
        .text("offset", ((page-1) * 50).to_string());


    let client = Client::new();
    let res = client.post("https://simkl.com/ajax/full/search.php")
        .headers(new_headers)
        .multipart(form_data)
        .send()
        .await?;


    let data: Value = res.json().await?;

    let data_obj = data.as_object()
        .ok_or("Unable to load as object")
        .map_err(|e| anyhow::Error::msg(e))?;

    let mut new_search_content = SearchContent(Vec::new());

    for (_,  value) in data_obj {

        let raw_id = value.get("url")
            .ok_or("Unable to load id")
            .map_err(|e| anyhow::Error::msg(e))?
            .as_str()
            .ok_or("Unable to load id as string")
            .map_err(|e| anyhow::Error::msg(e))?
            .to_string()
            .replace("/tv", "");

        let id = encode(&raw_id).to_string();

        let year = value.get("year")
            .ok_or("Unable to load year")
            .map_err(|e| anyhow::Error::msg(e))?
            .as_str()
            .ok_or("Unable to load year as string")
            .map_err(|e| anyhow::Error::msg(e))?
            .to_string();

        let rank = value.get("rank")
            .ok_or("Unable to load rank")
            .map_err(|e| anyhow::Error::msg(e))?
            .as_u64();

        let raw_thumbnail_id = value.get("poster")
            .ok_or("Unable to load thumbnail")
            .map_err(|e| anyhow::Error::msg(e))?
            .as_str()
            .ok_or("Unable to load thumbnail as string")
            .map_err(|e| anyhow::Error::msg(e))?
            .to_string();

        let thumbnail_url = format!("https://wsrv.nl/?url=https://simkl.in/posters/{}_m.webp", raw_thumbnail_id);

        let title_obj = value.get("titles")
            .ok_or("Unable to load title")
            .map_err(|e| anyhow::Error::msg(e))?
            .as_object()
            .ok_or("Unable to load title as object")
            .map_err(|e| anyhow::Error::msg(e))?;

        let title = match title_obj.get("a0") {
            Some(title) => title.as_str()
                .ok_or("Unable to load title as string")
                .map_err(|e| anyhow::Error::msg(e))?
                .to_string(),
            None => title_obj.get("m")
                .ok_or("title 'm' not found")
                .map_err(|e| anyhow::Error::msg(e))?
                .as_str()
                .ok_or("Unable to load title as string")
                .map_err(|e| anyhow::Error::msg(e))?
                .to_string()
        };

        new_search_content.0.push(SearchContentInfo {id, year, rank, thumbnail_url, title});

    }

    

    return Ok(new_search_content);
}