use nix::sys::wait::{WaitPidFlag, WaitStatus, waitpid};
use nix::unistd::Pid;

#[derive(Clone)]
pub struct Job {
    pub job_number: usize,
    pub pid: i32,
    pub command: String,
    pub finished: bool,
}

pub struct BackgroundManager {
    jobs: Vec<Job>,
    next_job_number: usize,
}

impl BackgroundManager {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            next_job_number: 1,
        }
    }

    /// Add a new job to the background manager
    pub fn add_job(&mut self, pid: i32, command: String) {
        let job = Job {
            job_number: self.next_job_number,
            pid,
            command,
            finished: false,
        };
        println!("[{}] {}", job.job_number, pid);
        self.jobs.push(job);
        self.next_job_number += 1;
    }

    /// Periodically check if any background jobs finished
    pub fn check_and_cleanup_jobs(&mut self) {
        let mut finished_jobs_to_report = Vec::new();

        // 1. Mark finished jobs and collect data for reporting
        for job in self.jobs.iter_mut() {
            if !job.finished {
                // Use Pid::from_raw(job.pid) to convert i32 to nix::unistd::Pid
                match waitpid(
                    nix::unistd::Pid::from_raw(job.pid),
                    Some(WaitPidFlag::WNOHANG),
                ) {
                    // Correctly combining Exited and Signaled statuses
                    Ok(WaitStatus::Exited(_, _) | WaitStatus::Signaled(_, _, _)) => {
                        job.finished = true;
                        // Clone the finished job to report it *outside* the mutable iteration loop
                        finished_jobs_to_report.push(job.clone());
                    }
                    _ => {} // Handles StillAlive, Stopped, Continued, and Errors
                }
            }
        }

        // 2. Print completion messages
        for job in finished_jobs_to_report {
            println!("[{}] + done {}", job.job_number, job.command);
        }

        // 3. Cleanup: Remove all entries that are marked finished
        self.jobs.retain(|job| !job.finished);
    }

    /// Print the list of active background jobs
    pub fn list_jobs(&self) {
        if self.jobs.is_empty() {
            println!("No active background processes.");
            return;
        }
        for job in &self.jobs {
            println!("[{}]+ {} {}", job.job_number, job.pid, job.command);
        }
    }

    /// Block until all tracked background jobs finish
    pub fn wait_all(&mut self) {
        // Wait for each job's pid to finish; ignore errors
        for job in &self.jobs {
            let _ = waitpid(Pid::from_raw(job.pid), None);
        }
        // After waiting, clear the list
        self.jobs.clear();
    }
}
