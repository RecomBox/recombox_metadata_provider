
use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue, USER_AGENT, ORIGIN, REFERER},
    multipart::{Form}
};
use visdom::Vis;
use fake_user_agent;
use html_escape::decode_html_entities;
use urlencoding::encode;

use super::{TrendingContent, TrendingContentInfo};

pub async fn new() -> anyhow::Result<TrendingContent, anyhow::Error> {

    let mut new_headers = HeaderMap::new();
    new_headers.insert(USER_AGENT, HeaderValue::from_str(fake_user_agent::get_chrome_rua())?);
    new_headers.insert(ORIGIN, HeaderValue::from_str("https://simkl.com")?);
    new_headers.insert(REFERER, HeaderValue::from_str("https://simkl.com/")?);

    let form_data = Form::new()
        .text("action", "best")
        .text("cat", "month")
        .text("filt_tv", "0")
        .text("offset", "0")
        .text("async", "true")
        .text("afilt_tv", "0")
        .text("double", "0");

    let client = Client::new();
    let res = client.post("https://simkl.com/ajax/full/anime.php")
        .headers(new_headers)
        .multipart(form_data)
        .send()
        .await?;
    
    let html = res.text().await?;

    let vis = Vis::load(&html)
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;

    let item_ele_list = vis.find(".SimklTVBestItems");

    let mut new_trending_content = TrendingContent(vec![]);

    for item_ele in item_ele_list {
        let item_vis = Vis::load(item_ele.html())
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let raw_title = item_vis.find(".SimklTVBestItemTitle").text();
        let title = decode_html_entities(raw_title.trim()).to_string();


        let raw_year = item_vis.find(".SimklTVAboutYearCountry")
            .find(".detailYearInfo").text();

        let year = decode_html_entities(raw_year.trim()).to_string();


        let raw_rating = item_vis.find(".SimklTVBestIcoScore").get(0)
            .ok_or("Can't find rating.")
            .map_err(|e| anyhow::Error::msg(e.to_string()))?
            .text();

        let splited_rating = raw_rating.split("/").nth(0)
            .ok_or("Can't find rating.")
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let rating: f32 = decode_html_entities(splited_rating.trim()).to_string().parse()?;

        let raw_id = item_vis.find(".SimklTVBestItemWraper")
            .find("a")
            .get(0)
            .ok_or("Can't find id.")
            .map_err(|e| anyhow::Error::msg(e.to_string()))?
            .get_attribute("href")
            .ok_or("Can't find id.")
            .map_err(|e| anyhow::Error::msg(e.to_string()))?
            .to_string()
            .replace("/anime", "");

        let id = encode(&raw_id).to_string();


        let thumbnail_url = item_vis.find(".SimklTVBestItemWraper")
            .find("a")
            .find("img")
            .get(0)
            .ok_or("Can't find thumbnail.")
            .map_err(|e| anyhow::Error::msg(e.to_string()))?
            .get_attribute("src")
            .ok_or("Can't find thumbnail.")
            .map_err(|e| anyhow::Error::msg(e.to_string()))?
            .to_string()
            .replace("//", "https://");

            
        new_trending_content.0.push(TrendingContentInfo { title, year, rating, id, thumbnail_url });
        
    }

    return Ok(new_trending_content);
}