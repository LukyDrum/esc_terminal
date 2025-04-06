use crate::{
    popup::PopUp,
    system::{BG_COLOR, FG_COLOR},
    windows::*,
};
use macroquad::{
    prelude::*,
    ui::{
        root_ui,
        widgets::{Button, InputText},
    },
};

pub struct LoginWindow {
    width: f32,
    height: f32,
    password_data: String,
    input_size: Vec2,
    is_visible: bool,
    login_button_clicked: bool,
}

impl Window for LoginWindow {
    async fn new_boxed() -> Box<dyn Window> {
        Box::new(LoginWindow {
            width: 500.0,
            height: 300.0,
            password_data: String::new(),
            input_size: Vec2::new(300.0, 60.0),
            is_visible: true,
            login_button_clicked: false,
        })
    }

    /// Alway in center
    fn position(&self) -> Vec2 {
        Vec2::new(screen_width() * 0.5, screen_height() * 0.5)
    }

    fn draw(&mut self) {
        // Draw box
        draw_outlined_box(
            self.position().x - self.width * 0.5,
            self.position().y - self.height * 0.5,
            self.width,
            self.height,
            5.0,
            BG_COLOR,
            FG_COLOR,
        );

        // Label
        draw_text(
            "Enter password",
            self.position().x - self.input_size.x * 0.5,
            self.position().y - self.input_size.y,
            50.0,
            FG_COLOR,
        );

        // Draw input
        let input_style = root_ui()
            .style_builder()
            .font_size(30)
            .color(BLANK)
            .color_hovered(BLANK)
            .color_clicked(BLANK)
            .build();
        let mut skin = root_ui().default_skin();
        skin.editbox_style = input_style;
        root_ui().push_skin(&skin);
        InputText::new(1)
            .position(self.position() - Vec2::new(self.input_size.x * 0.5, self.input_size.y * 0.4))
            .size(self.input_size)
            .ratio(5.0)
            .password(true)
            .ui(&mut root_ui(), &mut self.password_data);
        root_ui().pop_skin();

        let bigger_x = self.input_size.x * 1.1;
        let bigger_y = self.input_size.y * 1.1;
        draw_outlined_box(
            self.position().x - bigger_x * 0.5,
            self.position().y - bigger_y * 0.5,
            bigger_x,
            bigger_y,
            5.0,
            BG_COLOR,
            FG_COLOR,
        );

        let (button_width, button_height) = (100.0, 50.0);

        let button_style = root_ui()
            .style_builder()
            .color(BLANK)
            .color_hovered(BLANK)
            .text_color(BG_COLOR)
            .text_color_hovered(BG_COLOR)
            .build();
        let mut skin = root_ui().default_skin();
        skin.button_style = button_style;
        root_ui().push_skin(&skin);

        let position = self.position() + Vec2::new(-button_width * 0.5, self.height * 0.25);
        let (bigger_width, bigger_height) = (button_width * 1.05, button_height * 1.05);
        let box_position = self.position() + Vec2::new(-bigger_width * 0.5, self.height * 0.245);
        draw_outlined_box(
            box_position.x,
            box_position.y,
            bigger_width,
            bigger_height,
            5.0,
            FG_COLOR,
            FG_COLOR,
        );
        let button_size = Vec2::new(button_width, button_height);
        self.login_button_clicked = Button::new("Log-in")
            .position(position)
            .size(button_size)
            .ui(&mut root_ui());

        root_ui().pop_skin();
    }

    fn top_left(&self) -> Vec2 {
        self.position() - self.input_size * 0.5
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn set_visibility(&mut self, value: bool) {
        self.is_visible = value;
    }

    fn handle_input(&mut self, event: InputEvent) -> WindowReturnAction {
        match event {
            InputEvent::LeftMouse(_pos, _) => {
                if self.login_button_clicked {
                    WindowReturnAction::NewWindow(Box::new(PopUp::new_with_text(
                        "Error:\nLogin is disabled during emergency\nprotocol!".to_string(),
                    )))
                } else {
                    WindowReturnAction::None
                }
            }
            _ => WindowReturnAction::None,
        }
    }

    fn icon(&self) -> Option<Texture2D> {
        None
    }

    fn contains_pos(&self, pos: Vec2) -> bool {
        let tl = self.top_left();
        let br = self.top_left() + vec2(self.width, self.height);
        let (x, y) = (pos.x, pos.y);

        x >= tl.x && x <= br.x && y >= tl.y && y <= br.y
    }
}
