#[cfg(test)]
mod tests {
    use dotenv::dotenv;

use crate::{featured_content::FeaturedContentParams, global_types::Source, search_content::SearchContentParams, trending_content::TrendingContentParams, view_content::ViewContentParams};
    
    async fn init(){
        dotenv::dotenv().ok();
    }

    // ==================================================
    // Note: Uncomment the #[tokio::test] attributes to run the tests
    // ==================================================

    // #[tokio::test]
    async fn featured_content() {
        init().await;

        use crate::featured_content;
        use crate::global_types::Source;

        let token = std::env::var("TMDB_RAT_TOKEN")
            .expect("TMDB_RAT_TOKEN must be set");

        let params = FeaturedContentParams{
            source: Source::Anime,
            tmdb_token: token
        };

        let result = featured_content::new(&params).await.unwrap();
        println!("{:?}", result);
    }

    // #[tokio::test]
    async fn trending_content() {
        init().await;
        use crate::trending_content;

        let token = std::env::var("TMDB_RAT_TOKEN")
            .expect("TMDB_RAT_TOKEN must be set");

        let params = TrendingContentParams{
            source: Source::TV,
            tmdb_token: token
        };


        let result = trending_content::new(&params).await.unwrap();
        println!("{:?}", result);
        
    }

    // #[tokio::test]
    async fn search_content() {
        init().await;
        use crate::search_content;

        let token = std::env::var("TMDB_RAT_TOKEN")
            .expect("TMDB_RAT_TOKEN must be set");

        let params = SearchContentParams{
            tmdb_token: token,
            source: Source::Anime,
            search: "spider".to_string(),
            page: 1,
        };

        let result = search_content::new(&params).await.unwrap();
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn view_content() {
        init().await;
        use crate::view_content;

        let token = std::env::var("TMDB_RAT_TOKEN")
            .expect("TMDB_RAT_TOKEN must be set");

        let params = ViewContentParams{
            tmdb_token: token,
            source: Source::Anime,
            id: "1".to_string(),
        };

        let result = view_content::new(&params).await.unwrap();
        


        println!("{:?}", result);
    }
}
