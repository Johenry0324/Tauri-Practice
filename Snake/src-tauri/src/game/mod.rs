// Game module for Snake game
pub mod engine;
pub mod snake;
pub mod food;

// Re-export main types
pub use engine::GameEngine;
pub use snake::Snake;
pub use food::Food;

#[cfg(test)]
mod tests; 