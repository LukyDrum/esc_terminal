use std::collections::{HashMap, LinkedList};
use std::path::PathBuf;
use std::process::{Child, Command};
use std::rc::Rc;
use std::time::Instant;
use std::{fs, mem};

use chrono::{Local, Timelike};
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::Button;

use crate::document::DocumentWindow;
use crate::document_list::DocumentList;
use crate::login::LoginWindow;
use crate::minigame::MiniGame;
use crate::popup::PopUp;
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
pub static mut TEXTURE_STORAGE: Option<Rc<TextureStorage>> = None;

const HACK_FILE_NAME: &str = "secret.hack";
const USB_PATH: &'static str = env!("ESC_USB_PATH");

pub struct TextureStorage {
    document_icon: Option<Texture2D>,
    minimize_icon: Option<Texture2D>,
    popup_icon: Option<Texture2D>,
    close_icon: Option<Texture2D>,
    minigame_icon: Option<Texture2D>,
    documents: HashMap<String, Texture2D>,
}

pub fn texture_storage() -> Rc<TextureStorage> {
    unsafe { TEXTURE_STORAGE.clone().unwrap() }
}

impl TextureStorage {
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

    pub fn minigame(&self) -> Option<Texture2D> {
        self.minigame_icon.clone()
    }

    pub fn document_by_name(&self, name: &str) -> Option<Texture2D> {
        self.documents.get(name).cloned()
    }

    pub fn fallback_document(&self) -> Texture2D {
        todo!()
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
    usb_path: PathBuf,
    usb_detected_at: Option<Instant>,
    hack_status: HackStatus,

    udiskie: Child,
}

async fn load_texture_storage() {
    // Get document names
    let names = fs::read_dir("./assets/documents/").expect("Failed to read directory entries.");
    // Load each texture
    let mut documents = HashMap::new();
    for future in names.flatten().map(async |entry| {
        let name = entry
            .file_name()
            .into_string()
            .expect("Failed to parse OsString to String.");
        let texture = load_texture(format!("assets/documents/{name}").as_str())
            .await
            .expect("Failed to load texture.");
        let name = name
            .split(".")
            .nth(0)
            .expect("Invalid filename.")
            .to_string();

        (name, texture)
    }) {
        let (name, texture) = future.await;

        // Print debug info
        println!("Load document: {name}");

        documents.insert(name, texture);
    }

    unsafe {
        TEXTURE_STORAGE = Some(Rc::new(TextureStorage {
            document_icon: load_texture("assets/document_icon.png").await.ok(),
            minimize_icon: load_texture("assets/minimize.png").await.ok(),
            popup_icon: load_texture("assets/warning.png").await.ok(),
            close_icon: load_texture("assets/close.png").await.ok(),
            minigame_icon: load_texture("assets/minigame.png").await.ok(),
            documents,
        }));
    }
}

impl EscOS {
    pub async fn new() -> Self {
        // Spawn udiskie for automounting
        let udiskie = Command::new("udiskie")
            .arg("-a")
            .spawn()
            .expect("Failed to spawn udiskie!");

        // Load Texture storage
        load_texture_storage().await;

        EscOS {
            logo_texture: load_texture("assets/logo.png").await.unwrap(),
            login_window: LoginWindow::new_boxed().await,
            windows: vec![],
            is_unlocked: false,

            hack_file_content: fs::read_to_string("assets/".to_string() + HACK_FILE_NAME).unwrap(),
            last_usb_check: Instant::now(),
            usb_path: PathBuf::from(USB_PATH),
            usb_detected_at: None,
            hack_status: HackStatus::NoUSB,

            udiskie,
        }
    }

    pub async fn tick(&mut self) {
        // Check hack file
        if self.check_hack_file() && self.hack_status == HackStatus::NoUSB {
            self.hack_status = HackStatus::USBOpened(Instant::now());
            self.windows.push(Box::new(PopUp::new_with_text(
                "Hack in progress!".to_string(),
            )));
        }

        if let HackStatus::USBOpened(instant) = self.hack_status {
            if Instant::now().duration_since(instant).as_secs() > 2 {
                self.windows.push(Box::new(MiniGame::new()));
                self.hack_status = HackStatus::Minigame;
            }
        }

        // DEBUG
        if is_key_pressed(KeyCode::Home) {
            self.on_hack_completed().await;
        }

        self.draw_background();

        let mut windows_to_close = LinkedList::new();

        let mouse_pos = vec2(mouse_position().0, mouse_position().1);
        let wheel_scroll = mouse_wheel().1;
        let mut event = {
            if is_mouse_button_pressed(MouseButton::Left) {
                InputEvent::LeftMouse(mouse_pos, false)
            } else if is_mouse_button_down(MouseButton::Left) {
                InputEvent::LeftMouse(mouse_pos, true)
            } else if wheel_scroll != 0.0 {
                InputEvent::Scroll(wheel_scroll)
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
                WindowReturnAction::HackCompleted => {
                    windows_to_close.push_front(0);
                    windows_to_close.push_front(index);
                    self.on_hack_completed().await;
                }
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

    async fn on_hack_completed(&mut self) {
        self.hack_status = HackStatus::Completed;
        self.is_unlocked = true;
        self.windows.push(Box::new(PopUp::new_with_text(
            "Hack completed!".to_string(),
        )));

        // Open document list
        self.windows.push(
            DocumentList::new_boxed().await
        );
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
        let maybe_content = fs::read_to_string(self.usb_path.join(HACK_FILE_NAME)).ok();
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
