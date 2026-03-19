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


pub async fn new(source: &Source, search: &str, sort: u64, offset: u64) -> anyhow::Result<SearchContent, anyhow::Error> {
    return match source {
        Source::Anime => Ok(anime::new(source, search, sort, offset).await?),
        Source::Movies => Ok(movies::new(source, search, sort, offset).await?),
        Source::TV => Ok(tv::new(source, search, sort, offset).await?),
    };
}

