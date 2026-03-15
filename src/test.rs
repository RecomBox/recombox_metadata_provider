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

        let result = trending_content::new(&crate::global_types::Source::Movies).await.unwrap();
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn search_content() {
        use crate::search_content;

        let result = search_content::new(&crate::global_types::Source::Movies).await.unwrap();
        println!("{:?}", result);
    }
}
