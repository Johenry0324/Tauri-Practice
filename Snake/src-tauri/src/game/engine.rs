use crate::game::{snake::{Snake, Direction, Position}, food::Food};
use serde::{Deserialize, Serialize};

/// Game state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameState {
    Playing,
    Paused,
    GameOver,
}

/// Game configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub board_width: i32,
    pub board_height: i32,
    pub initial_snake_length: usize,
    pub initial_speed: u64, // milliseconds between moves
    pub speed_increment: u64,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            board_width: 20,
            board_height: 20,
            initial_snake_length: 3,
            initial_speed: 200,
            speed_increment: 10,
        }
    }
}

/// Game statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStats {
    pub score: u32,
    pub high_score: u32,
    pub games_played: u32,
    pub total_score: u32,
}

impl Default for GameStats {
    fn default() -> Self {
        Self {
            score: 0,
            high_score: 0,
            games_played: 0,
            total_score: 0,
        }
    }
}

/// Main game engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEngine {
    pub snake: Snake,
    pub food: Food,
    pub state: GameState,
    pub config: GameConfig,
    pub stats: GameStats,
    pub current_speed: u64,
}

impl GameEngine {
    /// Create a new game engine
    pub fn new(config: GameConfig) -> Self {
        let center_x = config.board_width / 2;
        let center_y = config.board_height / 2;
        let start_pos = Position { x: center_x, y: center_y };
        
        let snake = Snake::new(start_pos, config.initial_snake_length);
        let food = Food::new(Food::generate_new_position(&snake, config.board_width, config.board_height));
        
        Self {
            snake,
            food,
            state: GameState::Playing,
            config: config.clone(),
            stats: GameStats::default(),
            current_speed: config.initial_speed,
        }
    }
    
    /// Reset game to initial state
    pub fn reset(&mut self) {
        let center_x = self.config.board_width / 2;
        let center_y = self.config.board_height / 2;
        let start_pos = Position { x: center_x, y: center_y };
        
        self.snake = Snake::new(start_pos, self.config.initial_snake_length);
        self.food = Food::new(Food::generate_new_position(&self.snake, self.config.board_width, self.config.board_height));
        self.state = GameState::Playing;
        self.stats.score = 0;
        self.current_speed = self.config.initial_speed;
    }
    
    /// Start a new game
    pub fn start_new_game(&mut self) {
        self.update_stats();
        self.reset();
    }
    
    /// Change snake direction
    pub fn change_direction(&mut self, direction: Direction) {
        if matches!(self.state, GameState::Playing) {
            self.snake.change_direction(direction);
        }
    }
    
    /// Toggle pause state
    pub fn toggle_pause(&mut self) {
        match self.state {
            GameState::Playing => self.state = GameState::Paused,
            GameState::Paused => self.state = GameState::Playing,
            GameState::GameOver => (), // Can't pause when game over
        }
    }
    
    /// Update game state (called each frame)
    pub fn update(&mut self) {
        if !matches!(self.state, GameState::Playing) {
            return;
        }
        
        // Move snake
        self.snake.move_forward(self.config.board_width, self.config.board_height);
        
        // Check for self collision
        if self.snake.self_collision() {
            self.state = GameState::GameOver;
            return;
        }
        
        // Check if food is eaten
        if self.food.is_eaten_by(&self.snake) {
            self.snake.grow();
            self.stats.score += 10;
            self.increase_speed();
            
            // Generate new food
            let new_food_pos = Food::generate_new_position(&self.snake, self.config.board_width, self.config.board_height);
            self.food = Food::new(new_food_pos);
        }
    }
    
    /// Increase game speed
    fn increase_speed(&mut self) {
        self.current_speed = self.current_speed.saturating_sub(self.config.speed_increment);
        self.current_speed = self.current_speed.max(50); // Minimum 50ms
    }
    
    /// Update game statistics
    fn update_stats(&mut self) {
        self.stats.games_played += 1;
        self.stats.total_score += self.stats.score;
        
        if self.stats.score > self.stats.high_score {
            self.stats.high_score = self.stats.score;
        }
    }
    
    /// Get current game speed
    pub fn current_speed(&self) -> u64 {
        self.current_speed
    }
    
    /// Get snake body positions
    pub fn snake_body(&self) -> &[Position] {
        &self.snake.body
    }
    
    /// Get food position
    pub fn food_position(&self) -> Position {
        self.food.position()
    }
    
    /// Check if game is over
    pub fn is_game_over(&self) -> bool {
        matches!(self.state, GameState::GameOver)
    }
    
    /// Check if game is paused
    pub fn is_paused(&self) -> bool {
        matches!(self.state, GameState::Paused)
    }
    
    /// Check if game is playing
    pub fn is_playing(&self) -> bool {
        matches!(self.state, GameState::Playing)
    }
} 