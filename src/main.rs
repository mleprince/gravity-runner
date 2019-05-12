extern crate quicksilver;

mod runner;
mod world;

use quicksilver::lifecycle::Event;
use world::World;

use quicksilver::{
    geom::{Circle, Line, Rectangle, Shape, Transform, Triangle, Vector},
    graphics::{Background::Col, Color, View},
    input::{ButtonState, Key},
    lifecycle::{run, Settings, State, Window},
    prelude, Result,
};

use runner::*;

struct Game {
    world: World,
    position: f64,
    runners: Vec<Runner>,
}

const SQUARE_SIZE: u32 = 20; // in pixels
const SQUARE_SPEED: f64 = 10.0; // 1 carré qui disparait par second
const WIDTH: u32 = 800;
const GRAVITY_FORCE: u32 = 2;

impl Game {
    fn draw_world(self: &Self, window: &mut Window) -> Result<Vec<Rectangle>> {
        // rectangles of 10 pixels

        window.clear(Color::WHITE)?;

        /*
         * J'incrémente de 1 unité de temps
         */
        let index_world = self.position as u32 / SQUARE_SIZE;

        /*
        Je trouve la position en x du premier carré
        une frame fait 50ms
        on a une width de 800px
        je veux voir un carré disparaitre toutes les secondes
        un carré fait 20px => 40 carrés sur la window
        on avance de 20px toutes les secondes => 20px/seconde
        si on incrémente de 1pixels a chaque refrech => 1 frame = 50mS
        */

        let mut rectangles = Vec::new();

        for i in 0..(WIDTH / SQUARE_SIZE) + 1 {
            let x: i32 = (i * SQUARE_SIZE) as i32 - (self.position as u32 % SQUARE_SIZE) as i32;

            for (sqare_pos_y, color) in &self.world.matrix[(index_world + i) as usize] {
                let y = sqare_pos_y * SQUARE_SIZE;

                let rect = Rectangle::new((x, y), (SQUARE_SIZE, SQUARE_SIZE));

                window.draw(
                    &Rectangle::new((x, y), (SQUARE_SIZE, SQUARE_SIZE)),
                    Col(*color),
                );

                rectangles.push(rect);
            }
        }

        Ok(rectangles)
    }

    fn draw_runners(self: &mut Self, window: &mut Window, rectangles: Vec<Rectangle>) {
        self.runners
            .iter_mut()
            .filter(|runner| !runner.dead)
            .for_each(|runner| {
                if runner.in_flight(&rectangles) {
                    println!("Fall !");
                    runner.fall();
                }

                let pixel_speed = Self::get_pixel_speed(window.update_rate()) as f32;

                if runner.blocked(&rectangles) {
                    println!("blocked !");
                    runner.go_back(pixel_speed);
                } else if runner.is_late(window.screen_size()) {
                    runner.accelerate(pixel_speed / 2.0);
                }

                runner.is_alive(window.screen_size());

                window.draw(&runner.shape, Col(runner.color));
            });
    }

    fn increment_position(self: &mut Self, update_rate: f64) {
        self.position += Self::get_pixel_speed(update_rate);

        if self.position > 100000.0 {
            self.position = 0.0;
        }
    }

    fn get_pixel_speed(update_rate: f64) -> f64 {
        SQUARE_SIZE as f64 * SQUARE_SPEED / (1000.0 / update_rate)
    }
}

impl State for Game {
    fn new() -> Result<Game> {
        Ok(Game {
            world: World::get_simple_world(600 / SQUARE_SIZE, 100000),
            position: 0.0,
            runners: vec![Runner::default(800, 600, Color::GREEN)],
        })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // draw world

        let rectangles = self.draw_world(window)?;

        // draw runners

        self.draw_runners(window, rectangles);

        self.increment_position(window.update_rate());

        Ok(())
    }

    fn event(&mut self, event: &Event, _: &mut Window) -> Result<()> {
        if let Event::Key(Key::Space, ButtonState::Pressed) = event {
            for runner in &mut self.runners {
                if !runner.is_falling {
                    runner.change_gravity();
                }
            }
        }
        Ok(())
    }
}

fn main() {
    run::<Game>("Game", Vector::new(WIDTH, 600), Settings::default());
}
