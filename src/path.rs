use crate::houses::Houses;

use nannou::glam::Vec2;

use rand::seq::SliceRandom;
use rand::{Rng, RngCore};

pub struct Path {
    fitness: f32,
    path: Vec<usize>,
}

impl Path {
    pub fn new(rng: &mut dyn RngCore, house_count: usize) -> Self {
        let mut trajectory: Vec<usize> = (0..house_count).collect();
        trajectory.shuffle(rng);
        Self {
            path: trajectory,
            fitness: 0.0,
        }
    }

    pub fn fitness(&self) -> f32 {
        self.fitness
    }

    pub fn len(&self) -> usize {
        self.path.len()
    }

    pub fn path(&self) -> &Vec<usize> {
        &self.path
    }

    pub fn intersects(l1: (Vec2, Vec2), l2: (Vec2, Vec2)) -> f32 {
        let v1 = l1.1 - l1.0;
        let v2 = l2.1 - l2.0;

        let scalar = -v2.x * v1.y + v1.x * v2.y;
        if scalar.abs() < f32::EPSILON {
            return 0.0; // Lines are parallel or collinear
        }

        let s = (-v1.y * (l1.0.x - l2.0.x) + v1.x * (l1.0.y - l2.0.y)) / scalar;
        let t = (v2.x * (l1.0.y - l2.0.y) - v2.y * (l1.0.x - l2.0.x)) / scalar;

        if s > f32::EPSILON && s < 1.0 - f32::EPSILON && t > f32::EPSILON && t < 1.0 - f32::EPSILON
        {
            return 1.0;
        }

        0.0
    }

    pub fn evaluate(&mut self, houses: &Houses) {
        let mut paths = vec![];
        for i in 0..self.len() {
            let house_1 = houses.at(self.path[i]).unwrap();
            let house_2 = houses.at(self.path[(i + 1) % self.len()]).unwrap();
            paths.push((house_1.position(), house_2.position()));
        }

        let mut intersections = 0.0;
        for (i, l1) in paths.iter().enumerate() {
            for (j, l2) in paths.iter().enumerate() {
                if i == j {
                    continue;
                }
                intersections += Self::intersects(*l1, *l2);
            }
        }
        intersections /= paths.len() as f32;
        self.fitness = 1.0 / (1.0 + intersections);
    }

    pub fn crossover(&self, rng: &mut dyn RngCore, other: &Path, crossover_rate: f32) -> Self {
        let len = self.len();
        let mut path = vec![usize::MAX; len];

        if rng.gen_range(0.0..1.0) > crossover_rate {
            return Self {
                fitness: 0.0,
                path: self.path.clone(),
            };
        }

        let start = rng.gen_range(0..len);
        let end = rng.gen_range(start..len);
        (start..end).for_each(|i| path[i] = self.path[i]);

        let mut id = end;
        for &node in &other.path {
            if path.contains(&node) {
                continue;
            }
            while path[id] != usize::MAX {
                id = (id + 1) % len;
            }
            path[id] = node;
        }

        Self {
            path,
            fitness: 0.0, // Reset fitness as the path has changed
        }
    }

    pub fn mutate(&mut self, rng: &mut dyn RngCore, mutation_rate: f32) {
        for i in 0..self.len() {
            if rng.gen_range(0.0..1.0) > mutation_rate {
                continue;
            }
            let j = rng.gen_range(0..self.len());
            self.path.swap(i, j);
        }
    }
}
