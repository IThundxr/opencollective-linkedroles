use crate::verify_state::VerificationState;
use reqwest::{Client, RequestBuilder};
use std::sync::{Mutex, MutexGuard};

pub struct App {
    https: Client,
    verification_state: Mutex<VerificationState>,
}

impl App {
    pub fn new() -> Self {
        Self {
            https: Client::new(),
            verification_state: Mutex::new(VerificationState::new()),
        }
    }

    pub fn verification_state(&self) -> MutexGuard<'_, VerificationState> {
        self.verification_state
            .lock()
            .expect("Failed to lock verification state")
    }

    pub fn request_opencollective(&self, path: &'static str) -> RequestBuilder {
        self.https
            .post(format!("https://opencollective.com/{path}"))
    }
}
