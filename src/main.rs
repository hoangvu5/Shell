extern crate nix;

mod environment;
mod external;
mod lexer;
mod path_search;
mod prompt;
mod tilde;
mod redirection;
mod piping;
mod background;
mod builtin;

use environment::get_env;
use lexer::{get_input, get_tokens, TokenList};
use path_search::search_path;
use std::io;
use tilde::expand_tilde;
use external::execute_command;
use piping::execute_pipeline;
use background::BackgroundManager;
use builtin::execute_builtin;

fn main() -> io::Result<()> {
    let mut bg_manager = BackgroundManager::new();
    let mut history: Vec<String> = Vec::new();

    loop {
        bg_manager.check_and_cleanup_jobs();

        match get_input() {
            Ok(input) => {
                let mut tokens = get_tokens(&input);

                if tokens.items.is_empty() {
                    continue;
                }

                // Check for background '&'
                let mut background = false;
                if tokens.items.last().map(|t| t.as_str()) == Some("&") {
                    tokens.items.pop();
                    background = true;
                }

                // Expand tilde and environment variables
                for i in 0..tokens.items.len() {
                    let mut exp = tokens.items[i].clone();
                    exp = expand_tilde(&exp);
                    exp = get_env(&exp);
                    tokens.items[i] = exp;
                }

                // Check for built-in commands first
                if let Some(command) = tokens.items.first() {
                    let args = &tokens.items[1..];
                    match execute_builtin(command, args, &history, &mut bg_manager) {
                        Ok(true) => {
                            // Exit command was executed
                            return Ok(());
                        }
                        Ok(false) => {
                            // Built-in command executed successfully, add to history and continue
                            history.push(input.trim().to_string());
                            continue;
                        }
                        Err(_) => {
                            // Not a built-in command, continue to external command handling
                        }
                    }
                }


                // Split commands by '|'

                let mut pipeline: Vec<TokenList> = Vec::new();
                let mut current_cmd = TokenList { items: Vec::new() };

                for t in tokens.items {
                    if t == "|" {

                        if current_cmd.items.is_empty() {
                            eprintln!("Error: pipeline syntax error");
                            continue;
                        }

                        pipeline.push(current_cmd);
                        current_cmd = TokenList { items: Vec::new() };
                    } else {
                        current_cmd.items.push(t);
                    }
                }
                pipeline.push(current_cmd);


                // Handle pipeline
                if pipeline.len() > 1 {
                    // Check if the last command is empty (e.g., 'ls | ')
                    if pipeline.last().unwrap().items.is_empty() {
                         eprintln!("Error: pipeline syntax error ");
                         continue;
                    }

                    // Search PATH for each command in the pipeline
                    for cmd in &mut pipeline {
                        if !cmd.items.is_empty() && !cmd.items[0].contains('/') {
                            if let Some(full_path) = search_path(&cmd.items[0]) {
                                cmd.items[0] = full_path;
                            } else {
                                eprintln!("{}: command not found", cmd.items[0]);
                                continue;
                            }
                        }
                    }

                    if let Err(err) = execute_pipeline(&pipeline, background, &mut bg_manager) {
                        eprintln!("{}", err);
                    } else {
                        // Add to history only for successful pipeline commands
                        history.push(input.trim().to_string());
                    }
                    continue;
                }


                // I/O redirection for single command

                let tokens = &pipeline[0];
                let mut cmd_tokens = Vec::new();
                let mut input_file: Option<&str> = None;
                let mut output_file: Option<&str> = None;
                let mut parse_ok = true;

                let mut i = 0;
                while i < tokens.items.len() {
                    match tokens.items[i].as_str() {
                        "<" => {
                            if i + 1 < tokens.items.len() {
                                input_file = Some(&tokens.items[i + 1]);
                                i += 1;
                            } else {
                                eprintln!("Error: no input file specified after '<'");
                                parse_ok = false;
                                break;
                            }
                        }
                        ">" => {
                            if i + 1 < tokens.items.len() {
                                output_file = Some(&tokens.items[i + 1]);
                                i += 1;
                            } else {
                                eprintln!("Error: no output file specified after '>'");
                                parse_ok = false;
                                break;
                            }
                        }
                        _ => cmd_tokens.push(tokens.items[i].clone()),
                    }
                    i += 1;
                }

                if !parse_ok || cmd_tokens.is_empty() {
                    if cmd_tokens.is_empty() && parse_ok {
                         eprintln!("Error: no command specified");
                    }
                    continue;
                }

                // Search PATH if no slash

                if !cmd_tokens[0].contains('/') {
                    if let Some(full_path) = search_path(&cmd_tokens[0]) {
                        cmd_tokens[0] = full_path;
                    } else {
                        eprintln!("{}: command not found", cmd_tokens[0]);
                        continue;
                    }
                }


                // Execute (foreground or background)
                if let Err(err) = execute_command(&cmd_tokens, input_file, output_file, background, &mut bg_manager) {
                    eprintln!("{}", err);
                } else {
                    // Add to history only for successful external commands
                    history.push(input.trim().to_string());
                }

            }
            Err(err) => {
                eprintln!("\n{}", err);
                break;
            }
        }
    }
    Ok(())
}
