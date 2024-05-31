use std::env;

pub static CLIENT_ID: String = env::var("OPEN_COLLECTIVE_CLIENT_ID").expect("Missing Open Collective Client ID");
pub static CLIENT_SECRET: String = env::var("OPEN_COLLECTIVE_CLIENT_SECRET").expect("Missing Open Collective Client Secret");
pub static SLUG: String = env::var("OPEN_COLLECTIVE_SLUG").expect("Missing Open Collective Slug");
pub static REDIRECT_BASE: String = env::var("REDIRECT_BASE").expect("Missing Redirect Base");