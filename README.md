# Project 1: Shell

The purpose of this project is to design and develop a comprehensive shell interface that enhances process control, user interaction, and error handling mechanisms.

## Group Members
- **Kyle Boerger**: krb22p@fsu.edu
- **Hoang Vu**: hmv23@fsu.edu
- **Bao Le**: bgl23@fsu.edu
## Division of Labor

### Part 0: Tokenization
- **Assigned to**: Hoang Vu, Kyle Boerger

### Part 1: Prompt
- **Assigned to**: Kyle Boerger, Bao Le

### Part 2: Environment Variables
- **Assigned to**: Hoang Vu, Bao Le

### Part 3: Tilde Expansion
- **Assigned to**: Kyle Boerger, Hoang Vu

### Part 4: $PATH Search
- **Assigned to**: Hoang Vu, Bao Le

### Part 5: External Command Execution
- **Assigned to**: Kyle Boerger, Bao Le

### Part 6: I/O Redirection
- **Assigned to**: Bao Le, Hoang Vu

### Part 7: Piping
- **Assigned to**: Bao Le, Kyle Boerger

### Part 8: Background Processing
- **Assigned to**: Bao Le, Hoang Vu

### Part 9: Internal Command Execution
- **Assigned to**: Hoang Vu, Kyle Boerger

### Extra Credit
- **Assigned to**: Kyle Boerger, Hoang Vu, Bao Le

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
- **Compiler**: `cargo` for Rust.
- **Dependencies**:
    nix = { version = "0.27", features = ["process", "fs"] }
    signal-hook = "0.3"
    regex = "1.10"


### Compilation
For a C/C++ example:
```bash
make
```
This will build the executable in ...
### Execution
```bash
make run
```
This will run the program ...

## Development Log
Each member records their contributions here.

### Kyle Boerger

| Date       | Work Completed / Notes |
|------------|------------------------|
| 2025-09-18 | Tilde Expansion        |
| 2025-09-23 | External Command Exec  |
| 2025-09-27 | Background Processing  |

### Hoang Vu

| Date       | Work Completed / Notes |
|------------|------------------------|
| 2025-09-14 | Tokenization           |
| 2025-09-21 | $PATH Search           |
| 2025-09-28 | Internal Command Exec  |



### Bao Le

| Date       | Work Completed / Notes |
|------------|------------------------|
| 2025-09-15 | Prompt                 |
| 2025-09-18 | Environment Variables  |
| 2025-09-26 | I/O Redirection        |
| 2025-09-28 | Piping                 |


## Meetings
Document in-person meetings, their purpose, and what was discussed.

| Date       | Attendees            | Topics Discussed | Outcomes / Decisions |
|------------|----------------------|------------------|-----------------------|
| 2025-09-09 | All members          | First meeting    | Division of labor     |


Most team communitcation is done using Discord, there are no conflict and every process is finish smoothly


## Bugs
- **Bug 1**: Performing cd with a non existence directory will prompt this following warning "cd: command not found"


## Extra Credit


## Considerations
[Description]
