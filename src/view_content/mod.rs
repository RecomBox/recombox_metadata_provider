mod anime;
mod movies;
mod tv;


use serde::{Deserialize, Serialize};
use crate::global_types::{Source};

#[derive(Debug, Serialize, Deserialize)]
pub struct EpisodeInfo {
    pub title: String,
    pub thumbnail_url: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewContentInfo {
    pub external_id: String,
    pub url: String,
    pub title: String,
    pub thumbnail_url: String,
    pub banner_url: String,
    pub contextual: Vec<String>,
    pub description: String,
    pub trailer_url: String,
    pub countdown: i64,
    pub pictures: Vec<String>,
    pub episodes: Vec<Vec<EpisodeInfo>> // Seasons -> Episodes
    
}




pub async fn new(source: &Source, id: &str) -> anyhow::Result<ViewContentInfo, anyhow::Error> {
    return match source {
        Source::Anime => Ok(anime::new(id).await?),
        Source::Movies => Ok(movies::new(id).await?),
        Source::TV => Ok(tv::new(id).await?),
    };
}

