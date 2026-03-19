
use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue, USER_AGENT, ORIGIN, REFERER}
};

use visdom::Vis;
use fake_user_agent;
use html_escape::decode_html_entities;
use urlencoding::decode;


use super::{ViewContentInfo};

pub async fn new(id: &str) -> anyhow::Result<ViewContentInfo, anyhow::Error> {

    let mut new_headers = HeaderMap::new();
    new_headers.insert(USER_AGENT, HeaderValue::from_str(fake_user_agent::get_chrome_rua())?);
    new_headers.insert(ORIGIN, HeaderValue::from_str("https://simkl.com")?);
    new_headers.insert(REFERER, HeaderValue::from_str("https://simkl.com/")?);


    let client = Client::new();

    let res = client.get(format!("https://simkl.com/tv{}/episodes/", decode(id)?))
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

    


    let url = format!("https://simkl.com/tv{}", decode(id)?);


    let raw_title = vis.find(".SimklTVAboutTitleText")
        .find(".headDetail").text();

    let title = decode_html_entities(&raw_title.trim()).to_string();



    let raw_description = vis.find(".SimklTVAboutDetailsText")
        .find(".full-text").text();

    let description = decode_html_entities(&raw_description.trim()).to_string();



    let raw_trailer_id = vis.find(".liteYoutube")
        .attr("id")
        .ok_or(anyhow::Error::msg("Trailer id not found"))?
        .to_string();

    let trailer_url = format!("https://www.youtube.com/embed/{}?autoplay=1&vq=highres", raw_trailer_id);



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


    let mut pictures: Vec<String> = vec![];

    if !imdb_id.is_empty() {
        let mut new_headers = HeaderMap::new();
        new_headers.insert(USER_AGENT, HeaderValue::from_str(fake_user_agent::get_chrome_rua())?);
        new_headers.insert(ORIGIN, HeaderValue::from_str("https://www.imdb.com")?);
        new_headers.insert(REFERER, HeaderValue::from_str("https://www.imdb.com/")?);

        let res = client.get(format!("https://www.imdb.com/title/{}/mediaindex/?contentTypes=still_frame", imdb_id))
            .headers(new_headers)
            .send()
            .await?;

        let html = res.text().await?;

        let vis = Vis::load(&html)
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let banner_ele_li = vis.find(".ipc-page-section").find(".TlTHB")
            .find(".ipc-media").find(".ipc-image");

        for banner_ele in banner_ele_li {
            let url = match banner_ele.get_attribute("src") {
                Some(raw_url) => {
                    let raw_url = raw_url.to_string();
                    let parsed_url = raw_url.split("_V1_").nth(0)
                        .ok_or(anyhow::Error::msg("Mal url found"))?
                        .to_string();
                    format!("{}_V1_.jpg", parsed_url)
                },
                None => continue
            };
            pictures.push(url);
        }

    }

    pictures.push(thumbnail_url.clone());


    let contextual: Vec<String> = vec!["Movies".to_string(), rating];

    let banner_url = pictures.get(0).unwrap_or(&"".to_string()).clone();

    let mut episodes: Vec<Vec<String>> = vec![];

    let season_ele_li = vis.find(".SimklTVEpisodesBlock").find("tr");

    for season_ele in season_ele_li {
        

        let season_vis = Vis::load(season_ele.html())
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let season_head = season_vis.find(".SimklTVAboutTabsDetailsSeasonHead").text();
        
        if !(decode_html_entities(season_head.trim()).is_empty()) {
            continue; // Season Head Title tr block dont have episodes.
        }

        let eps_ele = season_vis.find(".goEpisode");
        
        let mut new_episodes: Vec<String> = vec![];
        for ep_ele in eps_ele {
            let ep_vis = Vis::load(ep_ele.html())
                .map_err(|e| anyhow::Error::msg(e.to_string()))?;

            let ep_number = ep_vis.find(".SimklTVEpisodesEpNumber").text();
            let ep_title = ep_vis.find(".SimklTVEpisodesEpTitle").text();

            let episode_title = format!("{}: {}", decode_html_entities(ep_number.trim()), decode_html_entities(ep_title.trim()));

            new_episodes.push(episode_title);
        }
        if new_episodes.len() == 0 {
            continue;
        }
        episodes.push(new_episodes);
    }

    let mut countdown: i64 = -1;

    let res = client.get(format!("https://countdown.tv{}", decode(id)?))
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
        url,
        title,
        contextual,
        description,
        trailer_url,
        thumbnail_url,
        banner_url,
        countdown,
        pictures,
        episodes: episodes,
    };

    
    
    return Ok(new_view_content_info);
    // return Err(anyhow::Error::msg("Not implemented"));
}