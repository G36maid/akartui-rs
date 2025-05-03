use std::slice::Chunks;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListDirection, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen};
use crate::game::{CellDisplay, Game, Puzzle, PuzzleMetadata, PuzzleSize};

pub fn ui(frame: &mut Frame, app: &mut App) {
    match app.current_screen {
        CurrentScreen::Menu => draw_menu(frame, app),
        CurrentScreen::Game => draw_game(frame, app),
        CurrentScreen::Archive => draw_archive(frame, app),
        CurrentScreen::Help => draw_help(frame, app),
        CurrentScreen::Settings => draw_settings(frame, app),
        CurrentScreen::Exiting => draw_exiting(frame, app),
        _ => {}
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

    let list = vec!["New Game", "Archive", "Settings", "Help", "Exit"];

    let menu = List::new(list)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(menu, chunks[1], &mut app.menu_list);

    // Footer
    let footer = Block::default()
        .borders(Borders::ALL)
        .title("Press q to quit");
    frame.render_widget(footer, chunks[2]);
}

fn draw_archive(frame: &mut Frame, app: &mut App) {
    //clear first
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

    // Footer
    let footer = Block::default()
        .borders(Borders::ALL)
        .title("Press q to quit");
    frame.render_widget(footer, chunks[2]);

    let middle = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(3, 10), // Header
            Constraint::Min(0),
        ])
        .split(chunks[1]);

    let archive_items: Vec<String> = (1..=750).map(|i| format!("Puzzle {:03}", i)).collect();
    let archive_list = List::new(
        archive_items
            .iter()
            .map(|s| ListItem::new(s.clone()))
            .collect::<Vec<_>>(),
    )
    .block(Block::default().borders(Borders::ALL).title("Archive"))
    .style(Style::default().fg(Color::White))
    .highlight_style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol(">> ")
    .repeat_highlight_symbol(true)
    .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(archive_list, middle[0], &mut app.archive_list);

    let list = vec!["New Game", "Archive", "Settings", "Help", "Exit"];

    let menu = List::new(list)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(menu, middle[1], &mut app.menu_list);
}

fn draw_game(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // 調高一點
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.size());

    // 取得 metadata
    let meta_text = if let Some(game) = &app.game {
        if let Some(puzzle) = &game.puzzle {
            let meta = &puzzle.metadata;
            format!(
                "Puzzle ID: {}\nType: {}\nAuthor: {}\nSize: {}x{}\nSource: {}\nInfo: {}",
                puzzle.id,
                meta.puzzle_type,
                meta.author,
                meta.size.rows,
                meta.size.cols,
                meta.source,
                meta.info
            )
        } else {
            "No puzzle loaded".to_string()
        }
    } else {
        "No game".to_string()
    };

    let metadata =
        Paragraph::new(meta_text).block(Block::default().borders(Borders::ALL).title("Game Info"));
    frame.render_widget(metadata, chunks[0]);

    let controls_hint = Paragraph::new("Controls: Arrow keys to move, Space to add lightbulb.")
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    frame.render_widget(controls_hint, chunks[2]);

    if let Some(game) = &app.game {
        let display = game.get_display();
        let rows = display.len();
        let cols = display[0].len();

        let row_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Ratio(1, rows as u32); rows])
            .split(chunks[1]);

        for (i, row_area) in row_areas.iter().enumerate() {
            let col_areas = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Ratio(1, cols as u32); cols])
                .split(*row_area);

            for (j, cell_area) in col_areas.iter().enumerate() {
                let (symbol, style): (String, Style) = match display[i][j] {
                    CellDisplay::Wall => ("██".to_string(), Style::default().fg(Color::DarkGray)),
                    CellDisplay::Target(n) => {
                        (format!("{}", n), Style::default().fg(Color::Yellow))
                    }
                    CellDisplay::LightBulb => {
                        ("💡".to_string(), Style::default().fg(Color::Yellow))
                    }
                    CellDisplay::Light(_) => ("·".to_string(), Style::default().fg(Color::White)),
                    CellDisplay::Flag => ("🚩".to_string(), Style::default().fg(Color::Red)),
                    CellDisplay::Dark => (" ".to_string(), Style::default()),
                };

                // 高亮游標
                let mut cell_style = style;
                if (i, j) == game.cursor_position {
                    cell_style = cell_style.bg(Color::Blue);
                }

                let para = Paragraph::new(symbol)
                    .style(cell_style)
                    .block(Block::default().borders(Borders::ALL));
                frame.render_widget(para, *cell_area);
            }
        }
    }
}

fn draw_settings(frame: &mut Frame, app: &mut App) {
    draw_menu(frame, app);
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
    //clear first
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

    // Footer
    let footer = Block::default()
        .borders(Borders::ALL)
        .title("Press q to quit");
    frame.render_widget(footer, chunks[2]);

    let middle = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(3, 10), // Header
            Constraint::Min(0),
        ])
        .split(chunks[1]);

    let archive_items: Vec<String> = (1..=750).map(|i| format!("Puzzle {:03}", i)).collect();
    let archive_list = List::new(
        archive_items
            .iter()
            .map(|s| ListItem::new(s.clone()))
            .collect::<Vec<_>>(),
    )
    .block(Block::default().borders(Borders::ALL).title("Archive"))
    .style(Style::default().fg(Color::White))
    .highlight_style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol(">> ")
    .repeat_highlight_symbol(true)
    .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(archive_list, middle[0], &mut app.archive_list);

    let list = vec!["New Game", "Archive", "Settings", "Help", "Exit"];

    let menu = List::new(list)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(menu, middle[1], &mut app.menu_list);

    let exiting_text = "Do you want to exit?\n\nPress Enter to exit\nPress Q to return to menu";
    let exiting_paragraph = Paragraph::new(exiting_text)
        .block(Block::default().borders(Borders::ALL).title("Exiting"))
        .wrap(Wrap { trim: true });
}
