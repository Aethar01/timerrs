use crate::timer::Timer;
use crossterm::{
    cursor,
    style::{Color, Print, Stylize},
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};
use std::io::{self, Stdout, Write};
use std::time::Duration;

struct DrawBarArgs<'a> {
    progress: f64,
    color: Color,
    fullscreen: bool,
    no_status: bool,
    full_clear: bool,
    center_row: u16,
    text_row: u16,
    bar_start_col: usize,
    bar_width: usize,
    percent_str: &'a str,
}

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

    pub fn conemu_reset_progress(&mut self) -> io::Result<()> {
        self.set_conemu_progress(0, 0)
    }

    pub fn draw(
        &mut self,
        timer: &Timer,
        color: Color,
        fullscreen: bool,
        no_status: bool,
    ) -> io::Result<()> {
        let (cols, rows) = terminal::size()?;
        let full_clear = self.check_resize(cols, rows);

        let progress = timer.progress();
        let percent_str = format!(" {:>3}%", (progress * 100.0) as u16);

        let (bar_width, bar_start_col) = self.get_bar_size(cols, fullscreen, percent_str.len());

        let center_row = rows / 2;
        let (text_col, text_row) = match fullscreen {
            true => (bar_start_col as u16, center_row),
            false => (0, 0),
        };

        match no_status {
            false => self.draw_status(timer, fullscreen, full_clear, text_col, text_row)?,
            true if fullscreen && full_clear => {
                self.stdout.queue(Clear(ClearType::All))?;
            }
            _ => {}
        }

        self.draw_bar(&DrawBarArgs {
            progress,
            color,
            fullscreen,
            no_status,
            full_clear,
            center_row,
            text_row,
            bar_start_col,
            bar_width,
            percent_str: &percent_str,
        })?;

        let conemu_state = if timer.is_paused { 4 } else { 1 };
        self.set_conemu_progress(conemu_state, (progress * 100.0) as u16)?;

        self.stdout.flush()?;
        Ok(())
    }

    fn set_conemu_progress(&mut self, state: u16, progress: u16) -> io::Result<()> {
        write!(self.stdout, "\x1b]9;4;{};{}\x07", state, progress)?;
        Ok(())
    }

    fn check_resize(&mut self, cols: u16, rows: u16) -> bool {
        match self.last_size == Some((cols, rows)) {
            true => false,
            false => {
                self.last_size = Some((cols, rows));
                true
            }
        }
    }

    fn get_bar_size(&self, cols: u16, fullscreen: bool, percent_len: usize) -> (usize, usize) {
        let bar_width = match fullscreen {
            true => (cols as usize).saturating_sub(percent_len + 4),
            false => (cols as usize).saturating_sub(percent_len + 10).min(40),
        };

        let total_bar_len = bar_width + percent_len;
        let bar_start_col = match fullscreen {
            true => (cols as usize).saturating_sub(total_bar_len) / 2,
            false => 0,
        };

        (bar_width, bar_start_col)
    }

    fn format_duration(&self, duration: Duration) -> String {
        let secs = duration.as_secs();
        let hours = secs / 3600;
        let mins = (secs % 3600) / 60;
        let secs = secs % 60;

        let mut s = String::new();
        if hours > 0 {
            s.push_str(&format!("{}h", hours));
        }
        if mins > 0 {
            s.push_str(&format!("{}m", mins));
        }
        s.push_str(&format!("{}s", secs));
        s
    }

    fn draw_status(
        &mut self,
        timer: &Timer,
        fullscreen: bool,
        full_clear: bool,
        col: u16,
        row: u16,
    ) -> io::Result<()> {
        match (fullscreen, full_clear) {
            (true, true) => {
                self.stdout
                    .queue(Clear(ClearType::All))?
                    .queue(cursor::MoveTo(col, row))?;
            }
            (true, false) => {
                self.stdout
                    .queue(cursor::MoveTo(0, row))?
                    .queue(Clear(ClearType::CurrentLine))?
                    .queue(cursor::MoveTo(0, row + 1))?
                    .queue(Clear(ClearType::CurrentLine))?
                    .queue(cursor::MoveTo(col, row))?;
            }
            _ => {
                self.stdout
                    .queue(cursor::MoveToColumn(0))?
                    .queue(Clear(ClearType::CurrentLine))?;
            }
        };

        let start_time = timer.start_time.format("%I:%M%p").to_string();
        self.stdout.queue(Print(start_time.bold()))?;

        if let Some(name) = &timer.name {
            self.stdout
                .queue(Print(": "))?
                .queue(Print(name.as_str().italic()))?;
        }

        let remaining = self.format_duration(timer.remaining_time());
        self.stdout
            .queue(Print(" - "))?
            .queue(Print(remaining.bold()))?;

        if timer.is_paused {
            self.stdout.queue(Print(" PAUSED".with(Color::Yellow)))?;
        }

        Ok(())
    }

    fn draw_bar(&mut self, args: &DrawBarArgs) -> io::Result<()> {
        let filled_width = (args.progress * args.bar_width as f64) as usize;
        let empty_width = args.bar_width.saturating_sub(filled_width);

        let filled_bar = "█".repeat(filled_width);
        let empty_bar = "░".repeat(empty_width);

        match args.fullscreen {
            true => {
                let bar_row = match args.no_status {
                    true => args.center_row,
                    false => args.text_row + 1,
                };
                self.stdout
                    .queue(cursor::MoveTo(args.bar_start_col as u16, bar_row))?;
                if args.no_status && !args.full_clear {
                    self.stdout.queue(Clear(ClearType::CurrentLine))?;
                }
            }
            false => {
                if !args.no_status {
                    self.stdout.queue(cursor::MoveDown(1))?;
                }
                self.stdout
                    .queue(cursor::MoveToColumn(0))?
                    .queue(Clear(ClearType::CurrentLine))?;
            }
        }

        self.stdout
            .queue(Print(filled_bar.with(args.color)))?
            .queue(Print(empty_bar.with(Color::DarkGrey)))?
            .queue(Print(args.percent_str))?;

        if !args.fullscreen && !args.no_status {
            self.stdout.queue(cursor::MoveUp(1))?;
        }

        Ok(())
    }
}
