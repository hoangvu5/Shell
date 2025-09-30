use std::io;

use crate::prompt;

#[derive(Debug, Default, Clone)]
pub struct TokenList {
    pub items: Vec<String>,
}

impl TokenList {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_token(&mut self, item: &str) {
        self.items.push(item.to_string());
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }
}

/// Prints prompt, reads input, trims newline/CRLF, returns input
pub fn get_input() -> io::Result<String> {
    use std::io::Write;
    print!("{}", prompt::build_prompt());
    io::stdout().flush()?; // make sure prompt is shown before reading

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }

    Ok(line)
}

/// Split input into tokens by space, collapsing consecutive spaces
pub fn get_tokens(input: &str) -> TokenList {
    let mut tokens = TokenList::new();

    for part in input.split(' ').filter(|s| !s.is_empty()) {
        tokens.add_token(part);
    }

    tokens
}
