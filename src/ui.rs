use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListDirection, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen};
use crate::game::CellDisplay;

pub fn ui(frame: &mut Frame, app: &mut App) {
    // 上下三分割
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Header/info
            Constraint::Min(1),    // Main content
            Constraint::Length(3), // Footer/helper
        ])
        .split(frame.size());

    // Header/info
    draw_info(frame, app, chunks[0]);

    // Footer/helper
    draw_helper(frame, app, chunks[2]);

    // 中間左右分割
    let middle = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(24), // 左側 archive 固定寬度
            Constraint::Min(1),     // 右側內容
        ])
        .split(chunks[1]);

    // 左側 archive
    draw_archive_sidebar(frame, app, middle[0]);

    // 右側內容
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
    let info = match app.current_screen {
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
                format!("選擇題目：{:03}", selected + 1)
            } else {
                "選擇題目".to_string()
            }
        }
        _ => "Akari Game".to_string(),
    };

    let para = Paragraph::new(info).block(Block::default().borders(Borders::ALL).title("Info"));
    frame.render_widget(para, area);
}

// Footer/helper
fn draw_helper(frame: &mut Frame, app: &App, area: Rect) {
    let text = match app.current_screen {
        CurrentScreen::Game => "↑↓←→:移動  Space:燈泡  F:旗子  Q:返回",
        CurrentScreen::Archive => "↑↓:移動  Enter:開始遊戲  Q:返回",
        CurrentScreen::Menu => "↑↓:選單  Enter:選擇  Q:離開",
        CurrentScreen::Settings => "設定畫面  Q:返回",
        CurrentScreen::Help => "Q:返回",
        CurrentScreen::Exiting => "Enter:確認離開  Q:取消",
        CurrentScreen::Win => "Q:返回",
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

// 右側內容：主選單
fn draw_menu_content(frame: &mut Frame, app: &mut App, area: Rect) {
    let list = vec!["New Game", "Archive", "Settings", "Help", "Exit"];
    let menu = List::new(list)
        .block(Block::default().borders(Borders::ALL).title("Menu"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(menu, area, &mut app.menu_list);
}

// 右側內容：Archive 預覽
fn draw_archive_content(frame: &mut Frame, app: &mut App, area: Rect) {
    let text = if let Some(selected) = app.archive_list.selected() {
        format!(
            "Puzzle {:03} 預覽/資訊\n(可在這裡顯示 metadata 或小棋盤)",
            selected + 1
        )
    } else {
        "請選擇題目".to_string()
    };
    let para = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Preview"));
    frame.render_widget(para, area);
}

// 右側內容：遊戲
fn draw_game_content(frame: &mut Frame, app: &mut App, area: Rect) {
    if let Some(game) = &app.game {
        let display = game.get_display();
        let rows = display.len();
        let cols = display[0].len();

        let row_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Ratio(1, rows as u32); rows])
            .split(area);

        for (i, row_area) in row_areas.iter().enumerate() {
            let col_areas = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Ratio(1, cols as u32); cols])
                .split(*row_area);

            for (j, cell_area) in col_areas.iter().enumerate() {
                let (symbol, style): (String, Style) = match display[i][j] {
                    CellDisplay::Wall => ("██".to_string(), Style::default().fg(Color::DarkGray)),
                    CellDisplay::Target(n) => (format!("{}", n), Style::default().fg(Color::White)),
                    CellDisplay::LightBulb => {
                        ("💡".to_string(), Style::default().fg(Color::LightYellow))
                    }
                    CellDisplay::Light(n) => match n {
                        1 => ("·1".to_string(), Style::default().fg(Color::Yellow)),
                        2 => ("▒2".to_string(), Style::default().fg(Color::Yellow)),
                        3 => ("▓3".to_string(), Style::default().fg(Color::Yellow)),
                        4 => (
                            "█4".to_string(),
                            Style::default().fg(Color::Yellow), //.add_modifier(Modifier::BOLD),
                        ),
                        _ => (
                            " 0".to_string(),
                            Style::default().fg(Color::Gray), //.add_modifier(Modifier::BOLD),
                        ),
                    },
                    CellDisplay::Flag => ("P".to_string(), Style::default().fg(Color::Red)),
                    CellDisplay::Dark => (" ".to_string(), Style::default().fg(Color::Black)),
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

// 右側內容：設定
fn draw_settings_content(frame: &mut Frame, _app: &mut App, area: Rect) {
    let para = Paragraph::new("設定畫面（未實作）")
        .block(Block::default().borders(Borders::ALL).title("Settings"));
    frame.render_widget(para, area);
}

// 右側內容：說明
fn draw_help_content(frame: &mut Frame, _app: &mut App, area: Rect) {
    let help_text = "Help for Akari Game\n\nControls:\nG - Start New Game\nA - Open Archive\nS - Open Settings\nH - Show This Help\nE - Exit Game\nQ - Quit Current Screen";
    let para = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .wrap(Wrap { trim: true });
    frame.render_widget(para, area);
}

// 右側內容：離開確認
fn draw_exiting_content(frame: &mut Frame, _app: &mut App, area: Rect) {
    let exiting_text = "Do you want to exit?\n\nPress Enter to exit\nPress Q to return to menu";
    let para = Paragraph::new(exiting_text)
        .block(Block::default().borders(Borders::ALL).title("Exiting"))
        .wrap(Wrap { trim: true });
    frame.render_widget(para, area);
}

fn draw_win(frame: &mut Frame, _app: &mut App, area: Rect) {
    let text = "🎉 恭喜過關！\n\nPress Q to return to menu";
    let para = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("You Win!"))
        .wrap(Wrap { trim: true });
    frame.render_widget(para, area);
}
