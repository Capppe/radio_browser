extern crate radio_browser;

#[cfg(test)]
mod tests {
    use radio_browser::tag::{Tag, TagHandler};

    #[tokio::test]
    async fn test_tags1() {
        let tag_handler = TagHandler::new(None);

        let tags: Vec<Tag> = tag_handler.get().await.unwrap();

        println!("Tags: {:?}", tags);

        assert!(tags.len() > 0)
    }

    #[tokio::test]
    async fn test_tags2() {
        let tag_handler = TagHandler::new(None);

        let tag: Vec<Tag> = tag_handler.limit(1).get().await.unwrap();

        println!("Tags: {:?}", tag);

        assert!(!tag.len() > 1)
    }
}
