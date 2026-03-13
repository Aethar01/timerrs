use crate::input::InputEvent;
use std::fs;
use std::io::Read;
use std::os::unix::net::UnixListener;
use std::sync::mpsc::Sender;
use std::thread;

pub fn start_listener(name: String, tx: Sender<InputEvent>) {
    let socket_path = format!("/tmp/timerrs_{}.sock", name);

    // Remove existing socket if it exists
    let _ = fs::remove_file(&socket_path);

    let listener = match UnixListener::bind(&socket_path) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind to socket {}: {}", socket_path, e);
            return;
        }
    };

    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(mut s) => {
                    let mut buf = String::new();
                    if let Ok(_) = s.read_to_string(&mut buf) {
                        let event = match buf.as_str() {
                            "pause" => Some(InputEvent::Pause),
                            "resume" => Some(InputEvent::Resume),
                            "toggle" => Some(InputEvent::TogglePause),
                            "quit" => Some(InputEvent::Quit),
                            _ => None,
                        };

                        if let Some(e) = event {
                            let _ = tx.send(e);
                        }
                    }
                }
                Err(_) => break,
            }
        }
    });
}

pub fn cleanup(name: &str) {
    let socket_path = format!("/tmp/timerrs_{}.sock", name);
    let _ = fs::remove_file(socket_path);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::os::unix::net::UnixStream;
    use std::sync::mpsc;
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

        std::thread::sleep(Duration::from_millis(50));

        let stream = UnixStream::connect(&expected);
        assert!(stream.is_ok());

        if let Ok(mut stream) = stream {
            let _ = stream.write_all(b"quit");
        }

        std::thread::sleep(Duration::from_millis(50));
        cleanup(name);
    }

    #[test]
    fn test_ipc_pause_command() {
        let name = "pause_test_timer";
        let socket_path = format!("/tmp/timerrs_{}.sock", name);

        let (tx, _rx) = mpsc::channel();
        start_listener(name.to_string(), tx);

        std::thread::sleep(Duration::from_millis(50));

        if let Ok(mut stream) = UnixStream::connect(&socket_path) {
            let _ = stream.write_all(b"pause");
        }

        std::thread::sleep(Duration::from_millis(50));
        cleanup(name);
    }

    #[test]
    fn test_ipc_resume_command() {
        let name = "resume_test_timer";
        let socket_path = format!("/tmp/timerrs_{}.sock", name);

        let (tx, _rx) = mpsc::channel();
        start_listener(name.to_string(), tx);

        std::thread::sleep(Duration::from_millis(50));

        if let Ok(mut stream) = UnixStream::connect(&socket_path) {
            let _ = stream.write_all(b"resume");
        }

        std::thread::sleep(Duration::from_millis(50));
        cleanup(name);
    }

    #[test]
    fn test_ipc_toggle_command() {
        let name = "toggle_test_timer";
        let socket_path = format!("/tmp/timerrs_{}.sock", name);

        let (tx, _rx) = mpsc::channel();
        start_listener(name.to_string(), tx);

        std::thread::sleep(Duration::from_millis(50));

        if let Ok(mut stream) = UnixStream::connect(&socket_path) {
            let _ = stream.write_all(b"toggle");
        }

        std::thread::sleep(Duration::from_millis(50));
        cleanup(name);
    }

    #[test]
    fn test_ipc_invalid_command() {
        let name = "invalid_test_timer";
        let socket_path = format!("/tmp/timerrs_{}.sock", name);

        let (tx, _rx) = mpsc::channel();
        start_listener(name.to_string(), tx);

        std::thread::sleep(Duration::from_millis(50));

        if let Ok(mut stream) = UnixStream::connect(&socket_path) {
            let _ = stream.write_all(b"invalid_command");
        }

        std::thread::sleep(Duration::from_millis(50));
        cleanup(name);
    }
}
