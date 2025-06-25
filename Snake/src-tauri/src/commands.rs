use crate::game::{GameEngine, engine::GameConfig, snake::Direction};

/// Test command to verify game logic
#[tauri::command]
pub fn test_game_logic() -> Result<String, String> {
    // Create a new game
    let config = GameConfig::default();
    let mut engine = GameEngine::new(config);
    
    // Test basic movement
    let initial_head = engine.snake.head();
    engine.update();
    let new_head = engine.snake.head();
    
    if new_head == initial_head {
        return Err("Snake did not move".to_string());
    }
    
    // Test direction change
    engine.change_direction(Direction::Down);
    let down_head = engine.snake.head();
    engine.update();
    let after_down = engine.snake.head();
    
    if after_down.y <= down_head.y {
        return Err("Snake did not move down".to_string());
    }
    
    // Test pause functionality
    engine.toggle_pause();
    if !engine.is_paused() {
        return Err("Game did not pause".to_string());
    }
    
    engine.toggle_pause();
    if !engine.is_playing() {
        return Err("Game did not resume".to_string());
    }
    
    Ok("All game logic tests passed!".to_string())
}

/// Get game state for testing
#[tauri::command]
pub fn get_game_state() -> Result<serde_json::Value, String> {
    let config = GameConfig::default();
    let engine = GameEngine::new(config);
    
    let state = serde_json::json!({
        "snake_length": engine.snake.length(),
        "snake_head": {
            "x": engine.snake.head().x,
            "y": engine.snake.head().y
        },
        "food_position": {
            "x": engine.food.position().x,
            "y": engine.food.position().y
        },
        "score": engine.stats.score,
        "is_playing": engine.is_playing(),
        "current_speed": engine.current_speed()
    });
    
    Ok(state)
} 