extern crate authy;

#[cfg(test)]
mod api_app {
    const API_URL: &'static str = "https://sandbox-api.authy.com";
    const API_KEY: &'static str = "bf12974d70818a08199d17d5e2bae630";

    use super::authy::{Client, Status, AuthyError};
    use super::authy::api::app;

    #[test]
    fn details() {
        let mut c = Client::new(API_URL, API_KEY);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let (status, details) = app::details(&c).expect("Details of authy app");
        assert!(status.success);
        assert_eq!(details.name, "Sandbox App 1");
    }

    #[test]
    fn details_bad_key() {
        let mut c = Client::new(API_URL, "a_bad_key");
        c.retry_wait = 3000;
        c.retry_count = 10;
        let res = app::details(&c);

        match res {
            Err(AuthyError::UnauthorizedKey(Status{success, message, ..})) => {
                assert!(!success);
                assert_eq!(message, "Invalid API key");
            },
            other => {
                unreachable!("Expecting AuthyError::UnauthorizedKey: {:?}", other);
            },
        };
    }

    #[test]
    fn stats() {
        let mut c = Client::new(API_URL, API_KEY);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let (status, stats) = app::stats(&c).expect("Stats of authy app");
        assert!(status.success);
    }
}
