mod args;
mod input;
mod timer;
mod ui;

use args::Args;
use clap::Parser;
use crossterm::{
    ExecutableCommand, cursor,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use input::InputEvent;
use std::{io, time::Duration};
use timer::Timer;
use ui::Ui;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    env_logger::Builder::new()
        .filter_level(match args.verbose {
            true => log::LevelFilter::Warn,
            false => log::LevelFilter::Error,
        })
        .format_module_path(false)
        .init();

    let mut timer = Timer::new(args.name, args.duration);
    let mut ui = Ui::new();

    terminal::enable_raw_mode()?;
    match (args.fullscreen, args.no_status) {
        (true, _) => {
            io::stdout().execute(EnterAlternateScreen)?;
        }
        (false, true) => {
            println!();
            io::stdout().execute(cursor::MoveUp(1))?;
        }
        (false, false) => {
            println!();
            println!();
            io::stdout().execute(cursor::MoveUp(2))?;
        }
    }
    io::stdout().execute(cursor::Hide)?;

    let mut run_loop = || -> anyhow::Result<()> {
        loop {
            match input::read_input(Duration::from_millis(50)) {
                InputEvent::Quit => break,
                InputEvent::TogglePause => timer.toggle_pause(),
                InputEvent::None => {}
            }

            ui.draw(&timer, args.color, args.fullscreen, args.no_status)?;

            if timer.is_finished() {
                break;
            }
        }
        Ok(())
    };

    let res = run_loop();

    // cleanup
    match (args.fullscreen, args.no_status) {
        (true, _) => {
            io::stdout().execute(LeaveAlternateScreen)?;
        }
        (false, true) => {
            io::stdout().execute(cursor::MoveDown(1))?;
        }
        (false, false) => {
            io::stdout().execute(cursor::MoveDown(2))?;
        }
    }
    io::stdout().execute(cursor::MoveToColumn(0))?;
    terminal::disable_raw_mode()?;

    if let Err(e) = res {
        eprintln!("Error: {}", e);
    } else if timer.is_finished() {
        match &timer.name {
            Some(name) => log::warn!("Timer '{}' finished!", name),
            None => log::warn!("Timer finished!"),
        }
    } else {
        match &timer.name {
            Some(name) => log::warn!("Timer '{}' cancelled.", name),
            None => log::warn!("Timer cancelled."),
        }
        std::process::exit(1);
    }
    Ok(())
}
