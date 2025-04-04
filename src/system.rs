use macroquad::prelude::*;


const BG_COLOR: Color = WHITE;

pub struct EscOS {
    logo_texture: Texture2D,
}

impl EscOS {
    pub async fn new() -> Self {
        EscOS {
            logo_texture: load_texture("assets/logo.png").await.unwrap()
        }
    }

    pub fn draw_background(&self) {
        clear_background(BG_COLOR);

        // Draw logo
        let x = screen_width() * 0.5 - self.logo_texture.width() * 0.5;
        let y = screen_height() * 0.5 - self.logo_texture.height() * 0.5;

        draw_texture(&self.logo_texture, x, y, BG_COLOR);
    }
}
