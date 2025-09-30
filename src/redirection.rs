use nix::errno::Errno;
use nix::sys::wait::waitpid;
use nix::unistd::{ForkResult, dup2, execv, fork};
use std::ffi::{CStr, CString};
use std::fs::OpenOptions;
use std::io;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::process;

// Import the manager and Job structure
use crate::background::BackgroundManager;

/// Parse I/O redirection tokens from a command
/// Returns (cmd_tokens, input_file, output_file, parse_ok)
pub fn parse_redirection(tokens: &Vec<String>) -> (Vec<String>, Option<String>, Option<String>, bool) {
    let mut cmd_tokens = Vec::new();
    let mut input_file: Option<String> = None;
    let mut output_file: Option<String> = None;
    let mut parse_ok = true;

    let mut i = 0;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "<" => {
                if i + 1 < tokens.len() {
                    input_file = Some(tokens[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: no input file specified after '<'");
                    parse_ok = false;
                    break;
                }
            }
            ">" => {
                if i + 1 < tokens.len() {
                    output_file = Some(tokens[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: no output file specified after '>'");
                    parse_ok = false;
                    break;
                }
            }
            _ => cmd_tokens.push(tokens[i].clone()),
        }
        i += 1;
    }

    (cmd_tokens, input_file, output_file, parse_ok)
}

/// Helper function to execute execv. It should return ! (never) on success.
fn execute_execv(path: &CString, args: &[&CStr]) -> ! {
    match execv(path, args) {
        Ok(_) => unreachable!(), // execv success means we never reach here
        Err(e) => {
            // Handle common execv errors
            let exit_code = match e {
                Errno::EACCES => 126, // Permission denied
                Errno::ENOENT => 127, // Command not found
                _ => 1,
            };
            eprintln!("Shell execution error: {}: {}", path.to_string_lossy(), e);
            process::exit(exit_code);
        }
    }
}

/// Execute a single command, possibly with I/O redirection or in background
pub fn execute_command(
    cmd_tokens: &Vec<String>,
    input_file: Option<&str>,
    output_file: Option<&str>,
    background: bool,
    manager: &mut BackgroundManager,
) -> io::Result<()> {
    if cmd_tokens.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No command provided",
        ));
    }


    // The background flag is passed in via the 'background' parameter.
    let tokens: Vec<String> = cmd_tokens.to_vec();
    let command_line_string = tokens.join(" "); // Store command for job report

    // 2. Convert command and arguments to CString
    let cmd_cstring = CString::new(tokens[0].clone())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    let args_cstring: Vec<CString> = tokens
        .iter()
        .map(|s| {
            CString::new(s.clone()).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
        })
        .collect::<Result<_, _>>()?;

    let args_ref: Vec<&CStr> = args_cstring.iter().map(|s| s.as_ref()).collect();

    // 3. Fork Process
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            // --- Child Process Setup (I/O Redirection) ---

            // Input redirection
            if let Some(input_path) = input_file {
                let input = OpenOptions::new()
                    .read(true)
                    .open(input_path)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                // Must use dup2 in child process
                dup2(input.as_raw_fd(), 0).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            }

            // Output redirection
            if let Some(output_path) = output_file {
                let output = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .mode(0o600) // -rw-------
                    .open(output_path)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                // Must use dup2 in child process
                dup2(output.as_raw_fd(), 1).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            }

            // Execute the command (uses the helper to handle errors)
            execute_execv(&cmd_cstring, &args_ref);
        }

        Ok(ForkResult::Parent { child }) => {
            // --- Parent Process Logic ---
            // Use the passed-in 'background' bool
            if background {
                // Background: Register the job with the manager (pid is i32, as expected)
                manager.add_job(child.as_raw(), command_line_string);
                Ok(())
            } else {
                // Foreground: Wait for child to finish
                match waitpid(child, None) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
                }
            }
        }
        Err(e) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Fork failed: {}", e),
        )),
    }
}
