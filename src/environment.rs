use std::env;

pub fn get_env(token: &str) -> String {
    if token.starts_with('$') {
        let var_name = &token[1..];

            // Get environment variable value
        env::var(var_name).unwrap_or_else(|_| String::from("$"))
    }
    else {
    token.to_string()
    }
}

