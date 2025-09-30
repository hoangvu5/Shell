use crate::background::BackgroundManager;
use nix::sys::wait::waitpid;
use nix::unistd::{ForkResult, dup2, execv, fork};
use std::ffi::CString;
use std::fs::OpenOptions;
use std::io::{self};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::process;

pub fn execute_command(
    cmd_tokens: &[String],
    input_file: Option<&str>,
    output_file: Option<&str>,
    background: bool,
    bg_manager: &mut BackgroundManager,
) -> io::Result<()> {
    if cmd_tokens.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No command provided",
        ));
    }

    let cmd_cstring = CString::new(cmd_tokens[0].clone())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let args_cstring: Vec<CString> = cmd_tokens
        .iter()
        .map(|s| {
            CString::new(s.clone()).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
        })
        .collect::<Result<_, _>>()?;

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            // Input redirection
            if let Some(input_path) = input_file {
                let input = OpenOptions::new()
                    .read(true)
                    .open(input_path)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                dup2(input.as_raw_fd(), 0).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            }

            // Output redirection
            if let Some(output_path) = output_file {
                let output = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .mode(0o600)
                    .open(output_path)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                dup2(output.as_raw_fd(), 1).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            }

            // Execute
            match execv(&cmd_cstring, &args_cstring) {
                Ok(_) => Ok(()),
                Err(_) => process::exit(1),
            }
        }
        Ok(ForkResult::Parent { child }) => {
            if background {
                // ✅ Register as background job instead of waiting
                bg_manager.add_job(child.as_raw(), cmd_tokens.join(" "));
                Ok(())
            } else {
                // Foreground → wait for completion
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
