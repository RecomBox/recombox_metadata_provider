mod anime;
mod movie;
mod tv;


use serde::{Deserialize, Serialize};
use crate::global_types::{Source};

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewContentParams {
	pub tmdb_token: String,
	pub source: Source,
	pub id: String
	
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewContentInfo {
	pub external_id: ExternalID,
	pub url: String,
	pub title: String,
	pub title_secondary: String,
	pub thumbnail_url: String,
	pub banner_url: String,
	pub contextual: Vec<String>,
	pub description: String,
	pub trailer_url: String,
	pub countdown: i64,
	pub pictures: Vec<String>,
	// Seasons -> Episodes
	// Example [5,6] = Season 1 have 5 eps, and Season 2 have 6
	pub episodes: Vec<u64> 
	
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExternalID {
	pub mal: Option<String>,
	pub kitsu: Option<String>,
	pub imdb: Option<String>,
	pub tmdb: Option<String>,
	pub thetvdb: Option<String>
}



pub async fn new(params: &ViewContentParams) -> anyhow::Result<ViewContentInfo, anyhow::Error> {
	return match params.source {
		Source::Anime => Ok(anime::new(params).await?),
		Source::Movie => Ok(movie::new(params).await?),
		Source::TV => Ok(tv::new(params).await?),
		_ => todo!()
	};
}

