mod socket;

use clap::{Parser, Subcommand};
use std::fs;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(version, about = "Control running timerrs instances", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    /// List all running timers
    List,
    /// Pause a timer
    Pause { name: String },
    /// Resume a timer
    Resume { name: String },
    /// Toggle a timer's paused state
    Toggle { name: String },
    /// Quit a timer
    Quit { name: String },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::List => list_timers(),
        Command::Pause { name } => send_command(&name, "pause"),
        Command::Resume { name } => send_command(&name, "resume"),
        Command::Toggle { name } => send_command(&name, "toggle"),
        Command::Quit { name } => send_command(&name, "quit"),
    }
}

fn list_timers() {
    let mut found = false;

    if let Ok(entries) = fs::read_dir(socket::SOCKET_DIR) {
        let timers = entries.flatten().filter_map(|entry| {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_str()?;
            let name = socket::get_socket_name(file_name_str)?.to_string();
            Some((entry.path(), name))
        });

        for (path, name) in timers {
            if UnixStream::connect(&path).is_ok() {
                println!("{}", name);
                found = true;
            } else {
                // Clean up dead sockets
                let _ = fs::remove_file(path);
            }
        }
    }

    if !found {
        println!("No running timers found.");
    }
}

fn send_command(name: &str, cmd: &str) {
    let socket_path = socket::get_socket_path(name);

    let mut stream = match UnixStream::connect(&socket_path) {
        Ok(stream) => stream,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::ConnectionRefused {
                let _ = fs::remove_file(&socket_path);
                eprintln!("Timer '{}' is not running.", name);
            } else if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!("Timer '{}' is not running.", name);
            } else {
                eprintln!(
                    "Error connecting to timer '{}' at {}: {}",
                    name,
                    socket_path.display(),
                    e
                );
            }
            exit(1);
        }
    };

    if let Err(e) = stream.write_all(cmd.as_bytes()) {
        eprintln!("Error writing to socket: {}", e);
        exit(1);
    }
}
