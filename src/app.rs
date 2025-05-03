use crate::game::{Direction, Game, PlayerOperation};
use crossterm::event::{KeyCode, KeyEvent};
use rand::Rng;

#[derive(Debug)]
pub enum CurrentScreen {
    Menu,
    Game,
    Archive,
    Settings,
    Help,
    Exiting,
}

//#[derive(Debug)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub current_selection: CurrentScreen,
    pub game: Option<Game>,
    exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Menu,
            current_selection: CurrentScreen::Menu,
            game: None,
            exit: false,
        }
    }

    pub fn should_quit(&self) -> bool {
        self.exit
    }

    pub fn handle_event(&mut self, key: KeyEvent) {
        match self.current_screen {
            CurrentScreen::Menu => self.handle_menu_events(key),
            CurrentScreen::Game => self.handle_game_events(key),
            CurrentScreen::Archive => self.handle_archive_events(key),
            CurrentScreen::Settings => self.handle_settings_events(key),
            CurrentScreen::Help => self.handle_help_events(key),
            CurrentScreen::Exiting => self.exit = true,
        }
    }

    fn handle_menu_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('g') => {
                let puzzle_id = rand::thread_rng().gen_range(1..=750);
                if let Err(e) = self.start_game(puzzle_id) {
                    eprintln!("Failed to start game: {}", e);
                }
            }
            KeyCode::Char('a') => self.current_screen = CurrentScreen::Archive,
            KeyCode::Char('s') => self.current_screen = CurrentScreen::Settings,
            KeyCode::Char('h') => self.current_screen = CurrentScreen::Help,
            KeyCode::Char('e') => self.current_screen = CurrentScreen::Exiting,
            KeyCode::Up => {
                self.current_selection = match self.current_selection {
                    CurrentScreen::Game => CurrentScreen::Exiting,
                    CurrentScreen::Archive => CurrentScreen::Game,
                    CurrentScreen::Settings => CurrentScreen::Archive,
                    CurrentScreen::Help => CurrentScreen::Settings,
                    CurrentScreen::Exiting => CurrentScreen::Help,
                    _ => CurrentScreen::Menu,
                }
            }
            KeyCode::Down => {
                self.current_selection = match self.current_selection {
                    CurrentScreen::Game => CurrentScreen::Archive,
                    CurrentScreen::Archive => CurrentScreen::Settings,
                    CurrentScreen::Settings => CurrentScreen::Help,
                    CurrentScreen::Help => CurrentScreen::Exiting,
                    CurrentScreen::Exiting => CurrentScreen::Game,
                    _ => CurrentScreen::Menu,
                }
            }
            KeyCode::Enter => {
                self.current_screen = match self.current_selection {
                    CurrentScreen::Game => CurrentScreen::Game,
                    CurrentScreen::Archive => CurrentScreen::Archive,
                    CurrentScreen::Settings => CurrentScreen::Settings,
                    CurrentScreen::Help => CurrentScreen::Help,
                    CurrentScreen::Exiting => CurrentScreen::Exiting,
                    _ => CurrentScreen::Menu,
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
        Ok(())
    }

    // Add other event handlers as needed
    fn handle_game_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.current_screen = CurrentScreen::Menu,
            KeyCode::Up => {
                if let Some(game) = &mut self.game {
                    game.move_cursor(Direction::Up);
                }
            }
            KeyCode::Down => {
                if let Some(game) = &mut self.game {
                    game.move_cursor(Direction::Down);
                }
            }
            KeyCode::Left => {
                if let Some(game) = &mut self.game {
                    game.move_cursor(Direction::Left);
                }
            }
            KeyCode::Right => {
                if let Some(game) = &mut self.game {
                    game.move_cursor(Direction::Right);
                }
            }
            KeyCode::Char(' ') => {
                if let Some(game) = &mut self.game {
                    game.player_operation(PlayerOperation::AddLightbulb);
                }
            }
            KeyCode::Char('p') => {
                if let Some(game) = &mut self.game {
                    game.player_operation(PlayerOperation::AddFlag);
                }
            }
            _ => {}
        }
    }

    fn handle_archive_events(&mut self, key: KeyEvent) {
        // Handle archive events
    }

    fn handle_settings_events(&mut self, key: KeyEvent) {
        // Handle settings events
    }

    fn handle_help_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.current_screen = CurrentScreen::Menu,
            _ => {}
        }
    }
}
