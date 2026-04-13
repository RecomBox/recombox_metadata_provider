
use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue, USER_AGENT, ORIGIN, REFERER}
};
use visdom::Vis;
use html_escape::decode_html_entities;
use urlencoding::decode;
use regex::Regex;


use super::{ViewContentInfo, EpisodeInfo};

pub async fn new(id: &str) -> anyhow::Result<ViewContentInfo, anyhow::Error> {

    let mut new_headers = HeaderMap::new();
    new_headers.insert(USER_AGENT, HeaderValue::from_str("PostmanRuntime/7.53.0")?);
    new_headers.insert(ORIGIN, HeaderValue::from_str("https://simkl.com")?);
    new_headers.insert(REFERER, HeaderValue::from_str("https://simkl.com/")?);


    let client = Client::new();

    let res = client.get(format!("https://simkl.com/movies{}/", decode(id)?))
        .headers(new_headers)
        .send()
        .await?;
    
    let html = res.text().await?;


    let vis = Vis::load(&html)
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;

    // -> Extract Banner URL
    // Don't ask me how it work. I had no idea too.
    let mut banner_url = String::new();
    {
        let class_re = Regex::new(r#"\.SimklLoginBg2\s*\{[^}]*\}"#).unwrap();
        if let Some(class_block) = class_re.find(&html) {
            let css = class_block.as_str();
            let url_re = Regex::new(r#"background-image\s*:\s*url\(['"]?(?P<url>[^'")]+)['"]?\)"#).unwrap();
            if let Some(caps) = url_re.captures(css) {
                let mut url = caps["url"].to_string();
                if url.starts_with("//") {
                    url = format!("https://wsrv.nl/?url=https:{}", url);
                }
                banner_url = url;
            }
        }
    }
    // <-

    let raw_thumbnail = vis.find(".SimklTVDetailPoster")
        .find("#detailPosterImg")
        .attr("src")
        .ok_or(anyhow::Error::msg("Thumbnail not found"))?
        .to_string();

    let thumbnail_url = format!("https://wsrv.nl/?url=https:{}", raw_thumbnail);

    
    let url = format!("https://simkl.com/movies{}", decode(id)?);


    let primary_raw_title = vis.find(".SimklTVAboutTitleText")
        .find("h2.headDetail").text();

    let secondary_raw_title = vis.find(".SimklTVAboutTitleText")
        .find("h1.headDetail").text();

    let title = match decode_html_entities(primary_raw_title.trim()).is_empty() {
        true => decode_html_entities(&secondary_raw_title.trim()).to_string(),
        false => decode_html_entities(&primary_raw_title.trim()).to_string()
    };
    
    let title_secondary = match decode_html_entities(secondary_raw_title.trim()).is_empty() {
        true => String::from(""),
        false => decode_html_entities(&secondary_raw_title.trim()).to_string()
    };



    let mut raw_description = vis.find(".SimklTVAboutDetailsText")
        .find(".full-text").text();

    if raw_description.is_empty() {
        raw_description = vis.find(".SimklTVAboutDetailsText").text();
    }

    let description = decode_html_entities(&raw_description.trim()).to_string();



    let raw_trailer_id = vis.find(".liteYoutube")
        .attr("id");

    let trailer_url = match raw_trailer_id {
        Some(id) => format!("https://www.youtube.com/watch?v={}&autoplay=1&vq=highres", id),
        None => String::from("")
    };



    let rating_container_ele = vis.find(".SimklTVAboutRatingBorder");

    let rating = format!("Rating: {}", rating_container_ele.find(".SimklTVRatingAverage").text());



    let mut imdb_id: String = String::from("");
    

    let a_ele = vis.find(".SimklTVAboutRatingBorder").find("a");
    
    for ele in a_ele {

        let imdb_url = match ele.get_attribute("href") {
            Some(url) => url.to_string(),
            None => continue
        };

        if imdb_url.contains("imdb") {
            imdb_id = imdb_url.split("/").nth(4)
                .ok_or(anyhow::Error::msg("Mal id not found"))?
                .to_string();
            break;
        }
    }


    let mut pictures: Vec<String> = vec![banner_url.clone()];

    // if !imdb_id.is_empty() {
    //     let mut new_headers = HeaderMap::new();
    //     new_headers.insert(USER_AGENT, HeaderValue::from_str(fake_user_agent::get_chrome_rua())?);
    //     new_headers.insert(ORIGIN, HeaderValue::from_str("https://www.imdb.com")?);
    //     new_headers.insert(REFERER, HeaderValue::from_str("https://www.imdb.com/")?);

    //     let res = client.get(format!("https://www.imdb.com/title/{}/mediaindex/?contentTypes=still_frame", imdb_id))
    //         .headers(new_headers)
    //         .send()
    //         .await?;

    //     let html = res.text().await?;


    //     let vis = Vis::load(&html)
    //         .map_err(|e| anyhow::Error::msg(e.to_string()))?;

    // }

    pictures.push(thumbnail_url.clone());


    let contextual: Vec<String> = vec!["Movies".to_string(), rating];

        
    let new_view_content_info = ViewContentInfo { 
        external_id: imdb_id,
        url,
        title,
        title_secondary,
        contextual,
        description,
        trailer_url,
        thumbnail_url: thumbnail_url.clone(),
        banner_url,
        countdown: -1,
        pictures,
        episodes: vec![vec![EpisodeInfo{
            title: "Full".to_string(),
            thumbnail_url: thumbnail_url.clone(),
        }]],
    };


    
    return Ok(new_view_content_info);
    // return Err(anyhow::Error::msg("Not implemented"));
}