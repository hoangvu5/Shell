use crate::background::BackgroundManager;
use crate::lexer::TokenList;
use nix::sys::wait::waitpid;
use nix::unistd::Pid;
use nix::unistd::{ForkResult, close, dup2, execv, fork, pipe};
use std::ffi::CString;
use std::io;

pub fn execute_pipeline(
    commands: &[TokenList],
    background: bool,
    bg_manager: &mut BackgroundManager,
    ) -> io::Result<()> {
    if commands.is_empty() {
        return Ok(());
    }
    if commands.len() > 3 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Only accept up to 2 pipes",
        ));
    }

    let mut pipes = Vec::new();

    // Create pipes
    for _ in 0..commands.len() - 1 {
        pipes.push(pipe().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?);
    }

    let mut last_pid: Option<Pid> = None;

    for (i, cmd) in commands.iter().enumerate() {
        if cmd.items.is_empty() {
            continue;
        }

        let cstrings: Vec<CString> = cmd
            .items
            .iter()
            .map(|s| CString::new(s.as_str()).unwrap())
            .collect();

        match unsafe { fork().map_err(|e| io::Error::new(io::ErrorKind::Other, e))? } {
            ForkResult::Child => {

                if i > 0 {
                    let (read_end, _) = pipes[i - 1];
                    dup2(read_end, 0).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                }
                if i < commands.len() - 1 {
                    let (_, write_end) = pipes[i];
                    dup2(write_end, 1).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                }


                for &(r, w) in &pipes {
                    let _ = close(r);
                    let _ = close(w);
                }

                execv(&cstrings[0], &cstrings)
                    .map_err(|_| io::Error::new(io::ErrorKind::Other, "execv failed"))?;
                std::process::exit(1);
            }
            ForkResult::Parent { child } => {
                last_pid = Some(child);
            }
        }
    }

    // Close pipes in parent
    for &(r, w) in &pipes {
        let _ = close(r);
        let _ = close(w);
    }

    if background {
        if let Some(pid) = last_pid {
            let mut all_cmds: Vec<String> = Vec::new();
            for c in commands {
                all_cmds.extend(c.items.clone());
                all_cmds.push("|".to_string());
            }
            all_cmds.pop(); // remove trailing "|"
            bg_manager.add_job(pid.as_raw(), all_cmds.join(" "));
        }
        Ok(())
    } else {
        // Foreground: wait for all
        for _ in 0..commands.len() {
            let _ = waitpid(Pid::from_raw(-1), None);
        }
        Ok(())
    }
}
