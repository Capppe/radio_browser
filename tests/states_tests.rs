extern crate radio_browser;

#[cfg(test)]
mod tests {
    use radio_browser::state::{State, StateHandler};

    #[tokio::test]
    async fn test_states1() {
        let state_handler = StateHandler::new(None);

        let states: Vec<State> = state_handler.get().await.unwrap();

        println!("States: {:?}", states);

        assert!(states.len() > 0);
    }

    #[tokio::test]
    async fn test_states2() {
        let state_handler = StateHandler::new(None);

        let states: Vec<State> = state_handler.limit(1).get().await.unwrap();

        println!("States: {:?}", states);

        assert!(states.len() > 0);
    }

    #[tokio::test]
    async fn test_states3() {
        let state_handler = StateHandler::new(None);

        let states: Vec<State> = state_handler
            .filter("Germany")
            .filter("ber")
            .limit(10)
            .get()
            .await
            .unwrap();

        println!("States: {:?}", states);

        assert!(states.len() > 0);
    }
}
