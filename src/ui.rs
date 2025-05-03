use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListDirection, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen};
//use crate::game::Puzzle;

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

    // let menus = Vec<ListItem> = app.menu_list.iter().map(|item| {
    //     ListItem::new(Paragraph::new(item).style(Style::default().fg(Color::White)))
    // }).collect();

    let menu = List::new(app.menu_list)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::BottomToTop);

    frame.render_widget(menu, chunks[1]);

    // Footer
    let footer = Block::default()
        .borders(Borders::ALL)
        .title("Press q to quit");
    frame.render_widget(footer, chunks[2]);
}

fn draw_game(frame: &mut Frame, app: &mut App) {
    if let Some(game) = &app.game {
        if let Some(puzzle) = &game.puzzle {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Game info like metadata
                    Constraint::Min(1),    // Game board (for the puzzle)
                    Constraint::Length(3), // Controls hint
                ])
                .split(frame.size());

            // Game info (top section)
            let metadata = Paragraph::new("Game info")
                .block(Block::default().borders(Borders::ALL).title("Game Info"));

            frame.render_widget(metadata, chunks[0]);

            todo!();

            // Controls hint (bottom section)
            let controls_hint =
                Paragraph::new("Controls: Arrow keys to move, Space to add lightbulb.")
                    .block(Block::default().borders(Borders::ALL).title("Controls"));

            frame.render_widget(controls_hint, chunks[2]);
        }
    }
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
