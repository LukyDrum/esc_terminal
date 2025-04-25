use macroquad::prelude::*;

use crate::system::{texture_storage, BG_COLOR, TEXTURE_STORAGE};

pub trait Window {
    async fn new_boxed() -> Box<dyn Window>
    where
        Self: Sized;

    fn position(&self) -> Vec2;

    fn top_left(&self) -> Vec2;

    fn draw(&mut self);

    fn is_visible(&self) -> bool;

    fn set_visibility(&mut self, value: bool);

    fn handle_input(&mut self, event: InputEvent) -> WindowReturnAction;

    fn icon(&self) -> Option<Texture2D>;

    fn contains_pos(&self, pos: Vec2) -> bool;
}

pub enum WindowReturnAction {
    None,
    Minimize,
    Close,
    NewWindow(Box<dyn Window>),
    HackCompleted,
}

#[derive(Copy, Clone)]
pub enum InputEvent {
    None,
    /// Position and if it is being held
    LeftMouse(Vec2, bool),
    Scroll(f32),
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

pub fn minimize_button(position: Vec2) -> Vec2 {
    let texture = unsafe { texture_storage().minimize().unwrap() };
    let size = texture.size();
    draw_texture(
        &texture,
        position.x - size.x * 0.5,
        position.y - size.y * 0.5,
        BG_COLOR,
    );

    size
}
