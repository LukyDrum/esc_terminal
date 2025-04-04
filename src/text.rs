use macroquad::prelude::*;

use crate::{
    system::{BG_COLOR, FG_COLOR},
    windows::{draw_outlined_box, draw_window_top_bar, Window},
};

pub struct TextWindow {
    position: Vec2,
    size: Vec2,
    is_visible: bool,
}

impl Window for TextWindow {
    fn new_boxed() -> Box<dyn Window>
    where
        Self: Sized,
    {
        Box::new(TextWindow {
            position: Vec2::new(screen_width() * 0.5, screen_height() * 0.7),
            size: Vec2::new(500.0, 700.0),
            is_visible: true,
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
            40.0,
            FG_COLOR,
            BG_COLOR,
        );

        let char_size = 20.0;
        let count = (self.size.x - 2.0) / (char_size * 0.5);
        let mut line = "a".repeat(count as usize + 2);
        line.push('\n');
        draw_multiline_text(
            line.repeat(31).as_str(),
            self.top_left().x + 20.0,
            self.top_left().y + 70.0,
            char_size,
            None,
            FG_COLOR,
        );
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn set_visibility(&mut self, value: bool) {
        self.is_visible = value;
    }
}
