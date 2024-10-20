extern crate radio_browser;

#[cfg(test)]
mod tests {
    use radio_browser::language::{Language, LanguageHandler};

    #[tokio::test]
    async fn test_languages1() {
        let language_handler = LanguageHandler::new(None);

        let languages: Vec<Language> = language_handler.get().await.unwrap();

        println!("Languages: {:?}", languages);

        assert!(languages.len() > 0);
    }

    #[tokio::test]
    async fn test_languages2() {
        let language_handler = LanguageHandler::new(None);

        let languages: Vec<Language> = language_handler.limit(1).get().await.unwrap();

        println!("Languages: {:?}", languages);

        assert!(languages.len() > 0);
    }

    #[tokio::test]
    async fn test_languages3() {
        let language_handler = LanguageHandler::new(None);

        let languages: Vec<Language> = language_handler
            .filter("german")
            .limit(10)
            .get()
            .await
            .unwrap();

        println!("Languages: {:?}", languages);

        assert!(languages.len() > 0);
    }
}
