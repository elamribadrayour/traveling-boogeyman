use crate::house::House;
use crate::houses::Houses;

use nannou::draw::Draw;
use nannou::geom::{Rect, Vec2};
use nannou::math::map_range;

pub struct HousesDrawer;

impl HousesDrawer {
    pub fn position(house: &House, window_rect: Rect) -> Vec2 {
        let position = house.position();
        Vec2::new(
            map_range(
                position.x,
                0.0,
                1.0,
                window_rect.left(),
                window_rect.right(),
            ),
            map_range(
                position.y,
                0.0,
                1.0,
                window_rect.bottom(),
                window_rect.top(),
            ),
        )
    }

    pub fn draw(draw: &Draw, window_rect: Rect, texture_size: f32, houses: &Houses) {
        for house in houses.iter() {
            let position = HousesDrawer::position(house, window_rect);
            draw.texture(&house.texture())
                .w_h(texture_size, texture_size)
                .x_y(position.x, position.y);
        }
    }
}
