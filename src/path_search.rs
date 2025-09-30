use std::env;
use std::fs;
use std::path::Path;

pub fn search_path(command: &str) -> Option<String> {
    if command.contains('/') {
        return None;
    }

    let path_var = match env::var("PATH") {
        Ok(path) => path,
        Err(_) => return None,
    };

    for directory in path_var.split(':') {
        let directory = if directory.is_empty() { "." } else { directory };

        let full_path = format!("{}/{}", directory, command);
        let path = Path::new(&full_path);

        if path.exists() && is_executable(path) {
            return Some(full_path);
        }
    }

    None
}

fn is_executable(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => {
            use std::os::unix::fs::PermissionsExt;
            let permissions = metadata.permissions();
            metadata.is_file() && (permissions.mode() & 0o111 != 0)
        }
        Err(_) => false,
    }
}
