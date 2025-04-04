use chrono::{Local, Timelike};
use macroquad::prelude::*;
use macroquad::ui::{root_ui, Style};

use crate::login::LoginWindow;
use crate::text::TextWindow;
use crate::windows::Window;

pub const BG_COLOR: Color = WHITE;
pub const FG_COLOR: Color = BLACK;

const BAR_COLOR: Color = FG_COLOR;
const BAR_TEXT_COLOR: Color = BG_COLOR;

const TOP_BAR_HEIGHT: f32 = 50.0;
const BAR_FONT_SIZE: (u16, f32) = (1, 40.0);

pub static mut LAST_MOUSE_POS: Vec2 = Vec2::new(0.0, 0.0);

pub struct EscOS {
    logo_texture: Texture2D,
    windows: Vec<Box<dyn Window>>,
}

impl EscOS {
    pub async fn new() -> Self {
        EscOS {
            logo_texture: load_texture("assets/logo.png").await.unwrap(),
            windows: vec![LoginWindow::new_boxed(), TextWindow::new_boxed()],
        }
    }

    pub fn tick(&mut self) {
        self.draw_background();
        self.draw_top_bar();

        for win in &mut self.windows {
            win.handle_input();
        }

        for win in &mut self.windows {
            if win.is_visible() {
                win.draw();
            }
        }

        unsafe {
            let pos = mouse_position();
            LAST_MOUSE_POS = Vec2::new(pos.0, pos.1);
        }
    }

    fn draw_background(&self) {
        clear_background(BG_COLOR);

        // Draw logo
        let x = screen_width() * 0.5 - self.logo_texture.width() * 0.5;
        let y = screen_height() * 0.5 - self.logo_texture.height() * 0.5;
        draw_texture(&self.logo_texture, x, y, BG_COLOR);
    }

    fn draw_top_bar(&self) {
        draw_rectangle(0.0, 0.0, screen_width(), TOP_BAR_HEIGHT, BAR_COLOR);
        let cur_time = Local::now();
        let time_text = format!(
            "{:<02}:{:<02}:{:<02}",
            cur_time.hour(),
            cur_time.minute(),
            cur_time.second()
        );
        let dim = measure_text(time_text.as_str(), None, BAR_FONT_SIZE.0, BAR_FONT_SIZE.1);
        draw_text(
            time_text.as_str(),
            screen_width() * 0.5 - dim.width * 0.5,
            TOP_BAR_HEIGHT * 0.5 - dim.height * 0.5 + dim.offset_y * 0.75,
            BAR_FONT_SIZE.1,
            BAR_TEXT_COLOR,
        );
    }
}
