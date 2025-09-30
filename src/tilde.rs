use std::env;

pub fn expand_tilde(token: &str) -> String {
    if token == "~" {
        env::var("HOME").unwrap_or_else(|_| String::from("~"))
    } else if token.starts_with("~/") {
        let home = env::var("HOME").unwrap_or_else(|_| String::from("~"));
        format!("{}/{}", home, &token[2..])
    } else {
        token.to_string()
    }
}
