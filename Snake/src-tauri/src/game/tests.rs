use crate::game::{snake::{Snake, Direction, Position}, food::Food, engine::{GameEngine, GameConfig}};

#[test]
fn test_snake_creation() {
    let start_pos = Position { x: 10, y: 10 };
    let snake = Snake::new(start_pos, 3);
    
    assert_eq!(snake.length(), 3);
    assert_eq!(snake.head(), Position { x: 10, y: 10 });
    assert_eq!(snake.direction, Direction::Right);
}

#[test]
fn test_snake_movement() {
    let start_pos = Position { x: 5, y: 5 };
    let mut snake = Snake::new(start_pos, 3);
    
    // Move right
    snake.move_forward(20, 20);
    assert_eq!(snake.head(), Position { x: 6, y: 5 });
    
    // Move down
    snake.change_direction(Direction::Down);
    snake.move_forward(20, 20);
    assert_eq!(snake.head(), Position { x: 6, y: 6 });
}

#[test]
fn test_wall_wrapping() {
    let start_pos = Position { x: 19, y: 0 };
    let mut snake = Snake::new(start_pos, 1);
    
    // Move right (should wrap to x=0)
    snake.move_forward(20, 20);
    assert_eq!(snake.head(), Position { x: 0, y: 0 });
    
    // Move up (should wrap to y=19)
    snake.change_direction(Direction::Up);
    snake.move_forward(20, 20);
    assert_eq!(snake.head(), Position { x: 0, y: 19 });
}

#[test]
fn test_direction_change_prevention() {
    let mut snake = Snake::new(Position { x: 5, y: 5 }, 1);
    
    // Should not be able to turn 180 degrees
    snake.change_direction(Direction::Left);
    assert_eq!(snake.direction, Direction::Right); // Should remain unchanged
    
    // Should be able to turn 90 degrees
    snake.change_direction(Direction::Down);
    assert_eq!(snake.direction, Direction::Down);
}

#[test]
fn test_snake_growth() {
    let start_pos = Position { x: 5, y: 5 };
    let mut snake = Snake::new(start_pos, 1);
    let initial_length = snake.length();
    
    snake.grow();
    snake.move_forward(20, 20);
    
    assert_eq!(snake.length(), initial_length + 1);
}

#[test]
fn test_food_generation() {
    let snake = Snake::new(Position { x: 5, y: 5 }, 3);
    let food_pos = Food::generate_new_position(&snake, 20, 20);
    
    // Food should not be on snake
    assert!(!snake.contains(food_pos));
    
    // Food should be within bounds
    assert!(food_pos.x >= 0 && food_pos.x < 20);
    assert!(food_pos.y >= 0 && food_pos.y < 20);
}

#[test]
fn test_game_engine_creation() {
    let config = GameConfig::default();
    let engine = GameEngine::new(config);
    
    assert_eq!(engine.snake.length(), 3);
    assert!(engine.is_playing());
    assert_eq!(engine.stats.score, 0);
}

#[test]
fn test_game_engine_movement() {
    let config = GameConfig::default();
    let mut engine = GameEngine::new(config);
    let initial_head = engine.snake.head();
    
    engine.update();
    
    // Snake should have moved
    assert_ne!(engine.snake.head(), initial_head);
}

#[test]
fn test_game_pause_toggle() {
    let config = GameConfig::default();
    let mut engine = GameEngine::new(config);
    
    assert!(engine.is_playing());
    
    engine.toggle_pause();
    assert!(engine.is_paused());
    
    engine.toggle_pause();
    assert!(engine.is_playing());
} 