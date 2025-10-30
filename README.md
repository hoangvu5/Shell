# Shell

The purpose of this project is to design and develop a comprehensive shell interface that enhances process control, user interaction, and error handling mechanisms. Submission for Fall 2025 - COP4610 - Project 1.

## File Listing

```
root/
├── .vscode/
├── src/
    ├── background.rs
    ├── environment.rs
    ├── external.rs
    ├── lexer.rs
    ├── main.rs
    ├── path_search.rs
    ├── piping.rs
    ├── prompt.rs
    ├── redirection.rs
    ├── tilde.rs
    ├── builtin.rs
├── .gitattributes
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── README.md

```

## How to Compile & Execute

### Requirements

- **Compiler**: `rustc` for Rust.
- **Dependencies**:
  nix = { version = "0.27", features = ["process", "fs"] }
  signal-hook = "0.3"
  regex = "1.10"

### Compilation

- cargo build

### Execution

- cargo run
  This will run the program ...

## Development Log

Each member records their contributions here.

### Kyle Boerger

| Date       | Work Completed / Notes |
| ---------- | ---------------------- |
| 2025-09-18 | Tilde Expansion        |
| 2025-09-23 | External Command Exec  |
| 2025-09-27 | Background Processing  |

### Hoang Vu

| Date       | Work Completed / Notes |
| ---------- | ---------------------- |
| 2025-09-14 | Tokenization           |
| 2025-09-21 | $PATH Search           |
| 2025-09-28 | Internal Command Exec  |

### Bao Le

| Date       | Work Completed / Notes |
| ---------- | ---------------------- |
| 2025-09-15 | Prompt                 |
| 2025-09-18 | Environment Variables  |
| 2025-09-26 | I/O Redirection        |
| 2025-09-28 | Piping                 |

## Meetings

Document in-person meetings, their purpose, and what was discussed.

| Date       | Attendees   | Topics Discussed | Outcomes / Decisions |
| ---------- | ----------- | ---------------- | -------------------- |
| 2025-09-09 | All members | First meeting    | Division of labor    |

Most team communitcation is done using Discord, there are no conflict and every process is finish smoothly
