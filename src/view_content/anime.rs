
use anyhow::anyhow;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc, TimeZone};
use serde_json::Value;



use crate::view_content::ViewContentParams;

use super::{ViewContentInfo, ExternalID};

pub async fn new(params: &ViewContentParams) -> anyhow::Result<ViewContentInfo, anyhow::Error> {
  let url = format!("https://kitsu.io/api/edge/anime/{}", params.id);

	let client = reqwest::Client::new();
  let res = client.get(url)
    .send()
    .await?;

  if !res.status().is_success(){
		return Err(anyhow!("request failed: {}, {}", res.status(), res.text().await?));
	}

  let res_data = res
    .json::<serde_json::Value>()
    .await?;

  


  let item = res_data.get("data")
    .ok_or(anyhow!("data not exist"))?
    .as_object()
    .ok_or(anyhow!("data not an object"))?;


  let attributes = item.get("attributes")
    .ok_or(anyhow!("attributes not exist"))?
    .as_object()
    .ok_or(anyhow!("attributes not an object"))?;

  let title = attributes.get("canonicalTitle")
    .ok_or(anyhow!("canonicalTitle not exist"))?
    .as_str()
    .ok_or(anyhow!("canonicalTitle not a string"))?;

  let description = attributes.get("description")
    .ok_or(anyhow!("description not exist"))?
    .as_str()
    .ok_or(anyhow!("description not a string"))?;

  let cover_images_opt = attributes.get("coverImage")
    .unwrap_or_default()
    .as_object();

  let banner_url = match cover_images_opt {
    Some(cover_images) => {
      cover_images.get("original")
        .unwrap_or_default()
        .as_str()
        .unwrap_or_default()
    },
    None => "",
  };

  let poster_images_opt = attributes.get("posterImage")
    .unwrap_or_default()
    .as_object();

  let poster_url = match poster_images_opt {
    Some(poster_images) => {
      poster_images.get("original")
        .unwrap_or_default()
        .as_str()
        .unwrap_or_default()
    },
    None => "",
  };

  // Contextual
  let mut contextual = Vec::new();

  contextual.push(String::from("Anime"));
  
  match attributes.get("averageRating"){
    Some(avg_rating) => {
      println!("{:?}", avg_rating);
      match avg_rating
        .as_str()
        .unwrap_or_default()
        .parse::<f32>(){
          Ok(raw_avg_rating) => {
            let rating = format!("{:.2}", raw_avg_rating / 10.0);
            contextual.push(format!("Rating: {}",rating));
          },
          Err(_) => {},
        }
    },
    None => {},
  }

  

  match attributes.get("ageRating") {
    Some(age_rating) => contextual.push(age_rating.as_str().unwrap_or_default().to_string().to_uppercase()),
    None => {},
  }
  match attributes.get("status") {
    Some(d) => contextual.push(format!("{}", d.as_str().unwrap_or_default().to_string().to_uppercase())),
    None => {},
  }

  // <-

  let url = item.get("links")
    .and_then(|f| f.get("self"))
    .and_then(|f| f.as_str())
    .unwrap_or_default();

  let next_release = match attributes.get("nextRelease").and_then(|f| f.as_str()) {
    Some(d) => {
      // Parse into NaiveDate
      let date = NaiveDate::parse_from_str(d, "%Y-%m-%d")
          .expect("Invalid date format");

      // Attach midnight time
      let datetime = NaiveDateTime::new(date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());

      // Convert to UTC-aware DateTime
      let utc_datetime = Utc.from_utc_datetime(&datetime);

      // Get milliseconds since epoch
      utc_datetime.timestamp_millis()
    },
    None => -1,
  };

  let ep_count = attributes.get("episodeCount")
    .and_then(|f| f.as_u64())
    .unwrap_or_default();

  let pictures = [
    banner_url.to_string(),
    poster_url.to_string(),
  ];

  let yt_video_id = attributes.get("youtubeVideoId")
    .and_then(|f| f.as_str())
    .unwrap_or_default();

  let yt_link = format!("https://www.youtube.com/watch?v={}", yt_video_id);

  let external_id = match get_external_ids(&params).await {
    Ok(d) => d,
    Err(_) => ExternalID::default(),
  };

  let data = ViewContentInfo{

    url: url.to_string(),
    title: title.to_string(),
    title_secondary: title.to_string(),
    description: description.to_string(),
    banner_url: banner_url.to_string(),
    thumbnail_url: poster_url.to_string(),
    contextual,
    countdown: next_release,
    episodes: [ep_count].to_vec(),
    pictures: pictures.to_vec(),
    trailer_url: yt_link.to_string(),
    external_id,
  };


  
  return Ok(data);
}


async fn get_external_ids(params: &ViewContentParams) -> anyhow::Result<ExternalID>{

  let url = format!("https://kitsu.io/api/edge/anime/{}/mappings", params.id);

  let client = reqwest::Client::new();
  let res = client.get(url)
    .send()
    .await;

  let res_data = res?
    .json::<serde_json::Value>()
    .await?;

  let data_li = res_data.get("data")
    .ok_or(anyhow!("data not exist"))?
    .as_array()
    .ok_or(anyhow!("data not an array"))?;

  let mut external_id = ExternalID::default();

  for item in data_li {

    let attributes = item.get("attributes")
      .ok_or(anyhow!("attributes not exist"))?
      .as_object()
      .ok_or(anyhow!("attributes not an object"))?;

    let external_site = attributes.get("externalSite")
      .ok_or(anyhow!("externalSite not exist"))?
      .as_str()
      .ok_or(anyhow!("externalSite not a str"))?;

    
    match external_site {
      "thetvdb" => {
        let id = attributes.get("externalId")
          .and_then(|f| f.as_str())
          .unwrap_or_default();

        external_id.thetvdb = Some(id.to_string());
      },
      "myanimelist/anime" => {
        let id = attributes.get("externalId")
          .and_then(|f| f.as_str())
          .unwrap_or_default();

        external_id.mal = Some(id.to_string());
      }
      _ => {}
    };

    if let Some(thetvdb) = external_id.thetvdb.as_ref() {
      let url = format!(
        "https://api.themoviedb.org/3/find/{}?external_source=tvdb_id",
        thetvdb
      );

      let client = reqwest::Client::new();
      let res = client
        .get(&url)
        .bearer_auth(&params.tmdb_token) // <-- Bearer token here
        .send()
        .await?
        .error_for_status()? // ensure 2xx
        .json::<Value>()
        .await?;

      let tv_results = res.get("tv_results")
        .ok_or(anyhow!("tv_results not exist"))?
        .as_array()
        .ok_or(anyhow!("tv_results not an array"))?;

      if tv_results.len() > 0 {
        let id = tv_results[0].get("id")
          .and_then(|f| f.as_u64())
          .unwrap_or_default();
        external_id.tmdb = Some(id.to_string());
      }

    }

  }

  return Ok(external_id);
}