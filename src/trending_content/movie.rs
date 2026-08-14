use anyhow::anyhow;
use chrono::{NaiveDate, Datelike};

use crate::trending_content::TrendingContentParams;

use super::{TrendingContentInfo};

pub async fn new(params: &TrendingContentParams) -> anyhow::Result<Vec<TrendingContentInfo>, anyhow::Error> {
	let url = "https://api.themoviedb.org/3/trending/movie/week";

	let querystring = [
		("language", "en-US"),
	];


	let client = reqwest::Client::new();
	let res = client.get(url)
		.query(&querystring)
		.header("accept", "application/json")
		.header("Authorization", format!("Bearer {}", params.tmdb_token))
		.send()
		.await?;

	if !res.status().is_success(){
		return Err(anyhow!("request failed: {}, {}", res.status(), res.text().await?));
	}

	let res_data = res
		.json::<serde_json::Value>()
		.await?;
	let item_list = res_data.get("results")
		.ok_or(anyhow!("results not exist"))?
		.as_array()
		.ok_or(anyhow!("results not an array"))?;

	let mut result: Vec<TrendingContentInfo> = Vec::new();
	

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

		let poster_path = item.get("poster_path")
			.ok_or(anyhow!("poster_path not exist"))?
			.as_str()
			.unwrap_or_default();

		let poster_url = format!("https://image.tmdb.org/t/p/original{}", poster_path);

		let release_date = item.get("release_date")
			.ok_or(anyhow!("release_date not exist"))?
			.as_str()
			.ok_or(anyhow!("release_date not a string"))?;

		let year = NaiveDate::parse_from_str(release_date, "%Y-%m-%d")?.year()
			.to_string();

		let rating = item.get("vote_average")
			.ok_or(anyhow!("vote_average not exist"))?
			.as_f64()
			.ok_or(anyhow!("vote_average not a number"))?
			.to_string();

		let data = TrendingContentInfo{
			id: id,
			title: title.to_string(),
			year: year,
			rating: rating,
			thumbnail_url: poster_url,
		};

		result.push(data);
	}

	return Ok(result);
}