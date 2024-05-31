use chrono::Utc;
use std::collections::HashMap;

pub(crate) struct VerificationState {
    states: HashMap<String, i64>,
}

impl VerificationState {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }

    pub fn generate(&mut self, state_id: String, expiry: i64) {
        self.states
            .insert(state_id, Utc::now().timestamp() + expiry);
    }

    pub fn verify(&self, state_id: &str) -> Result<(), &'static str> {
        if let Some(&expiry_time) = self.states.get(state_id) {
            if expiry_time >= Utc::now().timestamp() {
                return Ok(());
            }
        }

        Err("State not found or expired")
    }
}
