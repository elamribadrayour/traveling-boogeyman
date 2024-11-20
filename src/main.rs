mod boogeyman;
mod drawers;
mod house;
mod houses;
mod path;
mod paths;

use rand::{thread_rng, RngCore};

use crate::boogeyman::Boogeyman;
use crate::houses::Houses;
use crate::paths::Paths;

use nannou::prelude::*;

const PATH_COUNT: usize = 10;
const HOUSE_COUNT: usize = 15;
const NB_ITERATIONS: usize = 1000;

const TEXTURE_SIZE: f32 = 50.0;
const WINDOW_SIZE: f32 = 1024.0;

const MUTATION_RATE: f32 = 0.1;
const CROSSOVER_RATE: f32 = 0.7;
const TOURNAMENT_RATE: f32 = 0.3;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    houses: Houses,
    iteration: usize,
    bogeyman: Boogeyman,
    fitnesses: Vec<f32>,
    rng: Box<dyn RngCore>,
    paths: Paths<PATH_COUNT>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WINDOW_SIZE as u32, WINDOW_SIZE as u32)
        .view(view)
        .build()
        .unwrap();

    let iteration = 0;
    let mut rng = Box::new(thread_rng());
    let houses = Houses::new(&mut rng, app, HOUSE_COUNT);
    let bogeyman = Boogeyman::new(&mut rng, app, HOUSE_COUNT);
    let paths = Paths::new(&mut rng, HOUSE_COUNT);
    Model {
        fitnesses: vec![],
        houses,
        paths,
        iteration,
        bogeyman,
        rng,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.paths.evaluate(&model.houses);
    model.fitnesses.push(model.paths.fitness());
    if model.fitnesses.last().unwrap() == &1.0 || model.iteration >= NB_ITERATIONS {
        return;
    }

    model
        .paths
        .crossover(&mut model.rng, TOURNAMENT_RATE, CROSSOVER_RATE);
    model.paths.mutate(&mut model.rng, MUTATION_RATE);
    model.iteration += 1;

    println!(
        "iteration: {} -- len: {} -- mean: {:.2} -- mean-last-5: {:.2} -- last: {:.2}",
        model.iteration,
        model.paths.len(),
        model.fitnesses.iter().sum::<f32>() / model.fitnesses.len() as f32,
        model.fitnesses.iter().rev().take(5).sum::<f32>() / 5.0,
        model.fitnesses.last().unwrap()
    );
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let bg_color = rgb(0.5, 1.0, 0.5);
    let window_rect = app.window_rect().pad(TEXTURE_SIZE);

    draw.background().color(bg_color);
    drawers::HousesDrawer::draw(&draw, window_rect, TEXTURE_SIZE, &model.houses);
    drawers::BoogeymanDrawer::draw(
        &draw,
        window_rect,
        TEXTURE_SIZE,
        &model.bogeyman,
        &model.houses,
    );
    drawers::PathsDrawer::draw(&draw, window_rect, &model.paths, &model.houses);

    draw.text(&format!(
        "iteration: {} -- fitness: {:.2}",
        model.iteration,
        model.fitnesses.last().unwrap()
    ))
    .x_y(window_rect.left() + 100.0, window_rect.top() - 10.0)
    .font_size(15)
    .color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}
