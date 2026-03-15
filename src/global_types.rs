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
}
