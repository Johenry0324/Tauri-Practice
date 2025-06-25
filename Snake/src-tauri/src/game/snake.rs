use serde::{Deserialize, Serialize};

/// Position in the game grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// Direction of snake movement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Snake game entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snake {
    pub body: Vec<Position>,
    pub direction: Direction,
    pub growing: bool,
}

impl Snake {
    /// Create a new snake at the given position
    pub fn new(start_pos: Position, initial_length: usize) -> Self {
        let mut body = Vec::with_capacity(initial_length);
        
        // Create initial body segments
        for i in 0..initial_length {
            body.push(Position {
                x: start_pos.x - i as i32,
                y: start_pos.y,
            });
        }
        
        Self {
            body,
            direction: Direction::Right,
            growing: false,
        }
    }
    
    /// Get the head position
    pub fn head(&self) -> Position {
        self.body[0]
    }
    
    /// Get the tail position
    pub fn tail(&self) -> Position {
        self.body[self.body.len() - 1]
    }
    
    /// Check if position is occupied by snake body
    pub fn contains(&self, pos: Position) -> bool {
        self.body.contains(&pos)
    }
    
    /// Change direction (prevent 180-degree turns)
    pub fn change_direction(&mut self, new_direction: Direction) {
        let can_change = match (self.direction, new_direction) {
            (Direction::Up, Direction::Down) => false,
            (Direction::Down, Direction::Up) => false,
            (Direction::Left, Direction::Right) => false,
            (Direction::Right, Direction::Left) => false,
            _ => true,
        };
        
        if can_change {
            self.direction = new_direction;
        }
    }
    
    /// Move snake in current direction
    pub fn move_forward(&mut self, board_width: i32, board_height: i32) {
        let head = self.head();
        let new_head = match self.direction {
            Direction::Up => Position {
                x: head.x,
                y: (head.y - 1 + board_height) % board_height,
            },
            Direction::Down => Position {
                x: head.x,
                y: (head.y + 1) % board_height,
            },
            Direction::Left => Position {
                x: (head.x - 1 + board_width) % board_width,
                y: head.y,
            },
            Direction::Right => Position {
                x: (head.x + 1) % board_width,
                y: head.y,
            },
        };
        
        // Add new head
        self.body.insert(0, new_head);
        
        // Remove tail unless growing
        if !self.growing {
            self.body.pop();
        } else {
            self.growing = false;
        }
    }
    
    /// Check if snake collides with itself
    pub fn self_collision(&self) -> bool {
        let head = self.head();
        self.body.iter().skip(1).any(|&segment| segment == head)
    }
    
    /// Make snake grow on next move
    pub fn grow(&mut self) {
        self.growing = true;
    }
    
    /// Get snake length
    pub fn length(&self) -> usize {
        self.body.len()
    }
} 