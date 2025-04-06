use std::collections::LinkedList;
use std::process::{Child, Command};
use std::time::Instant;
use std::{fs, mem};

use chrono::{Local, Timelike};
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::Button;

use crate::login::LoginWindow;
use crate::popup::PopUp;
use crate::text::TextWindow;
use crate::windows::{draw_outlined_box, InputEvent, Window, WindowReturnAction};

pub const BG_COLOR: Color = WHITE;
pub const FG_COLOR: Color = BLACK;

const BAR_COLOR: Color = FG_COLOR;
const BAR_TEXT_COLOR: Color = BG_COLOR;

const TOP_BAR_HEIGHT: f32 = 50.0;
const BAR_FONT_SIZE: (u16, f32) = (1, 40.0);

const DOCK_ICON_SIZE: u32 = 64;
const DOCK_SPACING: u32 = 16;

pub static mut LAST_MOUSE_POS: Vec2 = Vec2::new(0.0, 0.0);
pub static mut TEXTURE_STORAGE: TextureStorage = TextureStorage::empty();

const HACK_FILE_NAME: &str = "secret.hack";
const USB_NAME: &str = "HACKY";

pub struct TextureStorage {
    document_icon: Option<Texture2D>,
    minimize_icon: Option<Texture2D>,
    popup_icon: Option<Texture2D>,
    close_icon: Option<Texture2D>,
}

impl TextureStorage {
    const fn empty() -> Self {
        TextureStorage {
            document_icon: None,
            minimize_icon: None,
            popup_icon: None,
            close_icon: None,
        }
    }

    pub fn document(&self) -> Option<Texture2D> {
        self.document_icon.clone()
    }

    pub fn minimize(&self) -> Option<Texture2D> {
        self.minimize_icon.clone()
    }

    pub fn popup(&self) -> Option<Texture2D> {
        self.popup_icon.clone()
    }

    pub fn close(&self) -> Option<Texture2D> {
        self.close_icon.clone()
    }
}

#[derive(Clone, Copy, PartialEq)]
enum HackStatus {
    NoUSB,
    USBOpened(Instant),
    Minigame,
    Completed,
}

pub struct EscOS {
    logo_texture: Texture2D,
    login_window: Box<dyn Window>,
    windows: Vec<Box<dyn Window>>,
    is_unlocked: bool,

    hack_file_content: String,
    last_usb_check: Instant,
    usb_path: String,
    usb_detected_at: Option<Instant>,
    hack_status: HackStatus,

    udiskie: Child,
}

impl EscOS {
    pub async fn new() -> Self {
        // Spawn udiskie for automounting
        let udiskie = Command::new("udiskie")
            .arg("-a")
            .spawn()
            .expect("Failed to spawn udiskie!");

        // Load Texture storage
        unsafe {
            TEXTURE_STORAGE = TextureStorage {
                document_icon: load_texture("assets/document_icon.png").await.ok(),
                minimize_icon: load_texture("assets/minimize.png").await.ok(),
                popup_icon: load_texture("assets/warning.png").await.ok(),
                close_icon: load_texture("assets/close.png").await.ok(),
            };
        }

        let user = whoami::username();
        EscOS {
            logo_texture: load_texture("assets/logo.png").await.unwrap(),
            login_window: LoginWindow::new_boxed().await,
            windows: vec![TextWindow::new_boxed().await],
            is_unlocked: false,

            hack_file_content: fs::read_to_string("assets/".to_string() + HACK_FILE_NAME).unwrap(),
            last_usb_check: Instant::now(),
            usb_path: format!("/run/media/{user}/{USB_NAME}/"),
            usb_detected_at: None,
            hack_status: HackStatus::NoUSB,

            udiskie,
        }
    }

    pub fn tick(&mut self) {
        // Check hack file
        if self.check_hack_file() && self.hack_status == HackStatus::NoUSB {
            self.hack_status = HackStatus::USBOpened(Instant::now());
            self.windows.push(Box::new(PopUp::new_with_text(
                "Hack in progress!".to_string(),
            )));
        }

        if let HackStatus::USBOpened(instant) = self.hack_status {
            if Instant::now().duration_since(instant).as_secs() > 2 {

            }
        }

        self.draw_background();

        let mut windows_to_close = LinkedList::new();

        let mouse_pos = vec2(mouse_position().0, mouse_position().1);
        let mut event = {
            if is_mouse_button_pressed(MouseButton::Left) {
                InputEvent::LeftMouse(mouse_pos, false)
            } else if is_mouse_button_down(MouseButton::Left) {
                InputEvent::LeftMouse(mouse_pos, true)
            } else {
                InputEvent::None
            }
        };

        for index in (0..self.windows.len()).rev() {
            if !self.windows[index].is_visible() {
                continue;
            }

            let this_event = if self.windows[index].contains_pos(mouse_pos) {
                mem::replace(&mut event, InputEvent::None)
            } else {
                InputEvent::None
            };
            match self.windows[index].handle_input(this_event) {
                WindowReturnAction::None => {}
                WindowReturnAction::Minimize => self.windows[index].set_visibility(false),
                WindowReturnAction::Close => windows_to_close.push_front(index),
                WindowReturnAction::NewWindow(new_win) => self.windows.push(new_win),
            }
        }
        for index in windows_to_close {
            self.windows.remove(index);
        }

        // If the system is locked, draw only login window and not dock
        if !self.is_unlocked {
            let this_event = if self.login_window.contains_pos(mouse_pos) {
                mem::replace(&mut event, InputEvent::None)
            } else {
                InputEvent::None
            };
            match self.login_window.handle_input(this_event) {
                WindowReturnAction::NewWindow(new_win) => self.windows.push(new_win),
                _ => {}
            }
            self.login_window.draw();
        }

        for win in &mut self.windows {
            if win.is_visible() {
                win.draw();
            }
        }

        self.draw_dock();
        self.draw_top_bar();

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
            .map(|(index, win)| (win.icon(), index))
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

    fn check_hack_file(&mut self) -> bool {
        let diff = Instant::now().duration_since(self.last_usb_check);
        if diff.as_secs_f32() < 1.0 {
            return false;
        }

        self.last_usb_check = Instant::now();
        let maybe_content = fs::read_to_string(self.usb_path.clone() + HACK_FILE_NAME).ok();
        match maybe_content {
            Some(content) => content == self.hack_file_content,
            None => false,
        }
    }
}

impl Drop for EscOS {
    fn drop(&mut self) {
        self.udiskie.kill().expect("Failed to kill udiskie!");
    }
}
