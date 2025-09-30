use std::env;
use std::fs;

pub fn build_prompt() -> String {
    let user = env::var("USER").unwrap_or( "unknown".to_string());
    let machine = fs::read_to_string("/etc/hostname").unwrap_or( "unknown".to_string()).trim().to_string();
    let pwd = env::current_dir().map(|p| p.display().to_string()).unwrap_or( "?".to_string());

    format!("{}@{}:{}> ", user, machine, pwd)
}
