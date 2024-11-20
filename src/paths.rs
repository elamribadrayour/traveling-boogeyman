use crate::houses::Houses;
use crate::path::Path;

use rand::{Rng, RngCore};

pub struct Paths<const PATH_COUNT: usize> {
    paths: [Path; PATH_COUNT],
}

impl<const PATH_COUNT: usize> Paths<PATH_COUNT> {
    pub fn new(rng: &mut dyn RngCore, house_count: usize) -> Self {
        let paths = core::array::from_fn(|_| Path::new(rng, house_count));
        Self { paths }
    }

    pub fn len(&self) -> usize {
        self.paths.len()
    }

    pub fn fitness(&self) -> f32 {
        self.paths
            .iter()
            .map(|path| path.fitness())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(-1.0)
    }

    pub fn best(&self) -> &Path {
        self.paths
            .iter()
            .max_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap())
            .unwrap()
    }

    pub fn evaluate(&mut self, houses: &Houses) {
        for path in &mut self.paths {
            path.evaluate(houses);
        }
    }

    pub fn select(&mut self, rng: &mut dyn RngCore, tournament_rate: f32) -> usize {
        let tournament_size = (tournament_rate * self.paths.len() as f32).ceil() as usize;
        let mut best_index = rng.gen_range(0..self.paths.len());
        for _ in 1..tournament_size {
            let index = rng.gen_range(0..self.paths.len());
            if self.paths[index].fitness() > self.paths[best_index].fitness() {
                best_index = index;
            }
        }
        best_index
    }

    pub fn crossover(&mut self, rng: &mut dyn RngCore, tournament_rate: f32, crossover_rate: f32) {
        let paths = core::array::from_fn(|_| {
            let i1 = self.select(rng, tournament_rate);
            let i2 = self.select(rng, tournament_rate);
            let p1 = &self.paths[i1];
            let p2 = &self.paths[i2];
            p1.crossover(rng, p2, crossover_rate)
        });
        self.paths = paths;
    }

    pub fn mutate(&mut self, rng: &mut dyn RngCore, mutation_rate: f32) {
        for path in &mut self.paths {
            path.mutate(rng, mutation_rate);
        }
    }
}
