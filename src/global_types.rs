pub enum Source {
    Anime,
    Movies,
    TV
}

impl Source {
    pub fn to_string(&self) -> String {
        match self {
            Source::Anime => String::from("anime"),
            Source::Movies => String::from("movies"),
            Source::TV => String::from("tv")
        }
    }

    pub fn from_str(s: &str) -> Source {
        match s.to_lowercase().as_str() {
            "anime" => Source::Anime,
            "movies" => Source::Movies,
            "tv" => Source::TV,
            _ => Source::Anime
        }
    }
}
