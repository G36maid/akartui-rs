use std::io;

//use crossterm::terminal;
use ratatui::{DefaultTerminal, Frame};

#[derive(Debug)]
pub enum CurrentScreen {
    Menu,
    Game,
    Archive,
    Settings,
    Exiting,
}

#[derive(Debug, Default)]
pub struct App {
    pub current_screen: CurrentScreen,
    exit: bool,
}

impl Default for CurrentScreen {
    fn default() -> Self {
        CurrentScreen::Menu // Specify which variant is default
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_event()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        todo!()
    }

    fn handle_event(&mut self) -> io::Result<()> {
        todo!()
    }
}
