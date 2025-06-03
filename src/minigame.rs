use std::time::{Duration, Instant};

use macroquad::prelude::*;

use crate::{
    system::{texture_storage, BG_COLOR, FG_COLOR},
    windows::{draw_outlined_box, Window, WindowReturnAction},
};

const CELL_SIZE: f32 = 40.0;
const NUM_OF_CELLS: usize = 20;
const DURATION_BETWEEN_MOVES: Duration = Duration::from_millis(200);

const EMPTY_COLOR: Color = BG_COLOR;
const PLAYER_COLOR: Color = GREEN;
const OBSTACLE_COLOR: Color = BLACK;
const PASSWORD_COLOR: Color = BLUE;
const FINISH_COLOR: Color = GOLD;

const MAP: &'static str = "
OOOOOOOOOOOOOOOOOOOO
O________X_________O
O__OOOOOOOOOOOOO___O
O______________O___O
O______________O___O
OOOOOOOOOOOOO__O_X_O
O______________O___O
O______________O___O
O__OOOOOOOOOOOOO___O
O__________________O
O________X_________O
O__________________O
O____________O_____O
OP___________O_X___O
O____________O_____O
O__________________O
O__________________O
O__________OOOOOOO_O
O_________________XO
OOOOOOOOOOOOOOOOOOOO";
const KEYS_TOTAL: u8 = 5;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Player,
    Obstacle,
    PasswordPiece,
    Finish,
}

fn cells_from_string(string: &'static str) -> ([[Cell; NUM_OF_CELLS]; NUM_OF_CELLS], (i32, i32)) {
    let mut cells = [[Cell::Empty; NUM_OF_CELLS]; NUM_OF_CELLS];

    let mut row = 0;
    let mut col = 0;
    let mut player_pos = (1, 9);
    for char in string.chars().skip(1) {
        match char.to_ascii_uppercase() {
            '\n' => {
                row += 1;
                col = 0;
                continue;
            }
            '_' => cells[row][col] = Cell::Empty,
            'O' => cells[row][col] = Cell::Obstacle,
            'X' => cells[row][col] = Cell::PasswordPiece,
            'P' => player_pos = (col as i32, row as i32),
            _ => continue,
        }
        print!("{char}");

        col += 1;
    }

    (cells, player_pos)
}

pub struct MiniGame {
    cells: [[Cell; NUM_OF_CELLS]; NUM_OF_CELLS],
    player_position: (i32, i32),
    player_movement: (i32, i32),
    last_update: Instant,
    new_movement: (i32, i32),
    keys_collected: u8,
    finish_reached: bool,
    top_left: Vec2,
    width: f32,
    height: f32,
}

impl MiniGame {
    pub fn new() -> Self {
        let (cells, player_position) = cells_from_string(MAP);
        MiniGame {
            cells,
            player_position,
            player_movement: (1, 0),
            last_update: Instant::now(),
            new_movement: (1, 0),
            keys_collected: 0,
            finish_reached: false,
            top_left: vec2(200.0, 180.0),
            width: CELL_SIZE * NUM_OF_CELLS as f32 + 5.0,
            height: CELL_SIZE * NUM_OF_CELLS as f32 + 5.0,
        }
    }

    pub fn restart(&mut self) {
        *self = Self::new();
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

    fn size(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }

    fn draw(&mut self) {
        let width = self.width;
        let height = self.height;
        draw_outlined_box(
            self.top_left.x - 2.5,
            self.top_left.y - 2.5,
            width,
            height,
            5.0,
            BG_COLOR,
            FG_COLOR,
        );

        let mut row_pos = self.top_left().y;
        for row in &self.cells {
            let mut col_pos = self.top_left.x;
            for cell in row {
                let color = match cell {
                    Cell::Obstacle => OBSTACLE_COLOR,
                    Cell::Empty => EMPTY_COLOR,
                    Cell::Player => PLAYER_COLOR,
                    Cell::PasswordPiece => PASSWORD_COLOR,
                    Cell::Finish => FINISH_COLOR,
                };
                draw_rectangle(col_pos, row_pos, CELL_SIZE, CELL_SIZE, color);

                col_pos += CELL_SIZE;
            }
            row_pos += CELL_SIZE;
        }
    }

    fn is_visible(&self) -> bool {
        true
    }

    fn set_visibility(&mut self, _value: bool) {}

    fn handle_input(&mut self, _event: crate::windows::InputEvent) -> WindowReturnAction {
        let input = if is_key_pressed(KeyCode::Left) {
            Some((-1, 0))
        } else if is_key_pressed(KeyCode::Right) {
            Some((1, 0))
        } else if is_key_pressed(KeyCode::Up) {
            Some((0, -1))
        } else if is_key_pressed(KeyCode::Down) {
            Some((0, 1))
        } else {
            None
        };
        if let Some(input) = input {
            self.new_movement = input;
        }

        self.cells[self.player_position.1 as usize][self.player_position.0 as usize] = Cell::Empty;
        if Instant::now().duration_since(self.last_update) > DURATION_BETWEEN_MOVES {
            self.player_movement = self.new_movement;
            self.player_position = (
                self.player_position.0 + self.new_movement.0,
                self.player_position.1 + self.new_movement.1,
            );
            self.last_update = Instant::now();

            // Check validity of position
            if self.player_position.0 < 0
                || self.player_position.0 >= NUM_OF_CELLS as i32
                || self.player_position.1 < 0
                || self.player_position.1 >= NUM_OF_CELLS as i32
            {
                self.restart();
                return WindowReturnAction::None;
            }

            // Check if obstacle
            if self.cells[self.player_position.1 as usize][self.player_position.0 as usize]
                == Cell::Obstacle
            {
                self.restart();
            }

            // Check if finish reached
            if self.cells[self.player_position.1 as usize][self.player_position.0 as usize]
                == Cell::Finish
            {
                return WindowReturnAction::HackCompleted;
            }

            if self.cells[self.player_position.1 as usize][self.player_position.0 as usize]
                == Cell::PasswordPiece
            {
                self.keys_collected += 1;
            }
        }
        self.cells[self.player_position.1 as usize][self.player_position.0 as usize] = Cell::Player;

        if self.keys_collected == KEYS_TOTAL {
            self.cells[10][10] = Cell::Finish;
        }

        WindowReturnAction::None
    }

    fn icon(&self) -> Option<Texture2D> {
        texture_storage().minigame()
    }

    fn contains_pos(&self, pos: Vec2) -> bool {
        false
    }
}
