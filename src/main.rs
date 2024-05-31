mod verify_state;
mod opencollective;
mod utils;

#[macro_use] extern crate rocket;

use dotenvy::dotenv;
use rocket::response::Redirect;
use rocket::{Build, Rocket, State};
use std::env;
use std::sync::Mutex;
use uuid::Uuid;
use crate::verify_state::VerificationState;

/**
 * The way this program works is through a multiple redirect system.
 *
 * The flow is as follows:
 *
 * 1. The user goes /linked-role
 * 2. The user is redirected to Open Collective's OAuth2 URL
 * 3. The user is redirected to /open-collective/redirect
 * 4. The user is redirected to Discord's OAuth2 URL
 * 5. The user is redirected to /discord/redirect
 * 6. An HTML page is shown to the user. Simultaneously, their
 *    metadata is updated and a webhook is sent to Discord with information.
 *
 * This works through using a JWT-like token encoding the state as through the various redirects.
 */

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();

    rocket::build()
        .manage(Mutex::new(VerificationState::new()))
        .mount("/", routes![linked_role, open_collective_redirect])
}

#[get("/linked-role")]
fn linked_role(verification_state: &State<Mutex<VerificationState>>) -> Redirect {
    let state_id = Uuid::new_v4().to_string();
    {
        let mut state_lock = verification_state.lock().expect("Failed to lock state");
        state_lock.generate(state_id.clone(), 900);
    }

    let url = format!("https://opencollective.com/oauth/authorize?client_id={}&response_type=code&redirect_uri={}&scope=account&state={}",
                      env::var("OPEN_COLLECTIVE_CLIENT_ID").expect("Missing Open Collective Client ID"), utils::base_url("open-collective/redirect"), state_id);

    Redirect::to(url)
}

#[get("/open-collective/redirect?<code>&<state>")]
fn open_collective_redirect(
    verification_state: &State<Mutex<VerificationState>>,
    code: String,
    state: &str,
) -> Result<Redirect, &'static str> {
    let mut state_lock = verification_state.lock().expect("Failed to lock state");
    return match state_lock.verify(state) {
        Ok(_) => Ok({
            let state_id = Uuid::new_v4().to_string();
            {
                state_lock.generate(state_id.clone(), 900);
            }

            let res = opencollective::get_data(code);

            Redirect::to(uri!("https://ithundxr.dev"))
        }),
        Err(_) => Err("Failed to verify state"),
    };
}