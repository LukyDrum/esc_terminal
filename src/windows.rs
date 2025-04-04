use macroquad::prelude::*;

pub trait Window {
    fn new_boxed() -> Box<dyn Window>
    where
        Self: Sized;

    fn position(&self) -> Vec2;

    fn top_left(&self) -> Vec2;

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

pub fn draw_window_top_bar(
    text: &str,
    font_size: f32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    fg_color: Color,
    bg_color: Color,
) {
    draw_outlined_box(x, y, width, height, 5.0, bg_color, fg_color);
    let measure = measure_text(text, None, font_size as u16, 1.0);
    draw_text(
        text,
        x + width * 0.5 - measure.width * 0.5,
        y + height * 0.5 + measure.height * 0.5,
        font_size,
        fg_color,
    );
}
