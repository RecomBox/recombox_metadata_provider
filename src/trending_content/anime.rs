
use anyhow::anyhow;
use chrono::{Datelike, NaiveDate};

use super::{TrendingContentInfo};

pub async fn new() -> anyhow::Result<Vec<TrendingContentInfo>, anyhow::Error> {

	let url = format!("https://kitsu.io/api/edge/trending/anime");

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

  

  let item_list = res_data.get("data")
    .ok_or(anyhow!("data not exist"))?
    .as_array()
    .ok_or(anyhow!("data not an array"))?;

  let mut result: Vec<TrendingContentInfo> = Vec::new();

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
    
    let rating = match attributes.get("averageRating"){
      Some(avg_rating) => {
        println!("{:?}", avg_rating);
        match avg_rating
          .as_str()
          .unwrap_or_default()
          .parse::<f32>(){
            Ok(raw_avg_rating) => {
              format!("{:.2}", raw_avg_rating / 10.0)
              
            },
            Err(_) => "".to_string(),
          }
      },
      None => "".to_string(),
    };

		let start_date = attributes.get("startDate")
      .ok_or(anyhow!("startDate not exist"))?
      .as_str()
      .ok_or(anyhow!("startDate not a string"))?;

    let start_year = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")?.year()
			.to_string();

    
    let end_year = match attributes.get("endDate") {
      Some(end_date_v) => {
        match end_date_v.as_str(){
          Some(end_date_str) => {
            NaiveDate::parse_from_str(
              end_date_str, 
              "%Y-%m-%d"
            )?.year().to_string()
          },
          None => "".to_string(),
        }
      },
      None => "".to_string(),
    };

    let year = match end_year.is_empty() {
      true => start_year,
      false => {
        if start_year == end_year {
          start_year
        } else {
          format!("{} - {}", start_year, end_year)
        }
      },
    };


    let data = TrendingContentInfo{
      id: id.to_string(),
      title: title.to_string(),
			rating: rating,
			year,
			thumbnail_url: poster_url.to_string(),
      
    };

    result.push(data);
  }

  println!("{:?}", result);

  return Ok(result);
}