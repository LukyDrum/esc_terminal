mod document;
mod document_list;
mod login;
mod minigame;
mod popup;
mod system;
mod windows;

use macroquad::prelude::*;
use macroquad::window::Conf as WindowConf;
use system::EscOS;

// Constants definition
// STYLE

fn window_conf() -> WindowConf {
    WindowConf {
        window_title: "ESC Terminal".to_string(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut esc_os = EscOS::new().await;

    loop {
        esc_os.tick().await;

        next_frame().await
    }
}
