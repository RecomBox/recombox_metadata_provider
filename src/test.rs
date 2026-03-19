#[cfg(test)]
mod tests {

    // #[tokio::test]
    async fn featured_content() {
        use crate::featured_content;
        use crate::global_types::Source;

        let result = featured_content::new(&crate::global_types::Source::Movies).await.unwrap();
        println!("{:?}", result);
    }

    // #[tokio::test]
    async fn trending_content() {
        use crate::trending_content;

        let result = trending_content::new(&crate::global_types::Source::Anime).await.unwrap();
        println!("{:?}", result);
    }

    // #[tokio::test]
    async fn search_content() {
        use crate::search_content;

        let result = search_content::new(&crate::global_types::Source::Anime).await.unwrap();
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn view_content() {
        use crate::view_content_info;

        // let result = view_content_info::new(&crate::global_types::Source::Anime, "%2F2211126%2Fyuusha-kei-ni-shosu-choubatsu-yuusha-9004-tai-keimu-kiroku").await.unwrap();
        
        // let result = view_content_info::new(&crate::global_types::Source::Movies, "%2F53906%2Fspider-man").await.unwrap();
        
        let result = view_content_info::new(&crate::global_types::Source::TV, "%2F2416005%2Fthe-pitt").await.unwrap();


        println!("{:?}", result);
    }
}
