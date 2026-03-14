#[cfg(test)]
mod tests {

    

    // #[tokio::test]
    async fn featured_content() {
        use crate::featured_content;

        let result = featured_content::new(&crate::global_types::Source::TV).await.unwrap();
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn trending_content() {
        use crate::trending_content;

        let result = trending_content::new(&crate::global_types::Source::Anime).await.unwrap();
        println!("{:?}", result);
    }
}
