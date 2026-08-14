

use anyhow::anyhow;
use chrono::{NaiveDate, Datelike};

use super::{ SearchContentInfo};
use crate::search_content::SearchContentParams;

pub async fn new(params: &SearchContentParams) -> anyhow::Result<Vec<SearchContentInfo>, anyhow::Error> {

	let url = "https://api.themoviedb.org/3/search/movie";

	let querystring = [
		("query", String::from(&params.search)),
		("include_adult", String::from("true")),
		("language", String::from("en-US")),
		("page", String::from(&params.page.to_string())),
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

	let mut result: Vec<SearchContentInfo> = Vec::new();
	

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
			.ok_or(anyhow!("bposter_path not exist"))?
			.as_str()
			.unwrap_or_default();

		let poster_url = format!("https://image.tmdb.org/t/p/original{}", poster_path);
		
		let release_date = item.get("release_date")
			.ok_or(anyhow!("release_date not exist"))?
			.as_str()
			.ok_or(anyhow!("release_date not a string"))?;

		let year = NaiveDate::parse_from_str(release_date, "%Y-%m-%d")?.year()
			.to_string();

		let data = SearchContentInfo{
			id: id,
			title: title.to_string(),
			thumbnail_url: poster_url,
			year
		};

		result.push(data);
	}
	

	return Ok(result)
}