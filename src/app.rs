use crate::game::{Direction, Game, PlayerOperation};
use crossterm::event::{KeyCode, KeyEvent};
use rand::Rng;
use ratatui::widgets::ListState;
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq)]
pub enum CurrentScreen {
    Menu,
    Game,
    Archive,
    Settings,
    Help,
    Exiting,
    Win,
}

//#[derive(Debug)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub menu_list: ListState,
    pub archive_list: ListState,
    pub game: Option<Game>,
    pub timer_start: Option<Instant>,
    pub timer_elapsed: Duration,
    exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Menu,
            menu_list: ListState::default(),
            archive_list: ListState::default(),
            game: None,
            timer_start: None,
            timer_elapsed: Duration::ZERO,
            exit: false,
        }
    }

    pub fn should_quit(&self) -> bool {
        self.exit
    }

    pub fn timer_string(&self) -> String {
        let elapsed = if self.current_screen == CurrentScreen::Game {
            if let Some(start) = self.timer_start {
                start.elapsed()
            } else {
                self.timer_elapsed
            }
        } else {
            self.timer_elapsed
        };
        let secs = elapsed.as_secs();
        format!("{:02}:{:02}", secs / 60, secs % 60)
    }

    pub fn handle_event(&mut self, key: KeyEvent) {
        match self.current_screen {
            CurrentScreen::Menu => self.handle_menu_events(key),
            CurrentScreen::Game => self.handle_game_events(key),
            CurrentScreen::Archive => self.handle_archive_events(key),
            CurrentScreen::Settings => self.handle_settings_events(key),
            CurrentScreen::Help => self.handle_help_events(key),
            CurrentScreen::Exiting => self.handle_exiting_events(key),
            CurrentScreen::Win => self.handle_win_events(key),
        }
    }

    fn handle_menu_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('g') => {
                let puzzle_id = rand::rng().random_range(1..=750);
                if let Err(e) = self.start_game(puzzle_id) {
                    eprintln!("Failed to start game: {}", e);
                }
            }
            KeyCode::Char('a') => self.current_screen = CurrentScreen::Archive,
            KeyCode::Char('s') => self.current_screen = CurrentScreen::Settings,
            KeyCode::Char('h') => self.current_screen = CurrentScreen::Help,
            KeyCode::Char('e') => self.current_screen = CurrentScreen::Exiting,
            KeyCode::Up => {
                self.menu_list.select_previous();
            }
            KeyCode::Down => {
                self.menu_list.select_next();
            }
            KeyCode::Enter => {
                if let Some(selected) = self.menu_list.selected() {
                    match selected {
                        0 => {
                            // New Game
                            let puzzle_id = rand::rng().random_range(1..=750);
                            if let Err(e) = self.start_game(puzzle_id) {
                                eprintln!("Failed to start game: {}", e);
                            }
                        }
                        1 => self.current_screen = CurrentScreen::Archive,
                        2 => self.current_screen = CurrentScreen::Settings,
                        3 => self.current_screen = CurrentScreen::Help,
                        4 => self.current_screen = CurrentScreen::Exiting,
                        _ => self.current_screen = CurrentScreen::Menu,
                    }
                }
            }
            _ => {}
        }
    }

    pub fn start_game(&mut self, puzzle_id: u32) -> Result<(), Box<dyn std::error::Error>> {
        let mut game = Game::new();
        game.init_game(puzzle_id)?;
        game.start();
        self.game = Some(game);
        self.current_screen = CurrentScreen::Game;
        self.timer_start = Some(Instant::now());
        self.timer_elapsed = Duration::ZERO;
        Ok(())
    }

    fn check_gameover(&mut self) {
        if let Some(game) = &self.game {
            if game.state == crate::game::GameState::GameOver {
                self.current_screen = CurrentScreen::Win;
                // 記錄最終時間
                if let Some(start) = self.timer_start.take() {
                    self.timer_elapsed = start.elapsed();
                }
            }
        }
    }

    // Add other event handlers as needed
    fn handle_game_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                if let Some(game) = &mut self.game {
                    game.quit();
                }
                self.current_screen = CurrentScreen::Menu;
            }
            KeyCode::Char('u') => {
                //undo
                if let Some(game) = &mut self.game {
                    game.undo();
                }
            }
            KeyCode::Up => {
                if let Some(game) = &mut self.game {
                    game.player_move_cursor(Direction::Up);
                    game.update();
                }
            }
            KeyCode::Down => {
                if let Some(game) = &mut self.game {
                    game.player_move_cursor(Direction::Down);
                    game.update();
                }
            }
            KeyCode::Left => {
                if let Some(game) = &mut self.game {
                    game.player_move_cursor(Direction::Left);
                    game.update();
                }
            }
            KeyCode::Right => {
                if let Some(game) = &mut self.game {
                    game.player_move_cursor(Direction::Right);
                    game.update();
                }
            }
            KeyCode::Char(' ') => {
                if let Some(game) = &mut self.game {
                    game.player_operation(PlayerOperation::AddLightbulb);
                    game.update();
                }
            }
            KeyCode::Char('f') => {
                if let Some(game) = &mut self.game {
                    game.player_operation(PlayerOperation::AddFlag);
                    game.update();
                }
            }
            _ => {}
        }
        self.check_gameover();
    }

    fn handle_archive_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.current_screen = CurrentScreen::Menu,
            KeyCode::Up => {
                self.archive_list.select_previous();
            }
            KeyCode::Down => {
                self.archive_list.select_next();
            }
            KeyCode::Enter => {
                if let Some(selected) = self.archive_list.selected() {
                    let puzzle_id = (selected + 1) as u32;
                    if let Err(e) = self.start_game(puzzle_id) {
                        eprintln!("Failed to start game: {}", e);
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_settings_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.current_screen = CurrentScreen::Menu,
            _ => {}
        }
    }

    fn handle_help_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.current_screen = CurrentScreen::Menu,
            _ => {}
        }
    }

    fn handle_exiting_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.current_screen = CurrentScreen::Menu,
            KeyCode::Enter => self.exit = true,
            _ => {}
        }
    }

    fn handle_win_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.current_screen = CurrentScreen::Menu,
            _ => {}
        }
    }
}
