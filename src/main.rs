//! The way this program works is through a multiple redirect system.
//!
//! The flow is as follows:
//!
//! 1. The user goes /linked-role
//! 2. The user is redirected to Open Collective's OAuth2 URL
//! 3. The user is redirected to /open-collective/redirect
//! 4. The user is redirected to Discord's OAuth2 URL
//! 5. The user is redirected to /discord/redirect
//! 6. An HTML page is shown to the user. Simultaneously, their
//!    metadata is updated and a webhook is sent to Discord with information.
//!
//! This works through using a JWT-like token encoding the state as through the various redirects.

mod app;
mod opencollective;
mod utils;
mod verify_state;

#[macro_use]
extern crate rocket;

use app::App;
use dotenvy::dotenv;
use rocket::{response::Redirect, Build, Rocket, State};
use std::env;
use uuid::Uuid;

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();

    rocket::build()
        .manage(App::new())
        .mount("/", routes![linked_role, open_collective_redirect])
}

#[get("/linked-role")]
fn linked_role(app: &State<App>) -> Redirect {
    let state_id = Uuid::new_v4();
    let mut state_lock = app.verification_state();

    let url = format!("https://opencollective.com/oauth/authorize?client_id={}&response_type=code&redirect_uri={}&scope=account&state={state_id}",
                      env::var("OPEN_COLLECTIVE_CLIENT_ID").expect("Missing Open Collective Client ID"), utils::base_url("open-collective/redirect"));

    state_lock.generate(state_id.to_string(), 900);

    Redirect::to(url)
}

#[get("/open-collective/redirect?<code>&<state>")]
fn open_collective_redirect(
    app: &State<App>,
    code: String,
    state: &str,
) -> Result<Redirect, &'static str> {
    let mut state_lock = app.verification_state();

    match state_lock.verify(state) {
        Ok(_) => {
            let state_id = Uuid::new_v4().to_string();

            state_lock.generate(state_id, 900);

            // TODO: make use of this res
            let _res = opencollective::get_data(app, code);

            Ok(Redirect::to(uri!("https://ithundxr.dev")))
        },

        Err(_) => Err("Failed to verify state"),
    }
}
