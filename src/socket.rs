use std::path::PathBuf;

pub const SOCKET_DIR: &str = "/tmp";
pub const SOCKET_PREFIX: &str = "timerrs_";
pub const SOCKET_EXTENSION: &str = ".sock";

pub fn get_socket_path(name: &str) -> PathBuf {
    PathBuf::from(format!(
        "{}/{}{}{}",
        SOCKET_DIR, SOCKET_PREFIX, name, SOCKET_EXTENSION
    ))
}

#[allow(dead_code)]
pub fn get_socket_name(file_name: &str) -> Option<&str> {
    if file_name.starts_with(SOCKET_PREFIX) && file_name.ends_with(SOCKET_EXTENSION) {
        Some(&file_name[SOCKET_PREFIX.len()..file_name.len() - SOCKET_EXTENSION.len()])
    } else {
        None
    }
}

#[cfg(test)]
mod tests;
