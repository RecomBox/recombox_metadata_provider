mod anime;
mod movies;
mod tv;

use serde::{Deserialize, Serialize};
use crate::global_types::{Source};


#[derive(Debug, Serialize, Deserialize)]
pub struct TrendingContent (Vec<TrendingContentInfo>);

#[derive(Debug, Serialize, Deserialize)]
pub struct TrendingContentInfo {
    pub id: String,
    pub title: String,
    pub year: String,
    pub rating: f32,
    pub thumbnail_url: String
}




pub async fn new(source: &Source) -> anyhow::Result<TrendingContent, anyhow::Error> {
    return match source {
        Source::Anime => Ok(anime::new().await?),
        Source::Movies => Ok(movies::new().await?),
        Source::TV => Ok(tv::new().await?)
    };
}

