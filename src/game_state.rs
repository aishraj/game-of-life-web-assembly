use models::Grid;
use models::Size;
use std::mem::swap;

pub struct GameState {
  pub current_generation: Grid,
  pub next_generation: Grid
}

impl GameState {
    pub fn new(size: Size) -> GameState {
        GameState {
            current_generation: Grid::new(size),
            next_generation: Grid::new(size.clone())
        }
    }

    pub fn flip_generations(&mut self) {
      swap(&mut self.current_generation, &mut self.next_generation);
    }
}