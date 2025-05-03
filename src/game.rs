//use ratatui::widgets::{List, ListItem, ListState};

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct PuzzleMetadata {
    pub puzzle_type: String,
    pub author: String,
    pub solver: String,
    pub source: String,
    pub info: String,
    pub size: PuzzleSize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PuzzleSize {
    pub cols: usize,
    pub rows: usize,
    pub unit: u32,
}

#[derive(Debug)]
pub struct Puzzle {
    pub id: u32,
    pub metadata: PuzzleMetadata,
    pub problem: Vec<Vec<String>>,
    //pub solution: Vec<Vec<String>>,
}

#[derive(PartialEq)]
pub enum GameState {
    Ready,
    Playing,
    //Paused,
    GameOver,
}

#[derive(PartialEq, Clone, Copy)]
pub enum CellType {
    Wall,
    Target(u8),
    Empty,
}

#[derive(PartialEq, Clone, Copy)]
pub enum LightState {
    IsWall,
    Light(u8),
    Dark,
}

impl LightState {
    pub fn light(value: u8) -> Self {
        LightState::Light(value)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum PlayerObject {
    IsWall,
    Lightbulb,
    Flag,
    Empty,
}

#[derive(PartialEq, Clone, Copy)]
pub enum CellDisplay {
    Wall,
    Target(u8),
    LightBulb,
    Light(u8),
    Flag,
    Dark,
}

pub enum PlayerOperation {
    AddLightbulb,
    AddFlag,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Game {
    pub state: GameState,
    pub puzzle: Option<Puzzle>,
    pub board: Vec<Vec<CellType>>,
    pub light_state: Vec<Vec<LightState>>,
    pub player_objects: Vec<Vec<PlayerObject>>,
    //pub display: Vec<Vec<CellDisplay>>,
    pub target_remain: Vec<Vec<Option<i8>>>,
    pub cursor_position: (usize, usize),
    //pub player_position_state: Vec<ListState>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState::Ready,
            puzzle: None,
            board: Vec::new(),
            light_state: Vec::new(),
            player_objects: Vec::new(),
            //display: Vec::new(),
            target_remain: Vec::new(),
            cursor_position: (0, 0),
            //player_position_state: Vec::new(),
        }
    }

    pub fn init_game(&mut self, puzzle_id: u32) -> Result<(), Box<dyn std::error::Error>> {
        // Calculate which archive folder to look in
        let folder = ((puzzle_id - 1) / 100 + 1).to_string();
        let file_path = format!("archive/{}/{:03}.json", folder, puzzle_id);

        // Read puzzle file
        let puzzle_data = fs::read_to_string(Path::new(&file_path))?;

        // Parse JSON data
        let puzzle_json: serde_json::Value = serde_json::from_str(&puzzle_data)?;

        // Extract metadata
        let metadata: PuzzleMetadata = serde_json::from_value(puzzle_json["metadata"].clone())?;

        // Extract problem and solution
        let problem: Vec<Vec<String>> = serde_json::from_value(puzzle_json["problem"].clone())?;
        //let solution: Vec<Vec<String>> = serde_json::from_value(puzzle_json["solution"].clone())?;

        // Initialize player position state
        //let player_position_state = vec![ListState::default(); 1];

        // Create puzzle
        self.puzzle = Some(Puzzle {
            id: puzzle_id,
            metadata,
            problem,
            //solution,
        });

        // Initialize board
        self.init_board();

        Ok(())
    }

    fn init_board(&mut self) {
        if let Some(puzzle) = &self.puzzle {
            let rows = puzzle.metadata.size.rows;
            let cols = puzzle.metadata.size.cols;

            // Initialize board with empty cells
            self.board = vec![vec![CellType::Empty; cols]; rows];
            self.light_state = vec![vec![LightState::Dark; cols]; rows];
            self.player_objects = vec![vec![PlayerObject::Empty; cols]; rows];

            // Set up initial board state based on puzzle problem
            for (i, row) in puzzle.problem.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {
                    match cell.as_str() {
                        "x" => {
                            self.board[i][j] = CellType::Wall;
                            self.light_state[i][j] = LightState::IsWall;
                            self.player_objects[i][j] = PlayerObject::IsWall;
                        }
                        "0" => {
                            self.board[i][j] = CellType::Target(0);
                            self.light_state[i][j] = LightState::IsWall;
                            self.player_objects[i][j] = PlayerObject::IsWall;
                        }
                        "1" => {
                            self.board[i][j] = CellType::Target(1);
                            self.light_state[i][j] = LightState::IsWall;
                            self.player_objects[i][j] = PlayerObject::IsWall;
                        }
                        "2" => {
                            self.board[i][j] = CellType::Target(2);
                            self.light_state[i][j] = LightState::IsWall;
                            self.player_objects[i][j] = PlayerObject::IsWall;
                        }
                        "3" => {
                            self.board[i][j] = CellType::Target(3);
                            self.light_state[i][j] = LightState::IsWall;
                            self.player_objects[i][j] = PlayerObject::IsWall;
                        }
                        "4" => {
                            self.board[i][j] = CellType::Target(4);
                            self.light_state[i][j] = LightState::IsWall;
                            self.player_objects[i][j] = PlayerObject::IsWall;
                        }
                        _ => {
                            self.board[i][j] = CellType::Empty;
                            self.player_objects[i][j] = PlayerObject::Empty;
                        }
                    }
                }
            }
            self.target_remain = vec![vec![None; cols]; rows];
            for (i, row) in puzzle.problem.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {
                    match cell.as_str() {
                        "0" => self.target_remain[i][j] = Some(0),
                        "1" => self.target_remain[i][j] = Some(1),
                        "2" => self.target_remain[i][j] = Some(2),
                        "3" => self.target_remain[i][j] = Some(3),
                        "4" => self.target_remain[i][j] = Some(4),
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn start(&mut self) {
        self.state = GameState::Playing;
        //panic!()
    }
    pub fn quit(&mut self) {
        //self.state = GameState::GameOver;
    }

    pub fn check_win(&self) -> bool {
        for i in 0..self.board.len() {
            for j in 0..self.board[0].len() {
                match self.board[i][j] {
                    CellType::Empty => {
                        if self.light_state[i][j] == LightState::Dark {
                            return false;
                        }
                    }
                    CellType::Target(_) | CellType::Wall => {}
                }
            }
        }
        for i in 0..self.board.len() {
            for j in 0..self.board[0].len() {
                if let Some(remain) = self.target_remain[i][j] {
                    if remain != 0 {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn update(&mut self) {
        // 1. clear all non-wall cells
        for i in 0..self.light_state.len() {
            for j in 0..self.light_state[i].len() {
                if self.light_state[i][j] != LightState::IsWall {
                    self.light_state[i][j] = LightState::light(0);
                }
            }
        }
        // 2. propagate
        for i in 0..self.light_state.len() {
            for j in 0..self.light_state[i].len() {
                self.update_light_state_cross(i, j);
            }
        }
        // 3. set light 0 into Dark
        for i in 0..self.light_state.len() {
            for j in 0..self.light_state[i].len() {
                if self.light_state[i][j] == LightState::light(0) {
                    self.light_state[i][j] = LightState::Dark;
                }
            }
        }
        //4. target calculate
        for i in 0..self.board.len() {
            for j in 0..self.board[0].len() {
                if let CellType::Target(orig) = self.board[i][j] {
                    let mut count = 0;
                    for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let ni = i as isize + di;
                        let nj = j as isize + dj;
                        if ni >= 0
                            && ni < self.board.len() as isize
                            && nj >= 0
                            && nj < self.board[0].len() as isize
                        {
                            let (ni, nj) = (ni as usize, nj as usize);
                            if self.player_objects[ni][nj] == PlayerObject::Lightbulb {
                                count += 1;
                            }
                        }
                    }
                    let remain = (orig as i8) - count;
                    self.target_remain[i][j] = Some(remain);
                }
            }
        }
        if self.check_win() {
            self.state = GameState::GameOver;
        }
    }

    //every light bulb will shoot cross direction light until it is blocked by wall
    //every light state in the area will be updated +1

    fn update_light_state_cross(&mut self, row: usize, col: usize) {
        //if self is a light bulb
        if self.player_objects[row][col] != PlayerObject::Lightbulb {
            return;
        }

        let rows = self.light_state.len();
        let cols = self.light_state[0].len();

        // Check if the position is within bounds
        if row >= rows || col >= cols {
            return;
        }

        // Update the light state at the current position
        // update up direction
        for i in (0..row).rev() {
            if self.player_objects[i][col] == PlayerObject::IsWall {
                break;
            }
            self.light_state[i][col] = match self.light_state[i][col] {
                LightState::Light(n) => LightState::Light(n + 1),
                LightState::Dark => LightState::Light(1),
                LightState::IsWall => LightState::IsWall,
            };
        }
        //update down direction
        for i in row + 1..rows {
            if self.player_objects[i][col] == PlayerObject::IsWall {
                break;
            }
            self.light_state[i][col] = match self.light_state[i][col] {
                LightState::Light(n) => LightState::Light(n + 1),
                LightState::Dark => LightState::Light(1),
                LightState::IsWall => LightState::IsWall,
            };
        }
        //update left direction
        for j in (0..col).rev() {
            if self.player_objects[row][j] == PlayerObject::IsWall {
                break;
            }
            self.light_state[row][j] = match self.light_state[row][j] {
                LightState::Light(n) => LightState::Light(n + 1),
                LightState::Dark => LightState::Light(1),
                LightState::IsWall => LightState::IsWall,
            };
        }
        //update right direction
        for j in col + 1..cols {
            if self.player_objects[row][j] == PlayerObject::IsWall {
                break;
            }
            self.light_state[row][j] = match self.light_state[row][j] {
                LightState::Light(n) => LightState::Light(n + 1),
                LightState::Dark => LightState::Light(1),
                LightState::IsWall => LightState::IsWall,
            };
        }
        //update self
        self.light_state[row][col] = LightState::light(4);
    }

    fn display_priority(&self, row: usize, col: usize) -> CellDisplay {
        //// Wall has highest priority
        if self.board[row][col] == CellType::Wall {
            return CellDisplay::Wall;
        } else
        // Target numbers next
        if let CellType::Target(_orig) = self.board[row][col] {
            if let Some(remain) = self.target_remain[row][col] {
                return CellDisplay::Target(remain as u8);
            }
        } else
        // Light bulbs placed by player
        if self.player_objects[row][col] == PlayerObject::Lightbulb {
            return CellDisplay::LightBulb;
        } else
        // Light level from propagation
        if let LightState::Light(n) = self.light_state[row][col] {
            return CellDisplay::Light(n);
        } else
        // Flags placed by player
        if self.player_objects[row][col] == PlayerObject::Flag {
            return CellDisplay::Flag;
        }

        //Dark
        CellDisplay::Dark
    }

    pub fn get_display(&self) -> Vec<Vec<CellDisplay>> {
        let mut display = vec![vec![CellDisplay::Dark; self.board[0].len()]; self.board.len()];
        for i in 0..self.board.len() {
            for j in 0..self.board[0].len() {
                display[i][j] = self.display_priority(i, j);
            }
        }
        display
    }

    pub fn player_move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if self.cursor_position.0 > 0 {
                    self.cursor_position.0 -= 1;
                }
            }
            Direction::Down => {
                if self.cursor_position.0 < self.board.len() - 1 {
                    self.cursor_position.0 += 1;
                }
            }
            Direction::Left => {
                if self.cursor_position.1 > 0 {
                    self.cursor_position.1 -= 1;
                }
            }
            Direction::Right => {
                if self.cursor_position.1 < self.board[0].len() - 1 {
                    self.cursor_position.1 += 1;
                }
            }
        }
    }

    pub fn player_operation(&mut self, operation: PlayerOperation) {
        let (row, col) = self.cursor_position;

        match operation {
            PlayerOperation::AddLightbulb => {
                match self.player_objects[row][col] {
                    PlayerObject::Lightbulb => {
                        //if empty
                        self.player_objects[row][col] = PlayerObject::Empty;
                    }
                    PlayerObject::Empty => {
                        //only allow in dark
                        if self.light_state[row][col] != LightState::Dark {
                            return;
                        }
                        //check if target is already satisfied
                        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                            let ni = row as isize + di;
                            let nj = col as isize + dj;
                            if ni >= 0
                                && ni < self.board.len() as isize
                                && nj >= 0
                                && nj < self.board[0].len() as isize
                            {
                                let (ni, nj) = (ni as usize, nj as usize);
                                if let Some(remain) = self.target_remain[ni][nj] {
                                    if remain <= 0 {
                                        return;
                                    }
                                }
                            }
                        }
                        self.player_objects[row][col] = PlayerObject::Lightbulb;
                    }
                    PlayerObject::Flag => {
                        self.player_objects[row][col] = PlayerObject::Empty;
                    }
                    _ => {}
                }
            }
            PlayerOperation::AddFlag => match self.player_objects[row][col] {
                PlayerObject::Empty => {
                    if self.light_state[row][col] == LightState::Dark {
                        self.player_objects[row][col] = PlayerObject::Flag;
                    }
                }
                PlayerObject::Flag => {
                    self.player_objects[row][col] = PlayerObject::Empty;
                }
                PlayerObject::Lightbulb => {
                    self.player_objects[row][col] = PlayerObject::Empty;
                }
                _ => {}
            },
        }
    }
}
