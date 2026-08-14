
use anyhow::anyhow;
use serde_json::Value;


use crate::view_content::{ ExternalID, ViewContentParams};

use super::{ViewContentInfo};

pub async fn new(params: &ViewContentParams) -> anyhow::Result<ViewContentInfo, anyhow::Error> {
	let url = format!("https://api.themoviedb.org/3/movie/{}", params.id);

	let querystring = [
		("language", "en-US"),
		("append_to_response", "videos,external_ids")
	];


	let client = reqwest::Client::new();
	let res = client.get(url)
		.query(&querystring)
		.header("accept", "application/json")
		.header("Authorization", format!("Bearer {}", params.tmdb_token))
		.send()
		.await?;

	if !res.status().is_success(){
		return Err(anyhow!("request failed: {}", res.status()));
	}

	let res_data = res
		.json::<serde_json::Value>()
		.await?;

	let item = res_data.as_object()
		.ok_or(anyhow!("results not an object"))?;
	
	let title = item.get("original_title")
		.ok_or(anyhow!("title not exist"))?
		.as_str()
		.ok_or(anyhow!("title not a string"))?;

	let secondary_title = item.get("title")
		.ok_or(anyhow!("title not exist"))?
		.as_str()
		.ok_or(anyhow!("title not a string"))?;

	let description = item.get("overview")
		.ok_or(anyhow!("overview not exist"))?
		.as_str()
		.ok_or(anyhow!("overview not a string"))?;

	let banner_path = item.get("backdrop_path")
		.ok_or(anyhow!("backdrop_path not exist"))?
		.as_str()
		.unwrap_or_default();

	let banner_url = format!("https://image.tmdb.org/t/p/original{}", banner_path);

	let poster_path = item.get("poster_path")
			.ok_or(anyhow!("bposter_path not exist"))?
			.as_str()
			.unwrap_or_default();

	let poster_url = format!("https://image.tmdb.org/t/p/original{}", poster_path);
		

	// Contextual
	let mut contextual: Vec<String> = Vec::new();

	contextual.push(String::from("Movie"));

	let raw_rating = item.get("vote_average")
		.ok_or(anyhow!("vote_average not exist"))?
		.as_f64()
		.ok_or(anyhow!("vote_average not a number"))?;
		
	let rating = (raw_rating * 100.0).round() / 100.0;

	contextual.push(String::from(format!("Rating: {}", rating.to_string())));

	contextual.push(String::from(
		item.get("original_language")
			.ok_or(anyhow!("original_language not exist"))?
			.as_str()
			.ok_or(anyhow!("original_language not a string"))?
			.to_uppercase()
	));
	// <-

	let pictures = [
		poster_url.to_string(),
		banner_url.to_string()
	];

	let videos = item.get("videos").and_then(|f| f.get("results"))
		.ok_or(anyhow!("videos not exist"))?
		.as_array()
		.ok_or(anyhow!("videos not an array"))?;

	let trailer_url = if let Some(trailer) = videos.iter().find(|v| {
		v.get("site").and_then(Value::as_str) == Some("YouTube")
			&& v.get("type").and_then(Value::as_str) == Some("Trailer")
			&& v.get("official").and_then(Value::as_bool) == Some(true)
	}) {
		trailer.get("key")
			.and_then(Value::as_str)
			.map(|key| format!("https://www.youtube.com/watch?v={}", key))
			.unwrap_or_default()
	} else {
		videos.first()
			.and_then(|first| first.get("key").and_then(Value::as_str))
			.map(|key| format!("https://www.youtube.com/watch?v={}", key))
			.unwrap_or_default()
	};

	let url = format!(
		"https://www.themoviedb.org/{}/{}",
		params.source.to_string(),
		params.id
	);

	let imdb_id = item.get("external_ids")
    .and_then(|f| f.get("imdb_id"))
		.ok_or(anyhow!("imdb_id not exist"))?
		.as_str()
		.ok_or(anyhow!("imdb_id not a string"))?
		.to_string();

	let external_id = ExternalID{
		tmdb: Some(params.id.to_string()),
		imdb: Some(imdb_id.to_string()),
		..Default::default()
	};


	let data = ViewContentInfo{
		url: url.to_string(),
		external_id: external_id,
		title: title.to_string(),
		title_secondary: secondary_title.to_string(),
		thumbnail_url: poster_url.to_string(),
		banner_url: banner_url.to_string(),
		contextual: contextual,
		trailer_url: trailer_url,
		description: description.to_string(),
		pictures: pictures.to_vec(),
		countdown: -1,
		episodes: vec![1],
	};

	return Ok(data);
	
}