use std::thread::spawn;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListDirection, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen};
use crate::game::CellDisplay;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn ui(frame: &mut Frame, app: &mut App) {
    // 上下三分割
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Header/info
            Constraint::Min(1),    // Main content
            Constraint::Length(3), // Footer/helper
        ])
        .split(frame.area());

    // Header/info
    draw_info(frame, app, chunks[0]);

    // Footer/helper
    draw_helper(frame, app, chunks[2]);

    let middle = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 5), Constraint::Min(1)])
        .split(chunks[1]);

    draw_archive_sidebar(frame, app, middle[0]);

    match app.current_screen {
        CurrentScreen::Menu => draw_menu_content(frame, app, middle[1]),
        CurrentScreen::Archive => draw_archive_content(frame, app, middle[1]),
        CurrentScreen::Game => draw_game_content(frame, app, middle[1]),
        CurrentScreen::Settings => draw_settings_content(frame, app, middle[1]),
        CurrentScreen::Help => draw_help_content(frame, app, middle[1]),
        CurrentScreen::Exiting => draw_exiting_content(frame, app, middle[1]),
        CurrentScreen::Win => draw_win(frame, app, middle[1]),
    }
}

// Header/info
fn draw_info(frame: &mut Frame, app: &App, area: Rect) {
    let info_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(2, 8),
            Constraint::Ratio(4, 8),
            Constraint::Ratio(2, 8),
        ])
        .split(area);
    let left_info = match app.current_screen {
        CurrentScreen::Game => {
            if let Some(game) = &app.game {
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
            }
        }
        CurrentScreen::Archive => {
            if let Some(selected) = app.archive_list.selected() {
                format!("Selected puzzle: {:03}", selected + 1)
            } else {
                "Select a puzzle".to_string()
            }
        }
        _ => "Akari Game".to_string(),
    };
    let left_para =
        Paragraph::new(left_info).block(Block::default().borders(Borders::ALL).title("Info"));
    frame.render_widget(left_para, info_chunks[0]);

    let ascii_lines = vec![
        Line::from(Span::styled(
            r#"/\  __ \   /\ \/ /    /\  __ \   /\  == \   /\__  _\ /\ \/\ \   /\ \"#,
            Style::default().fg(Color::LightRed),
        )),
        Line::from(Span::styled(
            r#" \ \  __ \  \ \  _"-.  \ \  __ \  \ \  __<   \/_/\ \/ \ \ \_\ \  \ \ \"#,
            Style::default().fg(Color::LightYellow),
        )),
        Line::from(Span::styled(
            r#"   \ \_\ \_\  \ \_\ \_\  \ \_\ \_\  \ \_\ \_\    \ \_\  \ \_____\  \ \_\"#,
            Style::default().fg(Color::LightGreen),
        )),
        Line::from(Span::styled(
            r#"    \/_/\/_/   \/_/\/_/   \/_/\/_/   \/_/ /_/     \/_/   \/_____/   \/_/"#,
            Style::default().fg(Color::LightBlue),
        )),
    ];

    let center_para = Paragraph::new(ascii_lines)
        .block(Block::default().borders(Borders::TOP))
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(center_para, info_chunks[1]);

    // 右

    let right_info = format!(
        "Time: {}\nStatus: {}",
        app.timer_string(),
        match app.current_screen {
            CurrentScreen::Game => "Playing",
            CurrentScreen::Archive => "Browsing",
            CurrentScreen::Win => "Finished",
            _ => "",
        }
    );
    let right_para =
        Paragraph::new(right_info).block(Block::default().borders(Borders::ALL).title("Status"));
    frame.render_widget(right_para, info_chunks[2]);
}

// Footer/helper
fn draw_helper(frame: &mut Frame, app: &App, area: Rect) {
    let text = match app.current_screen {
        CurrentScreen::Game => "<Arrow Keys>: Move  <Space>: Lightbulb  <F>: Flag  <Q>: Back",
        CurrentScreen::Archive => "<Arrow Keys>: Move  <Enter>: Start Game  <Q>: Back",
        CurrentScreen::Menu => "<Arrow Keys>: Menu  <Enter>: Select  <Q>: Quit",
        CurrentScreen::Settings => "Settings Screen  <Q>: Back",
        CurrentScreen::Help => "<Q>: Back",
        CurrentScreen::Exiting => "<Enter>: Confirm Exit  <Q>: Cancel",
        CurrentScreen::Win => "<Q>: Back",
    };
    let para = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Helper"));
    frame.render_widget(para, area);
}

// 左側 archive
fn draw_archive_sidebar(frame: &mut Frame, app: &mut App, area: Rect) {
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

    frame.render_stateful_widget(archive_list, area, &mut app.archive_list);
}

fn draw_menu_content(frame: &mut Frame, app: &mut App, area: Rect) {
    let menu_items = vec![
        ("🟢 <G> New Game ", "Start a random puzzle", Color::Green),
        ("A <A> Archive", "Browse all puzzles", Color::Cyan),
        (
            "⚙️ <S> Settings",
            "Configure your experience",
            Color::LightMagenta,
        ),
        ("❓ <H> Help", "How to play Akari", Color::LightBlue),
        ("🚪 <E> Exit", "Leave the game", Color::Red),
    ];

    let items: Vec<ListItem> = menu_items
        .iter()
        .map(|(title, desc, color)| {
            ListItem::new(vec![
                Line::from(Span::styled(
                    *title,
                    Style::default().fg(*color).add_modifier(Modifier::BOLD),
                )),
                Line::from(Span::styled(
                    *desc,
                    Style::default()
                        .fg(Color::Gray)
                        .add_modifier(Modifier::ITALIC),
                )),
                Line::from(""), // 空行分隔
            ])
        })
        .collect();

    let menu = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    "^v^ Akari Menu ^v^",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ))
                .title_alignment(ratatui::layout::Alignment::Center),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::REVERSED),
        )
        .highlight_symbol("--> ")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(menu, area, &mut app.menu_list);
}

fn draw_archive_content(frame: &mut Frame, app: &mut App, area: Rect) {
    let mut lines = vec![];

    if let Some(selected) = app.archive_list.selected() {
        let puzzle_id = selected + 1;
        lines.push(Line::from(vec![Span::styled(
            format!("📄 Puzzle {:03}", puzzle_id),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]));
        lines.push(Line::from(""));

        // 顯示 metadata
        if let Some(game) = &app.game {
            if let Some(puzzle) = &game.puzzle {
                let meta = &puzzle.metadata;
                lines.push(Line::from(vec![
                    Span::styled("Type: ", Style::default().fg(Color::Cyan)),
                    Span::raw(&meta.puzzle_type),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Author: ", Style::default().fg(Color::Cyan)),
                    Span::raw(&meta.author),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Size: ", Style::default().fg(Color::Cyan)),
                    Span::raw(format!("{} x {}", meta.size.rows, meta.size.cols)),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Source: ", Style::default().fg(Color::Cyan)),
                    Span::raw(&meta.source),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Info: ", Style::default().fg(Color::Cyan)),
                    Span::raw(&meta.info),
                ]));

                lines.push(Line::from("")); // 空行
                lines.push(Line::from(Span::styled(
                    "🧩 Mini Board Preview",
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )));
                // 印出棋盤
                for row in &puzzle.problem {
                    let row_str: String = row
                        .iter()
                        .map(|cell| match cell.as_str() {
                            "x" => "█", // 牆
                            "0" => "0",
                            "1" => "1",
                            "2" => "2",
                            "3" => "3",
                            "4" => "4",
                            _ => "·", // 空格
                        })
                        .collect();
                    lines.push(Line::from(row_str));
                }
            } else {
                lines.push(Line::from(Span::styled(
                    "No metadata loaded.",
                    Style::default().fg(Color::Red),
                )));
            }
        } else {
            lines.push(Line::from(Span::styled(
                "No puzzle loaded.",
                Style::default().fg(Color::Red),
            )));
        }
    } else {
        lines.push(Line::from(Span::styled(
            "Please select a puzzle from the left.",
            Style::default().fg(Color::Gray),
        )));
    }

    let para = Paragraph::new(lines)
        .block(
            Block::default().borders(Borders::ALL).title(Span::styled(
                "Preview",
                Style::default()
                    .fg(Color::LightCyan)
                    .add_modifier(Modifier::BOLD),
            )),
        )
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(para, area);
}

// 右側內容：遊戲
fn draw_game_content(frame: &mut Frame, app: &mut App, area: Rect) {
    if let Some(game) = &app.game {
        let display = game.get_display();
        let rows = display.len();
        let cols = display[0].len();

        let cell_width = area.width / cols as u16;
        let cell_height = area.height / rows as u16;

        let board_width = cell_width * cols as u16;
        let board_height = cell_height * rows as u16;

        let pad_x = (area.width - board_width) / 2;
        let pad_y = (area.height - board_height) / 2;

        for i in 0..rows {
            for j in 0..cols {
                let x = area.x + pad_x + j as u16 * cell_width;
                let y = area.y + pad_y + i as u16 * cell_height;
                let cell_area = Rect::new(x, y, cell_width, cell_height);

                let (title, style): (String, Style) = match display[i][j] {
                    CellDisplay::Wall => ("██".to_string(), Style::default().fg(Color::DarkGray)),
                    CellDisplay::Target(n) => {
                        let color = if n == 0 { Color::Green } else { Color::White };
                        (format!("{}", n), Style::default().fg(color))
                    }
                    CellDisplay::LightBulb => {
                        ("💡".to_string(), Style::default().fg(Color::LightYellow))
                    }
                    CellDisplay::Light(n) => (format!("{}", n), Style::default().fg(Color::Yellow)),
                    CellDisplay::Flag => ("P".to_string(), Style::default().fg(Color::Red)),
                    CellDisplay::Dark => ("".to_string(), Style::default().fg(Color::Black)),
                };

                let mut cell_style = style;
                if (i, j) == game.cursor_position {
                    cell_style = cell_style.bg(Color::Blue);
                }

                let para = Paragraph::new("").style(cell_style).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(title)
                        .title_alignment(ratatui::layout::Alignment::Center),
                );
                frame.render_widget(para, cell_area);
            }
        }
    }
}

fn draw_settings_content(frame: &mut Frame, _app: &mut App, area: Rect) {
    let para = Paragraph::new("Settings screen (not implemented yet)")
        .block(Block::default().borders(Borders::ALL).title("Settings"));
    frame.render_widget(para, area);
}

fn draw_help_content(frame: &mut Frame, _app: &mut App, area: Rect) {
    let lines = vec![
        Line::from(Span::styled(
            "🕯️  Akari (Light Up) Game Rules",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("• The board consists of white and black cells."),
        Line::from("• Place light bulbs in white cells so that every white cell is lit."),
        Line::from(
            "• Each bulb illuminates its entire row and column until blocked by a black cell.",
        ),
        Line::from(
            "• No two bulbs may shine on each other (no two bulbs in the same line of sight).",
        ),
        Line::from(
            "• A black cell may have a number (0-4), indicating how many bulbs must be placed",
        ),
        Line::from("  adjacent to its four sides (up, down, left, right)."),
        Line::from("• A black cell with 0 must not have any bulbs adjacent to it."),
        Line::from(
            "• An unnumbered black cell may have any number of bulbs adjacent to it, or none.",
        ),
        Line::from(
            "• Bulbs diagonally adjacent to a numbered cell do not count toward its requirement.",
        ),
        Line::from(""),
        Line::from(Span::styled(
            "🎮 Controls",
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("G - Start a random new game"),
        Line::from("A - Open archive"),
        Line::from("S - Settings"),
        Line::from("H - Show this help"),
        Line::from("E - Exit game"),
        Line::from("Q - Back/quit current screen"),
        Line::from(""),
        Line::from("Arrow keys - Move cursor"),
        Line::from("Space - Place/remove lightbulb"),
        Line::from("F - Place/remove flag"),
        Line::from(""),
        Line::from(Span::styled(
            "Have fun and good luck!",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::ITALIC),
        )),
    ];

    let para = Paragraph::new(lines)
        .block(
            Block::default().borders(Borders::ALL).title(Span::styled(
                "Help",
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(para, area);
}

fn draw_exiting_content(frame: &mut Frame, _app: &mut App, area: Rect) {
    let exiting_text = "Do you want to exit?\n\nPress Enter to exit\nPress Q to return to menu";
    let para = Paragraph::new(exiting_text)
        .block(Block::default().borders(Borders::ALL).title("Exiting"))
        .wrap(Wrap { trim: true });
    frame.render_widget(para, area);
}

fn draw_win(frame: &mut Frame, _app: &mut App, area: Rect) {
    let text = "Congratulations! You Win!\n\nPress Q to return to menu";
    let para = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("You Win!"))
        .wrap(Wrap { trim: true });
    frame.render_widget(para, area);
}
