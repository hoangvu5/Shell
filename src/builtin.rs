use std::env;
use std::path::Path;
use crate::background::BackgroundManager;

/// Execute built-in commands
/// Returns Ok(true) if shell should exit, Ok(false) if command was a builtin and executed
/// Returns Err if the command is not a builtin
pub fn execute_builtin(
    command: &str,
    args: &[String],
    history: &[String],
    bg_manager: &mut BackgroundManager,
) -> Result<bool, String> {
    match command {
        "exit" => {
            execute_exit(history, bg_manager)?;
            Ok(true) // Signal to exit the shell
        }
        "cd" => {
            execute_cd(args)?;
            Ok(false) // Continue shell execution
        }
        "jobs" => {
            execute_jobs(bg_manager)?;
            Ok(false) // Continue shell execution
        }
        _ => Err(format!("not a builtin")), // Not a built-in command
    }
}

/// Execute the exit command
fn execute_exit(history: &[String], bg_manager: &mut BackgroundManager) -> Result<(), String> {
    // Wait for any background processes to finish
    bg_manager.wait_all();

    // Display the last three valid commands
    let n = history.len();
    if n == 0 {
        println!("No valid commands were entered.");
    } else if n >= 3 {
        for cmd in &history[n - 3..] {
            println!("{}", cmd);
        }
    } else {
        println!("{}", history[n - 1]);
    }

    println!("Exiting shell.");
    Ok(())
}

/// Execute the cd command
fn execute_cd(args: &[String]) -> Result<(), String> {
    let target_path = if args.is_empty() {
        // If no arguments, change to $HOME
        env::var("HOME").map_err(|_| "HOME environment variable not set".to_string())?
    } else if args.len() > 1 {
        return Err("cd: too many arguments".to_string());
    } else {
        args[0].clone()
    };

    // Check if the target exists
    if !Path::new(&target_path).exists() {
        return Err(format!("cd: {}: No such file or directory", target_path));
    }

    // Check if the target is a directory
    if !Path::new(&target_path).is_dir() {
        return Err(format!("cd: {}: Not a directory", target_path));
    }

    // Change the directory
    env::set_current_dir(&target_path)
        .map_err(|e| format!("cd: {}: {}", target_path, e))?;

    Ok(())
}

/// Execute the jobs command
fn execute_jobs(bg_manager: &BackgroundManager) -> Result<(), String> {
    bg_manager.list_jobs();
    Ok(())
}
