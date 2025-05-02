use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};

mod app;
use app::App;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app_result = App::default().run(&mut terminal); // Using default() is clean and sufficient

    ratatui::restore();
    app_result
}
