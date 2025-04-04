mod screens;
mod system;

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
    let esc_os = EscOS::new().await;

    loop {
        esc_os.draw_background();
        esc_os.draw_top_bar();

        next_frame().await
    }
}
