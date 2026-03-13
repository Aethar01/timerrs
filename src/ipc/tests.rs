use super::*;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_cleanup_nonexistent() {
    cleanup("nonexistent_timer_test_12345");
}

#[test]
fn test_ipc_socket_path_format() {
    let name = "test_timer";
    let expected = format!("/tmp/timerrs_{}.sock", name);

    let (tx, _rx) = mpsc::channel();
    start_listener(name.to_string(), tx);

    sleep(Duration::from_millis(50));

    let stream = UnixStream::connect(&expected);
    assert!(stream.is_ok());

    if let Ok(mut stream) = stream {
        let _ = stream.write_all(b"quit");
    }

    sleep(Duration::from_millis(50));
    cleanup(name);
}

#[test]
fn test_ipc_pause_command() {
    let name = "pause_test_timer";
    let socket_path = format!("/tmp/timerrs_{}.sock", name);

    let (tx, _rx) = mpsc::channel();
    start_listener(name.to_string(), tx);

    sleep(Duration::from_millis(50));

    if let Ok(mut stream) = UnixStream::connect(&socket_path) {
        let _ = stream.write_all(b"pause");
    }

    sleep(Duration::from_millis(50));
    cleanup(name);
}

#[test]
fn test_ipc_resume_command() {
    let name = "resume_test_timer";
    let socket_path = format!("/tmp/timerrs_{}.sock", name);

    let (tx, _rx) = mpsc::channel();
    start_listener(name.to_string(), tx);

    sleep(Duration::from_millis(50));

    if let Ok(mut stream) = UnixStream::connect(&socket_path) {
        let _ = stream.write_all(b"resume");
    }

    sleep(Duration::from_millis(50));
    cleanup(name);
}

#[test]
fn test_ipc_toggle_command() {
    let name = "toggle_test_timer";
    let socket_path = format!("/tmp/timerrs_{}.sock", name);

    let (tx, _rx) = mpsc::channel();
    start_listener(name.to_string(), tx);

    sleep(Duration::from_millis(50));

    if let Ok(mut stream) = UnixStream::connect(&socket_path) {
        let _ = stream.write_all(b"toggle");
    }

    sleep(Duration::from_millis(50));
    cleanup(name);
}

#[test]
fn test_ipc_invalid_command() {
    let name = "invalid_test_timer";
    let socket_path = format!("/tmp/timerrs_{}.sock", name);

    let (tx, _rx) = mpsc::channel();
    start_listener(name.to_string(), tx);

    sleep(Duration::from_millis(50));

    if let Ok(mut stream) = UnixStream::connect(&socket_path) {
        let _ = stream.write_all(b"invalid_command");
    }

    sleep(Duration::from_millis(50));
    cleanup(name);
}
