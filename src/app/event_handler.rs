use crossterm::event::{self, KeyCode, KeyEvent};
use anyhow::Result;
use super::events;

pub struct EventHandler;

impl EventHandler {
    pub fn init() -> Result<events::Event> {
        match event::read()? {
            event::Event::Key(keycode) => Self::handle_keypress(keycode),
            _ => Ok(events::Event::Noop),
        }
    }

    fn handle_keypress(keyevent: KeyEvent) -> Result<events::Event> {
        match keyevent.code {
            KeyCode::Char('q') => Ok(events::Event::Exit),
            _ => Ok(events::Event::Noop)
        }
    }
}

