use clap::Parser;
use crossterm::style::Color;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Duration of the timer (e.g., "5m", "30s", "1h")
    #[arg(value_parser = parse_duration)]
    pub duration: Duration,

    /// Name of the timer
    #[arg(short, long)]
    pub name: Option<String>,

    /// Color of the filled progress bar
    #[arg(short, long, default_value = "white", value_parser = parse_color)]
    pub color: Color,

    /// Run the timer in fullscreen mode (clears terminal and centers UI)
    #[arg(short, long)]
    pub fullscreen: bool,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Disable the status bar
    #[arg(short = 's', long)]
    pub no_status: bool,

    /// Disable the UI
    #[arg(short = 'u', long)]
    pub no_ui: bool,

    /// Enable dunst notifications with progress bar
    #[cfg(feature = "notify")]
    #[arg(short = 'N', long)]
    pub notify: bool,
}

fn parse_duration(arg: &str) -> Result<Duration, humantime::DurationError> {
    humantime::parse_duration(arg)
}

fn parse_color(arg: &str) -> Result<Color, String> {
    match arg.to_lowercase().as_str() {
        "black" => Ok(Color::Black),
        "dark_grey" | "darkgrey" => Ok(Color::DarkGrey),
        "red" => Ok(Color::Red),
        "dark_red" | "darkred" => Ok(Color::DarkRed),
        "green" => Ok(Color::Green),
        "dark_green" | "darkgreen" => Ok(Color::DarkGreen),
        "yellow" => Ok(Color::Yellow),
        "dark_yellow" | "darkyellow" => Ok(Color::DarkYellow),
        "blue" => Ok(Color::Blue),
        "dark_blue" | "darkblue" => Ok(Color::DarkBlue),
        "magenta" => Ok(Color::Magenta),
        "dark_magenta" | "darkmagenta" => Ok(Color::DarkMagenta),
        "cyan" => Ok(Color::Cyan),
        "dark_cyan" | "darkcyan" => Ok(Color::DarkCyan),
        "white" => Ok(Color::White),
        "grey" => Ok(Color::Grey),
        _ => Err(format!("Invalid color: {}", arg)),
    }
}
