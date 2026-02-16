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
use std::error::Error;
use input::InputEvent;
use std::{io, time::Duration};
use timer::Timer;
use ui::Ui;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut timer = Timer::new(args.name, args.duration);
    let mut ui = Ui::new();

    terminal::enable_raw_mode()?;
    match (args.fullscreen, args.no_status, args.no_ui) {
        (true, _, false) => {
            io::stdout().execute(EnterAlternateScreen)?;
        }
        (false, true, false) => {
            println!();
            io::stdout().execute(cursor::MoveUp(1))?;
        }
        (false, false, false) => {
            println!();
            println!();
            io::stdout().execute(cursor::MoveUp(2))?;
        }
        (_, _, true) => {}
    }
    io::stdout().execute(cursor::Hide)?;

    let mut run_loop = || -> Result<(), Box<dyn Error>> {
        loop {
            match input::read_input(Duration::from_millis(50)) {
                InputEvent::Quit => break,
                InputEvent::TogglePause => timer.toggle_pause(),
                InputEvent::None => {}
            }

            if !args.no_ui {
                ui.draw(&timer, args.color, args.fullscreen, args.no_status)?;
            }

            if timer.is_finished() {
                break;
            }
        }
        Ok(())
    };

    let res = run_loop();

    // cleanup
    match (args.fullscreen, args.no_status, args.no_ui) {
        (true, _, false) => {
            io::stdout().execute(LeaveAlternateScreen)?;
        }
        (false, true, false) => {
            io::stdout().execute(cursor::MoveDown(1))?;
        }
        (false, false, false) => {
            io::stdout().execute(cursor::MoveDown(2))?;
        }
        (_, _, true) => {}
    }
    ui.conemu_reset_progress()?;
    io::stdout().execute(cursor::MoveToColumn(0))?;
    terminal::disable_raw_mode()?;

    if let Err(e) = res {
        eprintln!("Error: {}", e);
    } else if timer.is_finished() {
        if args.verbose {
            match &timer.name {
                Some(name) => eprintln!("Timer '{}' finished!", name),
                None => eprintln!("Timer finished!"),
            }
        }
    } else {
        if args.verbose {
            match &timer.name {
                Some(name) => eprintln!("Timer '{}' cancelled.", name),
                None => eprintln!("Timer cancelled."),
            }
        }
        std::process::exit(1);
    }
    Ok(())
}
