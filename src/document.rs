use macroquad::prelude::*;

use crate::{
    system::{texture_storage, BG_COLOR, FG_COLOR, LAST_MOUSE_POS},
    windows::{
        draw_outlined_box, draw_window_top_bar, minimize_button, InputEvent, Window,
        WindowReturnAction,
    },
};

const HEADER_HEIGHT: f32 = 70.0;
const FIXED_DOC_HEIGHT: f32 = 1000.0;
const SCROLL_SPEED: f32 = 35.0;

pub struct VerticalScroller {
    pub height: f32,
    pub scroller_height: f32,
    /// 0 to 1
    pub percent: f32,
}

impl VerticalScroller {
    pub const WIDTH: f32 = 10.0;
    const BG_COLOR: Color = Color::from_hex(0xA0A0A0);

    pub fn draw(&self, top_left: Vec2) {
        draw_rectangle(
            top_left.x,
            top_left.y,
            Self::WIDTH,
            self.height,
            Self::BG_COLOR,
        );

        let y = top_left.y + self.percent * (self.height - self.scroller_height);
        draw_rectangle(top_left.x, y, Self::WIDTH, self.scroller_height, FG_COLOR);
    }
}

pub struct DocumentWindow {
    position: Vec2,
    window_size: Vec2,
    document_name: String,
    document_texture: Texture2D,
    vertical_offset: f32,
    max_vertical_offset: f32,
    document_height: f32,
    scroller: VerticalScroller,

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
            self.window_size.y + 2.5,
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

        let params = DrawTextureParams {
            source: Some(Rect {
                x: 0.0,
                y: self.vertical_offset,
                w: self.document_texture.width(),
                h: self.window_size.y - HEADER_HEIGHT,
            }),
            ..Default::default()
        };

        draw_texture_ex(
            &self.document_texture,
            self.top_left().x + 2.5,
            self.top_left().y + HEADER_HEIGHT,
            WHITE,
            params,
        );

        let scroller_pos = self.position
            + vec2(
                self.window_size.x * 0.5 - VerticalScroller::WIDTH - 2.5,
                -self.window_size.y * 0.5 + HEADER_HEIGHT,
            );
        self.scroller.draw(scroller_pos);

        self.minimize_size = minimize_button(self.top_left() + self.minimize_position_relative);
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn set_visibility(&mut self, value: bool) {
        self.is_visible = value;
    }

    fn handle_input(&mut self, event: InputEvent) -> WindowReturnAction {
        self.scroller.percent = self.vertical_offset / self.max_vertical_offset;

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
            InputEvent::Scroll(scroll) => {
                self.vertical_offset = (self.vertical_offset - scroll * SCROLL_SPEED)
                    .clamp(0.0, self.max_vertical_offset);
                WindowReturnAction::None
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

impl DocumentWindow {
    pub fn new_boxed(document_name: String) -> Box<dyn Window> {
        let document_texture = texture_storage().document_by_name(document_name.as_str());
        let document_texture = if let Some(texture) = document_texture {
            texture
        } else {
            texture_storage().fallback_document()
        };

        let width = document_texture.width();
        let max_vertical_offset = document_texture.height() - FIXED_DOC_HEIGHT + 2. * HEADER_HEIGHT;

        let position = Vec2::new(200.0 + screen_width() * 0.5, 50.0 + FIXED_DOC_HEIGHT * 0.5);
        let scroller = VerticalScroller {
            height: FIXED_DOC_HEIGHT - 2.0 * HEADER_HEIGHT + 5.0,
            scroller_height: 50.0,
            percent: 0.0,
        };

        Box::new(DocumentWindow {
            position,
            window_size: Vec2::new(width + 5.0, FIXED_DOC_HEIGHT - HEADER_HEIGHT + 5.0),
            document_name,
            document_height: document_texture.height(),
            document_texture,
            vertical_offset: 0.0,
            max_vertical_offset,
            scroller,

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
