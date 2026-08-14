use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Source {
    Anime,
    Movie,
    TV
}

impl Source {
    pub fn to_string(&self) -> String {
        match self {
            Source::Anime => String::from("anime"),
            Source::Movie => String::from("movie"),
            Source::TV => String::from("tv")
        }
    }

    pub fn from_str(s: &str) -> Source {
        match s.to_lowercase().as_str() {
            "anime" => Source::Anime,
            "movies" => Source::Movie,
            "movie" => Source::Movie,
            "tv" => Source::TV,
            _ => Source::Anime
        }
    }
}
