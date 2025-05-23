use macroquad::{
    prelude::*,
    ui::{root_ui, widgets::Button},
};

use crate::{
    system::{texture_storage, BG_COLOR, FG_COLOR, TEXTURE_STORAGE},
    windows::{draw_outlined_box, InputEvent, Window, WindowReturnAction},
};

const WIDTH: f32 = 700.0;
const HEIGHT: f32 = 200.0;

pub struct PopUp {
    position: Vec2,
    text: String,
    close_button_clicked: bool,
}

impl PopUp {
    pub fn new_with_text(text: String) -> Self {
        PopUp {
            position: vec2(
                screen_width() * 0.5 - WIDTH * 0.5,
                screen_height() * 0.5 - HEIGHT * 0.5,
            ),
            text,
            close_button_clicked: false,
        }
    }
}

impl Window for PopUp {
    async fn new_boxed() -> Box<dyn Window>
    where
        Self: Sized,
    {
        Box::new(PopUp {
            position: vec2(
                screen_width() * 0.5 - WIDTH * 0.5,
                screen_height() * 0.5 - HEIGHT * 0.5,
            ),
            text: "WARNING".to_string(),
            close_button_clicked: false,
        })
    }

    fn position(&self) -> Vec2 {
        self.position
    }

    fn top_left(&self) -> Vec2 {
        vec2(
            screen_width() * 0.5 - WIDTH * 0.5,
            screen_height() * 0.5 - HEIGHT * 0.5,
        )
    }

    fn size(&self) -> Vec2 {
        Vec2::new(WIDTH, HEIGHT)
    }

    fn draw(&mut self) {
        let tl = self.top_left();

        draw_outlined_box(tl.x, tl.y, WIDTH, HEIGHT, 5.0, BG_COLOR, FG_COLOR);

        let dim = measure_text(self.text.as_str(), None, 1, 40.0);
        draw_multiline_text(
            self.text.as_str(),
            self.position.x + 50.0,
            self.position.y + 1.8 * dim.offset_y,
            40.0,
            None,
            FG_COLOR,
        );

        // Draw close button
        let icon = texture_storage().close().unwrap();
        let button_style = root_ui()
            .style_builder()
            .color(BLANK)
            .color_hovered(BLANK)
            .color_clicked(BLANK)
            .text_color(BLANK)
            .text_color_hovered(BLANK)
            .build();
        let mut skin = root_ui().default_skin();
        skin.button_style = button_style;
        root_ui().push_skin(&skin);

        let pos = self.top_left() + vec2(WIDTH - 50.0, 18.0);
        draw_texture(&icon, pos.x, pos.y, BG_COLOR);
        self.close_button_clicked = Button::new("")
            .size(Vec2::new(64.0, 64.0))
            .position(pos)
            .ui(&mut root_ui());

        root_ui().pop_skin();
    }

    fn is_visible(&self) -> bool {
        true
    }

    fn set_visibility(&mut self, _value: bool) {}

    fn handle_input(&mut self, event: InputEvent) -> WindowReturnAction {
        match event {
            InputEvent::LeftMouse(_pos, _) => {
                if self.close_button_clicked {
                    WindowReturnAction::Close
                } else {
                    WindowReturnAction::None
                }
            }
            _ => WindowReturnAction::None,
        }
    }

    fn icon(&self) -> Option<Texture2D> {
        texture_storage().popup()
    }

    fn contains_pos(&self, pos: Vec2) -> bool {
        let tl = self.top_left();
        let br = self.top_left() + vec2(WIDTH, HEIGHT);
        let (x, y) = (pos.x, pos.y);

        x >= tl.x && x <= br.x && y >= tl.y && y <= br.y
    }
}
