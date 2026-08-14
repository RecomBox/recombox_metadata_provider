use anyhow::anyhow;

use crate::featured_content::FeaturedContentParams;

use super::{FeaturedContentInfo};

pub async fn new(params: &FeaturedContentParams) -> anyhow::Result<Vec<FeaturedContentInfo>> {
	let url = "https://api.themoviedb.org/3/movie/popular";

	let querystring = [
		("language", "en-US"),
		("page", "1"),
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
	let item_list = res_data.get("results")
		.ok_or(anyhow!("results not exist"))?
		.as_array()
		.ok_or(anyhow!("results not an array"))?;

	let mut result: Vec<FeaturedContentInfo> = Vec::new();
	

	for item in item_list{
		let id = item.get("id")
			.ok_or(anyhow!("id not exist"))?
			.as_u64()
			.ok_or(anyhow!("id not a number"))?
			.to_string();

		let title = item.get("original_title")
			.ok_or(anyhow!("title not exist"))?
			.as_str()
			.ok_or(anyhow!("title not a string"))?;

		let short_desc = item.get("overview")
			.ok_or(anyhow!("overview not exist"))?
			.as_str()
			.ok_or(anyhow!("overview not a string"))?;

		let banner_path = item.get("backdrop_path")
			.ok_or(anyhow!("backdrop_path not exist"))?
			.as_str()
			.unwrap_or_default();

		let banner_url = format!("https://image.tmdb.org/t/p/original{}", banner_path);

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

		let data = FeaturedContentInfo{
      source: params.source.clone(),
			id: id,
			title: title.to_string(),
			short_description: short_desc.to_string(),
			banner_url: banner_url,
			contextual: contextual,
		};

		result.push(data);
	}

	return Ok(result);
}