use crate::timer::Timer;
use crossterm::{
    QueueableCommand, cursor,
    style::{Color, Print, Stylize},
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Stdout, Write};

pub struct Ui {
    stdout: Stdout,
    last_size: Option<(u16, u16)>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            stdout: io::stdout(),
            last_size: None,
        }
    }

    pub fn draw(&mut self, timer: &Timer, color: Color, fullscreen: bool) -> io::Result<()> {
        // define constants
        let (cols, rows) = terminal::size()?;
        
        let mut full_clear = false;
        if self.last_size != Some((cols, rows)) {
            full_clear = true;
            self.last_size = Some((cols, rows));
        }

        let progress = timer.progress();
        let percent = (progress * 100.0) as u16;
        let percent_str = format!(" {:>3}%", percent);
        let pause_msg = if timer.is_paused { " PAUSED" } else { "" };
        let center_row = rows / 2;
        let remaining = timer.remaining_time();
        let hours = remaining.as_secs() / 3600;
        let mins = (remaining.as_secs() % 3600) / 60;
        let secs = remaining.as_secs() % 60;
        let start_time_str = timer.start_time.format("%I:%M%p").to_string();
        let mut remaining_str = String::new();
        if hours > 0 {
            remaining_str.push_str(&format!("{}h", hours));
        }
        if mins > 0 {
            remaining_str.push_str(&format!("{}m", mins));
        }
        remaining_str.push_str(&format!("{}s", secs));

        // bar width
        let bar_width = if fullscreen {
            (cols as usize).saturating_sub(percent_str.len() + 4)
        } else {
            (cols as usize).saturating_sub(percent_str.len() + 10).min(40)
        };
        let filled_width = (progress * bar_width as f64) as usize;
        let empty_width = bar_width.saturating_sub(filled_width);
        let total_bar_len = bar_width + percent_str.len();
        let bar_start_col = if fullscreen {
            (cols as usize).saturating_sub(total_bar_len) / 2
        } else {
            0
        };
        let filled_bar = "\u{2588}".repeat(filled_width);
        let empty_bar = "\u{2591}".repeat(empty_width);

        // drawing start position
        let (text_col, text_row) = if fullscreen {
            (bar_start_col as u16, center_row)
        } else {
            (0, 0)
        };


        // clear the drawing area
        if fullscreen {
            if full_clear {
                self.stdout.queue(Clear(ClearType::All))?;
            } else {
                self.stdout.queue(cursor::MoveTo(0, text_row))?;
                self.stdout.queue(Clear(ClearType::CurrentLine))?;
                self.stdout.queue(cursor::MoveTo(0, text_row + 1))?;
                self.stdout.queue(Clear(ClearType::CurrentLine))?;
            }
            self.stdout.queue(cursor::MoveTo(text_col, text_row))?;
        } else {
            self.stdout.queue(cursor::MoveToColumn(0))?;
            self.stdout.queue(Clear(ClearType::CurrentLine))?;
        }

        // print the start time
        self.stdout.queue(Print(start_time_str.bold()))?;

        // print the name of the timer
        if let Some(name) = &timer.name {
            self.stdout.queue(Print(": "))?;
            self.stdout.queue(Print(name.as_str().italic()))?;
        }

        // print the remaining time
        self.stdout.queue(Print(" - "))?;
        self.stdout.queue(Print(remaining_str.bold()))?;

        // if the timer is paused, print pause message
        if !pause_msg.is_empty() {
            self.stdout.queue(Print(pause_msg.with(Color::Yellow)))?;
        }

        // print the progress bar
        if fullscreen {
            self.stdout
                .queue(cursor::MoveTo(bar_start_col as u16, text_row + 1))?;
        } else {
            self.stdout.queue(cursor::MoveDown(1))?;
            self.stdout.queue(cursor::MoveToColumn(0))?;
            self.stdout.queue(Clear(ClearType::CurrentLine))?;
        }
        self.stdout
            .queue(Print(filled_bar.with(color)))?
            .queue(Print(empty_bar.with(Color::DarkGrey)))?
            .queue(Print(percent_str))?;

        // if not fullscreen, move the cursor up one line
        if !fullscreen {
            self.stdout.queue(cursor::MoveUp(1))?;
        }

        // flush to stdout
        self.stdout.flush()?;
        Ok(())
    }
}
