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
        .filter_level(if args.verbose {
            log::LevelFilter::Warn
        } else {
            log::LevelFilter::Error
        })
        .format_timestamp(None)
        .init();

    let mut timer = Timer::new(args.name.clone(), args.duration);
    let mut ui = Ui::new();

    terminal::enable_raw_mode()?;
    if args.fullscreen {
        io::stdout().execute(EnterAlternateScreen)?;
        io::stdout().execute(cursor::Hide)?;
    } else {
        println!("\n");
        io::stdout().execute(cursor::MoveUp(2))?;
        io::stdout().execute(cursor::Hide)?;
    }

    let mut run_loop = || -> anyhow::Result<()> {
        loop {
            match input::read_input(Duration::from_millis(50)) {
                InputEvent::Quit => break,
                InputEvent::TogglePause => timer.toggle_pause(),
                InputEvent::None => {}
            }

            ui.draw(&timer, args.color, args.fullscreen)?;

            if timer.is_finished() {
                break;
            }
        }
        Ok(())
    };

    let res = run_loop();

    // cleanup
    if args.fullscreen {
        io::stdout().execute(cursor::Show)?;
        io::stdout().execute(LeaveAlternateScreen)?;
    } else {
        io::stdout().execute(cursor::MoveDown(2))?;
        io::stdout().execute(cursor::MoveToColumn(0))?;
    }
    terminal::disable_raw_mode()?;

    if let Err(e) = res {
        eprintln!("Error: {}", e);
    } else if timer.is_finished() {
        if let Some(name) = &timer.name {
            log::warn!("Timer '{}' finished!", name);
        } else {
            log::warn!("Timer finished!");
        }
    } else {
        if let Some(name) = &timer.name {
            log::warn!("Timer '{}' cancelled.", name);
        } else {
            log::warn!("Timer cancelled.");
        }
        std::process::exit(1);
    }

    Ok(())
}
