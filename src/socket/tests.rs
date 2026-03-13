use super::*;

#[test]
fn test_get_socket_path() {
    let path = get_socket_path("my_timer");
    assert_eq!(path.to_string_lossy(), "/tmp/timerrs_my_timer.sock");
}

#[test]
fn test_get_socket_name_valid() {
    assert_eq!(get_socket_name("timerrs_my_timer.sock"), Some("my_timer"));
}
