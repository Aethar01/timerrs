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
    let prefix: &'static str = "timerrs_";
    let extension: &'static str = ".sock";

    match args.command {
        Command::List => {
            let mut found = false;
            if let Ok(entries) = fs::read_dir("/tmp") {
                for entry in entries.flatten() {
                    if let Some(file_name) = entry.file_name().to_str()
                        && file_name.starts_with(prefix)
                        && file_name.ends_with(extension)
                    {
                        let name = &file_name[prefix.len()..file_name.len() - extension.len()];
                        println!("{}", name);
                        found = true;
                    }
                }
            }
            if !found {
                println!("No running timers found.");
            }
        }
        Command::Pause { name } => send_command(&name, "pause"),
        Command::Resume { name } => send_command(&name, "resume"),
        Command::Toggle { name } => send_command(&name, "toggle"),
        Command::Quit { name } => send_command(&name, "quit"),
    }
}

fn send_command(name: &str, cmd: &str) {
    let socket_path = format!("/tmp/timerrs_{}.sock", name);

    let mut stream = match UnixStream::connect(&socket_path) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!(
                "Error connecting to timer '{}' at {}: {}",
                name, socket_path, e
            );
            exit(1);
        }
    };

    if let Err(e) = stream.write_all(cmd.as_bytes()) {
        eprintln!("Error writing to socket: {}", e);
        exit(1);
    }
}
