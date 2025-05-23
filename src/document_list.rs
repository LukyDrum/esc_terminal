use macroquad::{
    color::{Color, WHITE}, input::{is_mouse_button_down, MouseButton}, math::{vec2, Vec2}, texture::{draw_texture, get_screen_data, Texture2D}, window::{screen_height, screen_width}
};

use crate::{
    system::{texture_storage, BG_COLOR, FG_COLOR, LAST_MOUSE_POS},
    windows::{
        draw_outlined_box, draw_window_top_bar, minimize_button, InputEvent, Window,
        WindowReturnAction, HEADER_HEIGHT,
    },
};

const WIDTH: f32 = 1000.0;
const HEIGHT: f32 = 500.0;

pub struct DocumentList {
    position: Vec2,
    window_size: Vec2,
    is_visible: bool,
    /// Relative to top-left
    minimize_position_relative: Vec2,
    minimize_size: Vec2,
    texture: Texture2D,
}

impl Window for DocumentList {
    async fn new_boxed() -> Box<dyn Window>
    where
        Self: Sized,
    {
        let texture = Texture2D::empty();
        let list = DocumentList {
            position: vec2(screen_width() * 0.5, screen_height() * 0.4),
            window_size: vec2(1000.0, 500.0),
            is_visible: true,
            minimize_position_relative: vec2(WIDTH - 50.0, HEADER_HEIGHT * 0.5),
            minimize_size: Vec2::ZERO,
            texture,
        };

        Box::new(list)
    }

    fn position(&self) -> Vec2 {
        self.position
    }

    fn top_left(&self) -> Vec2 {
        self.position - self.window_size * 0.5
    }

    fn draw(&mut self) {
        // Draw outer box
        draw_outlined_box(
            self.top_left().x,
            self.top_left().y,
            self.window_size.x,
            self.window_size.y + 2.5,
            5.0,
            BG_COLOR,
            FG_COLOR,
        );
        draw_window_top_bar(
            "Document List",
            30.0,
            self.top_left().x,
            self.top_left().y,
            self.window_size.x,
            HEADER_HEIGHT,
            FG_COLOR,
            BG_COLOR,
        );

        let Vec2{ x, y} = self.top_left();
        draw_texture(&self.texture, x, y, WHITE);

        self.minimize_size = minimize_button(self.top_left() + self.minimize_position_relative);
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn set_visibility(&mut self, value: bool) {
        self.is_visible = value;
    }

    fn size(&self) -> Vec2 {
        self.window_size
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
        texture_storage().document()
    }

    fn contains_pos(&self, pos: Vec2) -> bool {
        let tl = self.top_left();
        let br = self.top_left() + self.window_size;
        let (x, y) = (pos.x, pos.y);

        x >= tl.x && x <= br.x && y >= tl.y && y <= br.y
    }
}

impl DocumentList {
    fn is_pos_in_minimize_button(&self, pos: Vec2) -> bool {
        let min_tl = self.top_left() + self.minimize_position_relative - self.minimize_size * 0.5;
        let min_br = self.top_left() + self.minimize_position_relative + self.minimize_size * 0.5;
        pos.x > min_tl.x && pos.x < min_br.x && pos.y > min_tl.y && pos.y < min_br.y
    }
}
