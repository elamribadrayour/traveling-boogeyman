use rand::{Rng, RngCore};

use nannou::App;
use nannou_wgpu::Texture;

pub struct Boogeyman {
    position: usize,
    texture: Texture,
}

impl Boogeyman {
    pub fn new(rng: &mut dyn RngCore, app: &App, house_count: usize) -> Self {
        Self {
            position: rng.gen_range(0..house_count),
            texture: Texture::from_path(app, "assets/images/bogeyman.png").unwrap(),
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }
}
