use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &mut App) {
    match app.current_screen {
        CurrentScreen::Menu => draw_menu(frame, app),
        CurrentScreen::Game => draw_game(frame, app),
        CurrentScreen::Archive => draw_archive(frame, app),
        CurrentScreen::Help => draw_help(frame, app),
        CurrentScreen::Settings => draw_settings(frame, app),
        CurrentScreen::Exiting => {}
    }
}

fn draw_menu(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(1),    // Content
            Constraint::Length(3), // Footer
        ])
        .split(frame.size());

    // Header
    let header = Paragraph::new("Test text in header").block(
        Block::default()
            .borders(Borders::ALL)
            .title_alignment(ratatui::layout::Alignment::Center)
            .title("Akari Game"),
    );

    frame.render_widget(header, chunks[0]);

    // Menu items in the middle
    let menu_items = vec![
        ListItem::new("G = new Game"),
        ListItem::new("A = Archive"),
        ListItem::new("S = Settings"),
        ListItem::new("H = Help"),
        ListItem::new("E = Exit"),
    ];

    let menu = List::new(menu_items)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow));

    frame.render_widget(menu, chunks[1]);

    // Footer
    let footer = Block::default()
        .borders(Borders::ALL)
        .title("Press q to quit");
    frame.render_widget(footer, chunks[2]);
}

fn draw_game(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Game info
            Constraint::Min(1),    // Game board
            Constraint::Length(3), // Controls
        ])
        .split(frame.size());

    // Implement game UI
}

fn draw_archive(frame: &mut Frame, app: &mut App) {
    todo!()
    // Implement archive UI
}

fn draw_settings(frame: &mut Frame, app: &mut App) {
    todo!()
    // Implement settings UI
}

fn draw_help(frame: &mut Frame, _app: &mut App) {
    let help_text = "Help for Akari Game\n\nControls:\nG - Start New Game\nA - Open Archive\nS - Open Settings\nH - Show This Help\nE - Exit Game\nQ - Quit Current Screen";

    let help_paragraph = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .wrap(Wrap { trim: true });

    frame.render_widget(help_paragraph, frame.size());
}

fn draw_exiting(frame: &mut Frame, app: &mut App) {
    todo!()
    // Implement exiting UI
}
