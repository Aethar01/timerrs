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
