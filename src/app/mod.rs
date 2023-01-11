#![allow(unused)]

mod event_handler;
mod events;
mod formatter;
mod ui;
pub mod file_manager;

use std::{io, thread, time::Duration};

use anyhow::{anyhow, Result};
use crossterm::{event, execute, terminal};
use event_handler::EventHandler;
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Widget},
    Terminal,
};
use ui::Ui;

use crate::database;
use crate::output;
use crate::params::Params;
use crate::scanner;

pub struct App;

impl App {
    pub fn init(app_args: &Params) -> Result<()> {
        // let mut term = Self::init_terminal()?;

        let connection = database::get_connection(app_args)?;
        let duplicates = scanner::duplicates(app_args, &connection)?;

        // Self::init_render_loop(&mut term)?;
        // Self::cleanup(&mut term)?;

        match app_args.interactive {
            true => output::interactive(duplicates, app_args),
            false => output::print(duplicates, app_args) /* TODO: APP TUI INIT FUNCTION */
        }
        
        Ok(())
    }

    fn cleanup(term: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        terminal::disable_raw_mode()?;
        execute!(
            term.backend_mut(),
            terminal::LeaveAlternateScreen,
            event::DisableMouseCapture
        )?;

        term.show_cursor()?;
        Ok(())
    }

    fn render_cycle(term: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        match EventHandler::init()? {
            events::Event::Noop => Ui::render_frame(term),
            events::Event::Exit => Err(anyhow!("Exit")),
        }
    }

    fn init_render_loop(term: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        // this could be simplified with a `while Self::render_cycle(term).is_ok() {}` in the current state, but maybe
        // it's good to keep it to handle errors in the future
        loop {
            match Self::render_cycle(term) {
                Ok(_) => continue,
                Err(_) => break,
            }
        }

        Ok(())
    }

    fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(
            stdout,
            terminal::EnterAlternateScreen,
            event::EnableMouseCapture
        )?;
        let backend = CrosstermBackend::new(stdout);
        Ok(Terminal::new(backend)?)
    }
}
