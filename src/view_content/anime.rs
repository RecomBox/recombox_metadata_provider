
use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue, USER_AGENT, ORIGIN, REFERER}
};
use serde_json::{Value};
use visdom::Vis;
use html_escape::decode_html_entities;
use urlencoding::decode;



use super::{ViewContentInfo, EpisodeInfo, ExternalID};

pub async fn new(id: &str) -> anyhow::Result<ViewContentInfo, anyhow::Error> {

    let mut new_headers = HeaderMap::new();
    new_headers.insert(USER_AGENT, HeaderValue::from_str("PostmanRuntime/7.53.0")?);
    new_headers.insert(ORIGIN, HeaderValue::from_str("https://simkl.com")?);
    new_headers.insert(REFERER, HeaderValue::from_str("https://simkl.com/")?);


    let client = Client::new();

    // -> Extract Links
    let res = client.get(format!("https://simkl.com/anime{}/", decode(id)?))
        .headers(new_headers.clone())
        .send()
        .await?;

    let html = res.text().await?;

    let vis = Vis::load(&html)
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;

    let links_ele = vis.find(".SimklTVAboutTabsDetailsLinks");

    let kitsu_ele = links_ele.find("a:contains(Kitsu)");

    let kitsu_url = kitsu_ele.attr("href")
        .ok_or(anyhow::anyhow!("Failed to find kitsu id"))?
        .to_string();

    let kitsu_id = kitsu_url.split("/").last()
        .ok_or(anyhow::anyhow!("Failed to find kitsu id"))?
        .to_string();

    let mal_url = links_ele.find("a:contains(MAL)").attr("href")
        .ok_or(anyhow::anyhow!("Failed to find kitsu id"))?
        .to_string();

    let mal_url_split: Vec<&str> = mal_url.split("/").into_iter().collect();

    let mal_id = mal_url_split.get(mal_url_split.len() - 2).unwrap_or(&"")
        .to_string();

    let imdb_url = links_ele.find("a:contains(IMDB)").attr("href")
        .ok_or(anyhow::anyhow!("Failed to find kitsu id"))?
        .to_string();

    let imdb_url_split: Vec<&str> = imdb_url.split("/").into_iter().collect();

    let imdb_id = imdb_url_split.get(imdb_url_split.len() - 2).unwrap_or(&"")
        .to_string();


    let external_id = ExternalID {
        mal: Some(mal_id),
        kitsu: Some(kitsu_id),
        imdb: Some(imdb_id),
        ..ExternalID::default()
    };


    // <- 



    let res = client.get(format!("https://simkl.com/anime{}/episodes/", decode(id)?))
        .headers(new_headers.clone())
        .send()
        .await?;
    
    let html = res.text().await?;


    let vis = Vis::load(&html)
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;

    


    let raw_thumbnail = match vis.find(".SimklTVDetailPoster")
        .find("#detailPosterImg")
        .attr("src") {
            Some(url) => url.to_string(),
            None => String::from("")
        };

    let thumbnail_url = format!("https://wsrv.nl/?url=https:{}", raw_thumbnail);

    

    let url = format!("https://simkl.com/anime{}", decode(id)?);


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


    let mut pictures:Vec<String> = vec![thumbnail_url.clone()];

    let mut banner_url= String::new();

    if let Some(kitsu_id) = &external_id.kitsu {
            
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
        external_id,
        url,
        title,
        title_secondary,
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