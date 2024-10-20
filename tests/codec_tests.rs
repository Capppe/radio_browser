extern crate radio_browser;

#[cfg(test)]
mod tests {
    use radio_browser::codec::{Codec, CodecHandler, CodecOrder};

    #[tokio::test]
    async fn test_codecs1() {
        let codec_handler = CodecHandler::new(None);

        let codecs: Vec<Codec> = codec_handler.get().await.unwrap();

        println!("Codecs: {:?}", codecs);

        assert!(codecs.len() > 0);
    }

    #[tokio::test]
    async fn test_codecs2() {
        let codec_handler = CodecHandler::new(None);

        let codecs: Vec<Codec> = codec_handler
            .limit(10)
            .order(CodecOrder::StationCount)
            .reverse(true)
            .get()
            .await
            .unwrap();

        println!("Codecs: {:?}", codecs);

        assert!(codecs.len() > 0);
    }

    #[tokio::test]
    async fn test_codecs3() {
        let codec_handler = CodecHandler::new(None);

        let codecs: Vec<Codec> = codec_handler.filter("mp").limit(10).get().await.unwrap();

        println!("Codecs: {:?}", codecs);

        assert!(codecs.len() > 0);
    }
}
