mod anime;
mod movies;
mod tv;

use serde::{Deserialize, Serialize};
use crate::global_types::{Source};

#[derive(Debug, Serialize, Deserialize)]
pub struct FeaturedContentParams {
	pub source: Source,
	pub tmdb_token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeaturedContentInfo {
	pub source: Source,
	pub id: String,
	pub title: String,
	pub contextual: Vec<String>,
	pub short_description: String,
	pub banner_url: String,
}


pub async fn new(params: &FeaturedContentParams) -> anyhow::Result<Vec<FeaturedContentInfo>, anyhow::Error> {
	return match params.source {
		Source::Anime => Ok(anime::new(params).await?),
		Source::Movie => Ok(movies::new(params).await?),
		Source::TV => Ok(tv::new(params).await?),
		_ => todo!()
	};
}

