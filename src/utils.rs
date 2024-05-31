use std::env;
use regex::Regex;

pub fn base_url(input: &str) -> String {
    let regex = Regex::new("/$").unwrap();
    let base = env::var("REDIRECT_BASE").expect("Missing Redirect Base");
    format!("{}/{}", regex.replace(&base, ""), input)
}