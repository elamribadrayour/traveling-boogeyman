use crate::house::House;

use rand::RngCore;

use nannou::App;

pub struct Houses {
    houses: Vec<House>,
}

impl Houses {
    pub fn new(rng: &mut dyn RngCore, app: &App, house_count: usize) -> Self {
        Self {
            houses: (0..house_count).map(|_| House::new(rng, app)).collect(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &House> {
        self.houses.iter()
    }

    pub fn at(&self, index: usize) -> Option<&House> {
        self.houses.get(index)
    }
}
