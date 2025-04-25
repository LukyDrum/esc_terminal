use macroquad::prelude::*;

use crate::{
    system::{texture_storage, BG_COLOR, FG_COLOR, LAST_MOUSE_POS},
    windows::{
        draw_outlined_box, draw_window_top_bar, minimize_button, InputEvent, Window,
        WindowReturnAction,
    },
};

const HEADER_HEIGHT: f32 = 70.0;
const FIXED_DOC_HEIGHT: f32 = 800.0;

pub struct DocumentWindow {
    position: Vec2,
    window_size: Vec2,
    document_name: String,
    document_texture: Texture2D,

    is_visible: bool,
    /// Relative to top-left
    minimize_position_relative: Vec2,
    minimize_size: Vec2,
}

impl Window for DocumentWindow {
    async fn new_boxed() -> Box<dyn Window>
    where
        Self: Sized,
    {
        panic!("PLS don't call this function.")
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
            self.window_size.y,
            5.0,
            BG_COLOR,
            FG_COLOR,
        );
        draw_window_top_bar(
            &self.document_name,
            30.0,
            self.top_left().x,
            self.top_left().y,
            self.window_size.x,
            HEADER_HEIGHT,
            FG_COLOR,
            BG_COLOR,
        );

        draw_texture_ex(
            &self.document_texture,
            self.top_left().x,
            self.top_left().y + HEADER_HEIGHT,
            WHITE,
            DrawTextureParams::default(),
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
        unsafe { texture_storage().document() }
    }

    fn contains_pos(&self, pos: Vec2) -> bool {
        let tl = self.top_left();
        let br = self.top_left() + self.window_size;
        let (x, y) = (pos.x, pos.y);

        x >= tl.x && x <= br.x && y >= tl.y && y <= br.y
    }
}

impl DocumentWindow {
    pub fn new_boxed(document_name: String) -> Box<dyn Window> {
        let document_texture = texture_storage().document_by_name(document_name.as_str());
        let document_texture = if let Some(texture) = document_texture {
            texture
        } else {
            texture_storage().fallback_document()
        };

        let width = document_texture.width();

        Box::new(DocumentWindow {
            position: Vec2::new(screen_width() * 0.5, screen_height() * 0.7),
            window_size: Vec2::new(width, FIXED_DOC_HEIGHT - HEADER_HEIGHT),
            document_name,
            document_texture,

            is_visible: true,
            minimize_position_relative: Vec2::new(width - 50.0, HEADER_HEIGHT * 0.5),
            minimize_size: Vec2::ZERO,
        })
    }

    fn is_pos_in_header(&self, pos: Vec2) -> bool {
        pos.x > self.top_left().x
            && pos.x < self.top_left().x + self.window_size.x
            && pos.y > self.top_left().y
            && pos.y < self.top_left().y + HEADER_HEIGHT
    }

    fn is_pos_in_minimize_button(&self, pos: Vec2) -> bool {
        let min_tl = self.top_left() + self.minimize_position_relative - self.minimize_size * 0.5;
        let min_br = self.top_left() + self.minimize_position_relative + self.minimize_size * 0.5;
        pos.x > min_tl.x && pos.x < min_br.x && pos.y > min_tl.y && pos.y < min_br.y
    }
}
