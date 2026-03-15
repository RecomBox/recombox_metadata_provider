mod anime;
mod movies;
mod tv;

use serde::{Deserialize, Serialize};
use crate::global_types::{Source};


#[derive(Debug, Serialize, Deserialize)]
pub struct SearchContent (Vec<SearchContentInfo>);

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchContentInfo {
    pub id: String,
    pub title: String,
    pub year: String,
    pub rank: Option<u64>,
    pub thumbnail_url: String
}


pub async fn new(source: &Source) -> anyhow::Result<SearchContent, anyhow::Error> {
    return match source {
        Source::Anime => Ok(anime::new(source,"love", 0, 0).await?),
        Source::Movies => Ok(movies::new(source,"love", 0, 0).await?),
        Source::TV => Ok(tv::new(source,"love", 0, 0).await?),
    };
}

