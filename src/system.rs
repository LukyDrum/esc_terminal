use std::collections::LinkedList;

use chrono::{Local, Timelike};
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::Button;

use crate::login::LoginWindow;
use crate::text::TextWindow;
use crate::windows::{draw_outlined_box, Window};

pub const BG_COLOR: Color = WHITE;
pub const FG_COLOR: Color = BLACK;

const BAR_COLOR: Color = FG_COLOR;
const BAR_TEXT_COLOR: Color = BG_COLOR;

const TOP_BAR_HEIGHT: f32 = 50.0;
const BAR_FONT_SIZE: (u16, f32) = (1, 40.0);

const DOCK_ICON_SIZE: u32 = 64;
const DOCK_SPACING: u32 = 16;

pub static mut LAST_MOUSE_POS: Vec2 = Vec2::new(0.0, 0.0);

pub struct EscOS {
    logo_texture: Texture2D,
    windows: Vec<Box<dyn Window>>,
}

impl EscOS {
    pub async fn new() -> Self {
        EscOS {
            logo_texture: load_texture("assets/logo.png").await.unwrap(),
            windows: vec![
                LoginWindow::new_boxed().await,
                TextWindow::new_boxed().await,
            ],
        }
    }

    pub fn tick(&mut self) {
        self.draw_background();

        for win in &mut self.windows {
            win.handle_input();
        }

        for win in &mut self.windows {
            if win.is_visible() {
                win.draw();
            }
        }

        self.draw_top_bar();
        self.draw_dock();

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

    fn draw_dock(&mut self) {
        let icons = self
            .windows
            .iter()
            .enumerate()
            .map(|(index, win)| (win.icon().cloned(), index))
            .filter_map(|(opt, index)| match opt {
                Some(texture) => Some((texture, index)),
                None => None,
            })
            .collect::<LinkedList<_>>();

        if icons.is_empty() {
            return;
        }

        let width = (icons.len() as u32 * (DOCK_ICON_SIZE + DOCK_SPACING)) as f32;
        let height = (DOCK_ICON_SIZE + DOCK_SPACING) as f32;
        let left = screen_width() * 0.5 - width * 0.5;
        let top = screen_height() - (height + DOCK_SPACING as f32);
        draw_outlined_box(left, top, width, height, 5.0, BG_COLOR, FG_COLOR);

        // Transparent button style
        let darker = Color::from_rgba(0, 0, 0, 50);
        let button_style = root_ui()
            .style_builder()
            .color(BLANK)
            .color_hovered(darker)
            .color_clicked(darker)
            .text_color(BLANK)
            .text_color_hovered(BLANK)
            .build();
        let mut skin = root_ui().default_skin();
        skin.button_style = button_style;
        root_ui().push_skin(&skin);
        let mut x = left;
        for (icon, win_index) in icons {
            draw_texture(
                &icon,
                x + DOCK_SPACING as f32 * 0.5,
                top + DOCK_SPACING as f32 * 0.5,
                BG_COLOR,
            );

            if self.windows[win_index].is_visible() {
                draw_circle(
                    x + (DOCK_ICON_SIZE + DOCK_SPACING) as f32 * 0.5,
                    screen_height() - 8.0,
                    4.0,
                    FG_COLOR,
                );
            }

            // Add a transparent button
            if Button::new("")
                .size(Vec2::new(
                    (DOCK_ICON_SIZE + DOCK_SPACING) as f32,
                    (DOCK_ICON_SIZE + DOCK_SPACING) as f32,
                ))
                .position(Vec2::new(x, top))
                .ui(&mut root_ui())
            {
                let is_visible = self.windows[win_index].is_visible();
                self.windows[win_index].set_visibility(!is_visible);
            }

            x += (DOCK_ICON_SIZE + DOCK_SPACING) as f32;
        }

        root_ui().pop_skin();
    }
}
