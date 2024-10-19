extern crate radio_browser;

#[cfg(test)]
mod tests {
    use radio_browser::tag::TagHandler;

    #[tokio::test]
    async fn test_get_tags() {
        let tag_handler = TagHandler::new();

        let tags = tag_handler.tags(None).await.unwrap();

        println!("Tags: {:?}", tags);

        assert!(tags.len() > 0)
    }

    #[tokio::test]
    async fn test_get_tags_with_params() {
        let tag_handler = TagHandler::new();

        let tag = tag_handler.limit(1).tags(Some("jazz")).await.unwrap();

        println!("Tags: {:?}", tag);

        assert!(!tag.len() > 1)
    }
}
