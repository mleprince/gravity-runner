extern crate quicksilver;

mod state_manager;
mod states;
mod model;

use crate::model::world::SQUARE_SPEED;
use crate::model::world::SQUARE_SIZE;
use quicksilver::geom::Vector;
use quicksilver::lifecycle::run;
use quicksilver::lifecycle::Settings;
use state_manager::StateManager;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const GRAVITY_FORCE: u32 = 2;

const PIXEL_SPEED: f32 = SQUARE_SIZE as f32 * SQUARE_SPEED as f32 / 60.0;

fn main() {
    run::<StateManager>("Game", Vector::new(WIDTH, HEIGHT), Settings::default());
}
