mod anime;
mod movie;
mod tv;

use serde::{Deserialize, Serialize};
use crate::global_types::{Source};

#[derive(Debug, Serialize, Deserialize)]
pub struct TrendingContentParams {
    pub tmdb_token: String,
    pub source: Source
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrendingContentInfo {
    pub id: String,
    pub title: String,
    pub year: String,
    pub rating: String,
    pub thumbnail_url: String
}




pub async fn new(params: &TrendingContentParams) -> anyhow::Result<Vec<TrendingContentInfo>> {
    return match params.source {
        Source::Anime => Ok(anime::new().await?),
        Source::Movie => Ok(movie::new(params).await?),
        Source::TV => Ok(tv::new(params).await?),
        _ => todo!()
    };
}

