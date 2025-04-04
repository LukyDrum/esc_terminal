use macroquad::prelude::*;

pub trait Window {
    fn new_boxed() -> Box<dyn Window>
    where
        Self: Sized;

    fn position(&self) -> Vec2;

    fn draw(&mut self);
}

pub fn draw_outlined_box(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    thickness: f32,
    background_color: Color,
    outline_color: Color,
) {
    draw_rectangle(x, y, width, height, background_color);
    draw_rectangle_lines(x, y, width, height, thickness, outline_color);
}
