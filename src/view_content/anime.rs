
use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue, USER_AGENT, ORIGIN, REFERER}
};
use serde_json::{Value};
use visdom::Vis;
use html_escape::decode_html_entities;
use urlencoding::decode;



use super::{ViewContentInfo, EpisodeInfo};

pub async fn new(id: &str) -> anyhow::Result<ViewContentInfo, anyhow::Error> {

    let mut new_headers = HeaderMap::new();
    new_headers.insert(USER_AGENT, HeaderValue::from_str("PostmanRuntime/7.53.0")?);
    new_headers.insert(ORIGIN, HeaderValue::from_str("https://simkl.com")?);
    new_headers.insert(REFERER, HeaderValue::from_str("https://simkl.com/")?);


    let client = Client::new();

    let res = client.get(format!("https://simkl.com/anime{}/episodes/", decode(id)?))
        .headers(new_headers)
        .send()
        .await?;
    
    let html = res.text().await?;


    let vis = Vis::load(&html)
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;

    


    let raw_thumbnail = vis.find(".SimklTVDetailPoster")
        .find("#detailPosterImg")
        .attr("src")
        .ok_or(anyhow::Error::msg("Thumbnail not found"))?
        .to_string();

    let thumbnail_url = format!("https://wsrv.nl/?url=https:{}", raw_thumbnail);

    

    let url = format!("https://simkl.com/anime{}", decode(id)?);


    let h2_raw_title = vis.find(".SimklTVAboutTitleText")
        .find("h2.headDetail").text();

    let h1_raw_title = vis.find(".SimklTVAboutTitleText")
        .find("h1.headDetail").text();

    let raw_title = match decode_html_entities(h2_raw_title.trim()).is_empty() {
        true => h1_raw_title,
        false => h2_raw_title
    };

    let title = decode_html_entities(&raw_title.trim()).to_string();


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



    let mut mal_id: String = String::from("");
    

    let a_ele = vis.find(".SimklTVAboutRatingBorder").find("a");
    
    for ele in a_ele {

        let mal_url = match ele.get_attribute("href") {
            Some(url) => url.to_string(),
            None => continue
        };

        if mal_url.contains("myanimelist") {
            mal_id = mal_url.split("/").nth(4)
                .ok_or(anyhow::Error::msg("Mal id not found"))?
                .to_string();
            break;
        }
    }

    let mut pictures:Vec<String> = vec![thumbnail_url.clone()];

    let mut banner_url= String::new();

    if !mal_id.is_empty() {
        let res = client.get(format!("https://kitsu.io/api/edge/mappings?filter[externalSite]=myanimelist/anime&filter[externalId]={}", mal_id))
            .send()
            .await?;

        let data: Value = res.json().await?;

        let kitsu_map_id: Option<String> = data.get("data")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.get("id"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());


        if let Some(kitsu_id) = kitsu_map_id {
            let res = client.get(format!("https://kitsu.io/api/edge/mappings/{}/relationships/item", kitsu_id))
                .send()
                .await?;

            let data: Value = res.json().await?;

            let kitsu_id = data.get("data")
                .ok_or("kitsu_id not found.")
                .map_err(|e| anyhow::Error::msg(e))?
                .get("id")
                .ok_or("kitsu_id not found.")
                .map_err(|e| anyhow::Error::msg(e))?
                .as_str()
                .ok_or("kitsu_id not found.")
                .map_err(|e| anyhow::Error::msg(e))?
                .to_string();
                
            let res = client.get(format!("https://kitsu.io/api/edge/anime/{}", kitsu_id))
                .send()
                .await?;

            let data: Value = res.json().await?;

            banner_url = match data.get("data")
                .and_then(|f| f.get("attributes"))
                .and_then(|f| f.get("coverImage"))
                .and_then(|f| f.get("original")) {
                    Some(url) => url.as_str()
                        .ok_or("url not found.")
                        .map_err(|e| anyhow::Error::msg(e))?
                        .to_string(),
                    None => String::new()
                };
            
            if !banner_url.is_empty() {
                pictures.push(banner_url.clone());
            }
                
        }

    }


    let contextual: Vec<String> = vec!["Anime".to_string(), rating];

    

    let eps_ele = vis.find(".SimklTVEpisodesBlock")
        .find(".goEpisode");

    let mut episodes: Vec<EpisodeInfo> = vec![];

    for ep_ele in eps_ele {
        let ep_vis = Vis::load(ep_ele.html())
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let ep_number = ep_vis.find(".SimklTVEpisodesEpNumber").text();
        let ep_title = ep_vis.find(".SimklTVEpisodesEpTitle").text();

        let episode_title = format!("{}: {}", decode_html_entities(ep_number.trim()), decode_html_entities(ep_title.trim()));

        let ep_thumbnail = match ep_vis.find("img.lazy").attr("data-original")
            .ok_or(anyhow::Error::msg("Ep thumbnail not found")) {
                Ok(url) => format!("https://wsrv.nl/?url=https:{}", url),
                Err(_) => "".to_string()
            };

        let new_ep_info = EpisodeInfo{
            title: episode_title,
            thumbnail_url: ep_thumbnail
        };
        episodes.push(new_ep_info);
    }

    let mut countdown: i64 = -1;

    let res = client.get(format!("https://animecountdown.com{}", decode(id)?))
        .send()
        .await?;

    let html = res.text().await?;

    let cd_vis = Vis::load(&html)
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;

    let cd_type_ele_li = cd_vis.find(".type-airing");

    for cd_type_ele in cd_type_ele_li {
        let cd_type_vis = Vis::load(cd_type_ele.html())
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let cd_content = cd_type_vis.find("countdown-content-page-item-left-desc");

        if !cd_content.text().to_lowercase().contains(&"Countdown to".to_lowercase()) {
            continue;
        }

        countdown = match cd_content.find("span").attr("data-ts") {
            Some(ts) => if ts.to_string().trim().is_empty() { 0 } else { ts.to_string().trim().parse()? },
            None => 0
        }
        
    }






    let new_view_content_info = ViewContentInfo { 
        external_id: mal_id,
        url,
        title,
        contextual,
        description,
        trailer_url,
        thumbnail_url,
        banner_url,
        countdown,
        pictures,
        episodes: vec![episodes],
    };

    
    
    return Ok(new_view_content_info);
    // return Err(anyhow::Error::msg("Not implemented"));
}