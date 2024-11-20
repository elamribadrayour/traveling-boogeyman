use crate::boogeyman::Boogeyman;
use crate::houses::Houses;

use nannou::draw::Draw;
use nannou::geom::Rect;

use crate::drawers::houses_drawer::HousesDrawer;

pub struct BoogeymanDrawer;

impl BoogeymanDrawer {
    pub fn draw(
        draw: &Draw,
        window_rect: Rect,
        texture_size: f32,
        boogeyman: &Boogeyman,
        houses: &Houses,
    ) {
        let bogeyman_position = HousesDrawer::position(
            houses.iter().nth(boogeyman.position()).unwrap(),
            window_rect,
        );
        draw.texture(boogeyman.texture())
            .w_h(texture_size, texture_size)
            .x_y(bogeyman_position.x, bogeyman_position.y);
    }
}
