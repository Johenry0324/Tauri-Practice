use crate::game::snake::{Position, Snake};
use serde::{Deserialize, Serialize};

/// Food entity in the game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Food {
    pub position: Position,
}

impl Food {
    /// Create new food at the given position
    pub fn new(position: Position) -> Self {
        Self { position }
    }
    
    /// Generate new food position that doesn't collide with snake
    pub fn generate_new_position(
        snake: &Snake,
        board_width: i32,
        board_height: i32,
    ) -> Position {
        use rand::Rng;
        
        let mut rng = rand::thread_rng();
        let mut new_position;
        
        // Keep generating until we find a position not occupied by snake
        loop {
            new_position = Position {
                x: rng.gen_range(0..board_width),
                y: rng.gen_range(0..board_height),
            };
            
            if !snake.contains(new_position) {
                break;
            }
        }
        
        new_position
    }
    
    /// Check if food is eaten by snake
    pub fn is_eaten_by(&self, snake: &Snake) -> bool {
        snake.head() == self.position
    }
    
    /// Get food position
    pub fn position(&self) -> Position {
        self.position
    }
} 