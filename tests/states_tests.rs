extern crate radio_browser;

#[cfg(test)]
mod tests {
    use radio_browser::state::{State, StateHandler};

    #[tokio::test]
    async fn test_get_states() {
        let state_handler = StateHandler::new(None);

        let states: Vec<State> = state_handler.get().await.unwrap();

        println!("States: {:?}", states);

        assert!(states.len() > 0);
    }

    #[tokio::test]
    async fn test_get_states_by_country() {
        let state_handler = StateHandler::new(None);

        let states: Vec<State> = state_handler.get().await.unwrap();

        println!("States: {:?}", states);

        assert!(states.len() > 0);
    }
}
