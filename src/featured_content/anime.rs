use anyhow::anyhow;
use chrono::{Datelike, Utc};

use crate::featured_content::FeaturedContentParams;

use super::{FeaturedContentInfo};

pub async fn new(params: &FeaturedContentParams) -> anyhow::Result<Vec<FeaturedContentInfo>> {
	let url = format!("https://kitsu.io/api/edge/anime");

  let now = Utc::now();
  let query = [
    ("filter[seasonYear]", now.year().to_string()),
  ];

	let client = reqwest::Client::new();
  let res = client.get(url)
    .query(&query)
    .send()
    .await;

  let res_data = res?
    .json::<serde_json::Value>()
    .await?;

  

  let item_list = res_data.get("data")
    .ok_or(anyhow!("data not exist"))?
    .as_array()
    .ok_or(anyhow!("data not an array"))?;

  let mut result: Vec<FeaturedContentInfo> = Vec::new();

  for item in item_list{
    let id = item.get("id")
      .ok_or(anyhow!("id not exist"))?
      .as_str()
      .ok_or(anyhow!("id not a str"))?;

    let attributes = item.get("attributes")
      .ok_or(anyhow!("attributes not exist"))?
      .as_object()
      .ok_or(anyhow!("attributes not an object"))?;

    let title = attributes.get("canonicalTitle")
      .ok_or(anyhow!("canonicalTitle not exist"))?
      .as_str()
      .ok_or(anyhow!("canonicalTitle not a string"))?;

    let short_desc = attributes.get("synopsis")
      .ok_or(anyhow!("synopsis not exist"))?
      .as_str()
      .ok_or(anyhow!("synopsis not a string"))?;

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


    let data = FeaturedContentInfo{
      source: params.source.clone(),
      id: id.to_string(),
      title: title.to_string(),
      short_description: short_desc.to_string(),
      banner_url: banner_url.to_string(),
      contextual,
      
    };

    result.push(data);
  }

  println!("{:?}", result);

  return Ok(result);
}