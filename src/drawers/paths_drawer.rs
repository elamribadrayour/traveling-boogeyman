use crate::houses::Houses;
use crate::paths::Paths;

use nannou::draw::Draw;
use nannou::geom::Rect;

use crate::drawers::houses_drawer::HousesDrawer;

pub struct PathsDrawer<const PATH_COUNT: usize>;

impl<const PATH_COUNT: usize> PathsDrawer<PATH_COUNT> {
    pub fn draw(draw: &Draw, window_rect: Rect, paths: &Paths<PATH_COUNT>, houses: &Houses) {
        let best = paths.best();
        let len = best.len();
        for i in 0..len {
            let house_1 = houses.at(best.path()[i]).unwrap();
            let house_2 = houses.at(best.path()[(i + 1) % len]).unwrap();
            let pos_1 = HousesDrawer::position(house_1, window_rect);
            let pos_2 = HousesDrawer::position(house_2, window_rect);
            draw.arrow()
                .start(pos_1)
                .end(pos_2)
                .weight(2.0)
                .caps_round()
                .color(nannou::color::BLACK);
        }
    }
}
