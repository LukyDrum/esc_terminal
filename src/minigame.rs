use chrono::{Duration, TimeDelta};
use macroquad::prelude::*;

use crate::{
    system::{BG_COLOR, FG_COLOR},
    windows::{draw_outlined_box, Window},
};

const CELL_SIZE: f32 = 20.0;
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
    is_visible: bool,
}

impl Window for MiniGame {
    async fn new_boxed() -> Box<dyn Window>
    where
        Self: Sized,
    {
        todo!()
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
        self.is_visible
    }

    fn set_visibility(&mut self, _value: bool) {}

    fn handle_input(
        &mut self,
        _event: crate::windows::InputEvent,
    ) -> crate::windows::WindowReturnAction {
        todo!()
    }

    fn icon(&self) -> Option<Texture2D> {
        None
    }

    fn contains_pos(&self, pos: Vec2) -> bool {
        false
    }
}
