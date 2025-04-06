use chrono::{Duration, TimeDelta};
use macroquad::prelude::*;

use crate::{
    system::{BG_COLOR, FG_COLOR, TEXTURE_STORAGE},
    windows::{draw_outlined_box, Window, WindowReturnAction},
};

const CELL_SIZE: f32 = 40.0;
const NUM_OF_CELLS: usize = 20;
const DURATION_BETWEEN_MOVES: TimeDelta = Duration::milliseconds(500);

#[derive(Copy, Clone)]
enum Cell {
    Empty,
    Player,
    Obstacle,
}

pub struct MiniGame {
    cells: [[Cell; NUM_OF_CELLS]; NUM_OF_CELLS],
    player_position: (usize, usize),
    top_left: Vec2,
}

impl MiniGame {
    pub fn new() -> Self {
        MiniGame {
            cells: [[Cell::Empty; NUM_OF_CELLS]; NUM_OF_CELLS],
            player_position: (0, 0),
            top_left: vec2(200.0, 180.0),
        }
    }
}

impl Window for MiniGame {
    async fn new_boxed() -> Box<dyn Window>
    where
        Self: Sized,
    {
        Box::new(Self::new())
    }

    fn position(&self) -> Vec2 {
        self.top_left
    }

    fn top_left(&self) -> Vec2 {
        self.top_left
    }

    fn draw(&mut self) {
        let width = CELL_SIZE * NUM_OF_CELLS as f32;
        let height = width;
        draw_outlined_box(
            self.top_left.x,
            self.top_left.y,
            width,
            height,
            5.0,
            BG_COLOR,
            FG_COLOR,
        );
    }

    fn is_visible(&self) -> bool {
        true
    }

    fn set_visibility(&mut self, _value: bool) {}

    fn handle_input(&mut self, _event: crate::windows::InputEvent) -> WindowReturnAction {
        WindowReturnAction::None
    }

    fn icon(&self) -> Option<Texture2D> {
        unsafe { TEXTURE_STORAGE.minigame() }
    }

    fn contains_pos(&self, pos: Vec2) -> bool {
        false
    }
}
