use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use std::time::Duration;

pub enum InputEvent {
    None,
    Quit,
    TogglePause,
}

pub fn read_input(timeout: Duration) -> InputEvent {
    if event::poll(timeout).unwrap_or(false) {
        if let Ok(Event::Key(key)) = event::read() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return InputEvent::Quit,
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return InputEvent::Quit;
                    }
                    KeyCode::Char('p') => return InputEvent::TogglePause,
                    KeyCode::Char(' ') => return InputEvent::TogglePause,
                    _ => {}
                }
            }
        }
    }
    InputEvent::None
}
