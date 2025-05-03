use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub enum CurrentScreen {
    Menu,
    Game,
    Archive,
    Settings,
    Help,
    Exiting,
}

#[derive(Debug)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub current_selection: CurrentScreen,
    exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Menu,
            current_selection: CurrentScreen::Menu,
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
            KeyCode::Char('g') => self.current_screen = CurrentScreen::Game,
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

    // Add other event handlers as needed
    fn handle_game_events(&mut self, key: KeyEvent) {
        // Handle game events
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
