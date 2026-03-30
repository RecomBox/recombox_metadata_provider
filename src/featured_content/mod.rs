mod anime;
mod movies;
mod tv;

use serde::{Deserialize, Serialize};
use crate::global_types::{Source};


#[derive(Debug, Serialize, Deserialize)]
pub struct FeaturedContent (pub Vec<FeaturedContentInfo>);

#[derive(Debug, Serialize, Deserialize)]
pub struct FeaturedContentInfo {
    pub id: String,
    pub title: String,
    pub contextual: Vec<String>,
    pub short_description: String,
    pub banner_url: String
}




pub async fn new(source: &Source) -> anyhow::Result<FeaturedContent, anyhow::Error> {
    return match source {
        Source::Anime => Ok(anime::new().await?),
        Source::Movies => Ok(movies::new().await?),
        Source::TV => Ok(tv::new().await?)
    };
}

