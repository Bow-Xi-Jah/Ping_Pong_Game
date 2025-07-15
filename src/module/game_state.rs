use sdl2::keyboard::{Scancode, KeyboardState};


#[derive(PartialEq, Clone, Copy)]
pub enum GameState {
    GAME_RUNNING,
    GAME_PAUSED,
    GAME_QUIT
}

impl GameState {
    pub fn pause(&mut self){
        if *self == GameState::GAME_RUNNING {
                *self = GameState::GAME_PAUSED;
            }
            else if *self == GameState::GAME_PAUSED {
                *self = GameState::GAME_RUNNING;
            }
    }
}