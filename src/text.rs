use macroquad::prelude::*;

use crate::{
    system::{BG_COLOR, FG_COLOR, LAST_MOUSE_POS, TEXTURE_STORAGE},
    windows::{
        draw_outlined_box, draw_window_top_bar, minimize_button, InputEvent, Window,
        WindowReturnAction,
    },
};

const HEADER_HEIGHT: f32 = 70.0;

pub struct TextWindow {
    position: Vec2,
    size: Vec2,
    is_visible: bool,
    /// Relative to top-left
    minimize_position_relative: Vec2,
    minimize_size: Vec2,
}

impl Window for TextWindow {
    async fn new_boxed() -> Box<dyn Window>
    where
        Self: Sized,
    {
        Box::new(TextWindow {
            position: Vec2::new(screen_width() * 0.5, screen_height() * 0.7),
            size: Vec2::new(500.0, 700.0),
            is_visible: true,
            minimize_position_relative: Vec2::new(460.0, HEADER_HEIGHT * 0.5),
            minimize_size: Vec2::ZERO,
        })
    }

    fn position(&self) -> Vec2 {
        self.position
    }

    fn top_left(&self) -> Vec2 {
        self.position - self.size * 0.5
    }

    fn draw(&mut self) {
        // Draw outer box
        draw_outlined_box(
            self.top_left().x,
            self.top_left().y,
            self.size.x,
            self.size.y,
            5.0,
            BG_COLOR,
            FG_COLOR,
        );
        draw_window_top_bar(
            "Document preview",
            30.0,
            self.top_left().x,
            self.top_left().y,
            self.size.x,
            HEADER_HEIGHT,
            FG_COLOR,
            BG_COLOR,
        );

        let char_size = 20.0;
        let count = (self.size.x - 2.0) / (char_size * 0.5);
        let mut line = "a".repeat(count as usize + 2);
        line.push('\n');
        draw_multiline_text(
            line.repeat(30).as_str(),
            self.top_left().x + 20.0,
            self.top_left().y + HEADER_HEIGHT + 25.0,
            char_size,
            None,
            FG_COLOR,
        );

        self.minimize_size = minimize_button(self.top_left() + self.minimize_position_relative);
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn set_visibility(&mut self, value: bool) {
        self.is_visible = value;
    }

    fn handle_input(&mut self, event: InputEvent) -> WindowReturnAction {
        // Check if mouse is pressed in header part
        match event {
            InputEvent::LeftMouse(pos, held) => {
                if is_mouse_button_down(MouseButton::Left) && self.is_pos_in_header(pos) {
                    let diff = unsafe { pos - LAST_MOUSE_POS };
                    self.position += diff;
                }

                if self.is_pos_in_minimize_button(pos) && !held {
                    WindowReturnAction::Minimize
                } else {
                    WindowReturnAction::None
                }
            }
            _ => WindowReturnAction::None,
        }
    }

    fn icon(&self) -> Option<Texture2D> {
        unsafe { TEXTURE_STORAGE.document() }
    }

    fn contains_pos(&self, pos: Vec2) -> bool {
        let tl = self.top_left();
        let br = self.top_left() + self.size;
        let (x, y) = (pos.x, pos.y);

        x >= tl.x && x <= br.x && y >= tl.y && y <= br.y
    }
}

impl TextWindow {
    fn is_pos_in_header(&self, pos: Vec2) -> bool {
        pos.x > self.top_left().x
            && pos.x < self.top_left().x + self.size.x
            && pos.y > self.top_left().y
            && pos.y < self.top_left().y + HEADER_HEIGHT
    }

    fn is_pos_in_minimize_button(&self, pos: Vec2) -> bool {
        let min_tl = self.top_left() + self.minimize_position_relative - self.minimize_size * 0.5;
        let min_br = self.top_left() + self.minimize_position_relative + self.minimize_size * 0.5;
        pos.x > min_tl.x && pos.x < min_br.x && pos.y > min_tl.y && pos.y < min_br.y
    }
}
