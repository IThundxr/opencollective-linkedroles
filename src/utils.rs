use regex::Regex;
use crate::env_vars;

pub fn base_url(input: &str) -> &str {
    let regex = Regex::new("/$").unwrap();
    format!("{}/{}", regex.replace(&env_vars::REDIRECT_BASE, ""), input)
}