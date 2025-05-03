pub enum GameState {
    Ready,
    Playing,
    Paused,
    GameOver,
}
pub enum CellType {
    Wall,
    Target(u8),
    Empty,
}

pub enum CellState {
    IsWall,
    Light,
    Dark,
}

pub enum PlayerObject {
    IsWall,
    Light,
    Dark,
    None,
}

pub struct Game {
    pub state: GameState,
    pub puzzle: Puzzle,
    pub board: Vec<Vec<CellState>>,
    pub cells_state: Vec<Vec<CellState>>,
    pub player_objects: Vec<Vec<PlayerObject>>,
}

pub struct Puzzle {
    pub id: u32,
    pub title: String,
    pub description: String,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState::Ready,
        }
    }

    fn init_game(&mut self) {
        // Initialize game state
    }

    fn start_game(&mut self) {
        // Start game logic
    }

    fn update_game(&mut self) {
        // Update game logic
    }

    fn handle_input(&mut self) {
        // Handle user input
    }

    fn update_player_objects(&mut self) {
        // Update player objects
    }
}
