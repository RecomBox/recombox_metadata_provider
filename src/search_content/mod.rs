mod anime;
mod movie;
mod tv;

use serde::{Deserialize, Serialize};
use crate::global_types::{Source};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchContentParams {
	pub tmdb_token: String,
	pub source: Source,
	pub search: String,
	pub page: u64,
}




#[derive(Debug, Serialize, Deserialize)]
pub struct SearchContentInfo {
	pub id: String,
	pub title: String,
	pub year: String,
	pub thumbnail_url: String
}


pub async fn new(params: &SearchContentParams) -> anyhow::Result<Vec<SearchContentInfo>> {
	return match params.source {
		Source::Anime => Ok(anime::new(params).await?),
		Source::Movie => Ok(movie::new(params).await?),
		Source::TV => Ok(tv::new(params).await?),
		_ => todo!()
	};
}

