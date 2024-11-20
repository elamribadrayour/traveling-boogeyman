use rand::{Rng, RngCore};

use nannou::geom::Vec2;
use nannou::App;
use nannou_wgpu::Texture;

pub struct House {
    position: Vec2,
    texture: Texture,
}

impl House {
    pub fn new(rng: &mut dyn RngCore, app: &App) -> Self {
        Self {
            position: Vec2::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)),
            texture: Texture::from_path(app, "assets/images/haunted-house.png").unwrap(),
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }
}
