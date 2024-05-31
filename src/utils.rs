use std::env;

pub fn base_url(input: &str) -> String {
    let base = env::var("REDIRECT_BASE").expect("Missing Redirect Base");
    
    if base.ends_with('/') {
        base.pop();
    }
    
    format!("{base}/{input}")
}