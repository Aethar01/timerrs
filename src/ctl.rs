use clap::{Parser, ValueEnum};
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(version, about = "Control running timerrs instances", long_about = None)]
struct Args {
    /// Name of the timer to control (Un-named timers cannot be controlled)
    name: String,

    /// Command to send to the timer
    #[arg(value_enum)]
    command: Command,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Command {
    Pause,
    Resume,
    Toggle,
    Quit,
}

impl Command {
    fn as_str(&self) -> &'static str {
        match self {
            Command::Pause => "pause",
            Command::Resume => "resume",
            Command::Toggle => "toggle",
            Command::Quit => "quit",
        }
    }
}

fn main() {
    let args = Args::parse();

    let socket_path = format!("/tmp/timerrs_{}.sock", args.name);

    let mut stream = match UnixStream::connect(&socket_path) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!(
                "Error connecting to timer '{}' at {}: {}",
                args.name, socket_path, e
            );
            exit(1);
        }
    };

    if let Err(e) = stream.write_all(args.command.as_str().as_bytes()) {
        eprintln!("Error writing to socket: {}", e);
        exit(1);
    }
}
