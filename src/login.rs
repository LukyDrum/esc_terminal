use crate::windows::*;
use macroquad::{
    prelude::*,
    ui::{root_ui, widgets::InputText},
};

pub struct LoginWindow {
    width: f32,
    height: f32,
    password_data: String,
    input_size: Vec2,
}

impl Window for LoginWindow {
    fn new_boxed() -> Box<dyn Window> {
        Box::new(LoginWindow {
            width: 500.0,
            height: 300.0,
            password_data: String::new(),
            input_size: Vec2::new(300.0, 70.0),
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
            WHITE,
            BLACK,
        );

        // Label
        draw_text(
            "Enter password",
            self.position().x - self.input_size.x * 0.5,
            self.position().y - self.input_size.y,
            50.0,
            BLACK,
        );

        // Draw input
        InputText::new(1)
            .position(self.position() - self.input_size * 0.5)
            .size(self.input_size)
            .ratio(5.0)
            .ui(&mut root_ui(), &mut self.password_data);
        let bigger_x = self.input_size.x * 1.1;
        let bigger_y = self.input_size.y * 1.1;
        draw_outlined_box(
            self.position().x - bigger_x * 0.5,
            self.position().y - bigger_y * 0.5,
            bigger_x,
            bigger_y,
            5.0,
            WHITE,
            BLACK,
        );
    }
}
