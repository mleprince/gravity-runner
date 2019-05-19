use core::ops::Not;
use quicksilver::geom::{Line, Rectangle, Shape, Vector};
use quicksilver::graphics::Color;
use quicksilver::input::Key;
use quicksilver::lifecycle::Window;
use quicksilver::prelude::Background::Col;

pub const RUNNER_SIZE: u32 = 20; // square in pixels

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Gravity {
    UP = -1,
    DOWN = 1,
}

impl Gravity {
    fn value(&self) -> f32 {
        match *self {
            Gravity::UP => -(crate::GRAVITY_FORCE as f32),
            Gravity::DOWN => crate::GRAVITY_FORCE as f32,
        }
    }
}

impl Not for Gravity {
    type Output = Gravity;

    fn not(self) -> Gravity {
        match self {
            Gravity::UP => Gravity::DOWN,
            Gravity::DOWN => Gravity::UP,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Runner {
    pub color: Color,
    pub shape: Rectangle,
    pub gravity: Gravity,
    pub is_falling: bool,
    pub key: Key,
    pub is_dead: bool,
}

impl Runner {
    pub fn default(pos_x: u32, pos_y: u32, index: u32) -> Runner {
        Runner {
            color: Self::get_runner_color(index),
            shape: Rectangle::new((pos_x, pos_y), (RUNNER_SIZE, RUNNER_SIZE + 10)),
            gravity: Gravity::DOWN,
            is_falling: false,
            key: Self::get_runner_key(index),
            is_dead: false,
        }
    }

    fn get_runner_color(index: u32) -> Color {
        match index {
            0 => Color::YELLOW,
            1 => Color::BLUE,
            2 => Color::GREEN,
            _ => Color::BLACK,
        }
    }

    fn get_runner_key(index: u32) -> Key {
        match index {
            0 => Key::A,
            1 => Key::M,
            2 => Key::Space,
            _ => Key::P,
        }
    }

    pub fn update_position(self: &mut Self, other_runners: &[Runner], world: &[Rectangle]) {
        if self.in_flight(world, other_runners) {
            self.fall();
        }

        if self.blocked(world, &other_runners) {
            println!("blocked !");
            self.go_back(crate::PIXEL_SPEED);
        } else if self.is_late() {
            self.accelerate(crate::PIXEL_SPEED / 2.0);
        }

        if self.has_fallen_to_the_unknown() {
            self.is_dead = true;
        }
    }

    pub fn change_gravity(self: &mut Self) {
        self.gravity = !self.gravity;
    }

    fn in_flight(self: &mut Self, rectangles: &[Rectangle], other_runners: &[Runner]) -> bool {
        let y_line = match self.gravity {
            Gravity::UP => self.shape.y() - self.gravity.value(),
            Gravity::DOWN => self.shape.y() + self.shape.height() + self.gravity.value(),
        };

        let line = Line::new(
            (self.shape.x(), y_line),
            (self.shape.x() + self.shape.width(), y_line),
        );

        let overlaped_rect = rectangles
            .iter()
            .filter(|rect| line.overlaps_rectangle(rect))
            .count();

        let overlaped_runner = other_runners
            .iter()
            .filter(|runner| line.overlaps_rectangle(&runner.shape))
            .count();

        // la ligne supérieur ou inférieure du runner touche un rect  => pas in flight
        self.is_falling = !(overlaped_rect > 0) && !(overlaped_runner > 0);

        self.is_falling
    }

    pub fn blocked(self: &Self, rectangles: &[Rectangle], other_runners: &[Runner]) -> bool {
        let line = Line::new(
            (self.shape.x() + self.shape.width(), self.shape.y()),
            (
                self.shape.x() + self.shape.width(),
                self.shape.y() + self.shape.height(),
            ),
        );

        let overlaped_rect = rectangles
            .iter()
            .filter(|rect| line.overlaps_rectangle(rect))
            .count();

        let overlaped_runner = other_runners
            .iter()
            .filter(|runner| line.overlaps_rectangle(&runner.shape))
            .count();

        overlaped_rect > 0 || overlaped_runner > 0
    }

    fn is_late(self: &Self) -> bool {
        self.shape.pos.x < crate::WIDTH as f32 / 2.0
    }

    fn accelerate(self: &mut Self, pixel_speed: f32) {
        self.shape.pos.x += pixel_speed / 2.0;
    }

    fn fall(self: &mut Self) {
        self.shape.pos = Vector::new(
            self.shape.pos.x,
            self.shape.pos.y + self.gravity.value() as f32,
        );
    }

    fn go_back(self: &mut Self, pixel_speed: f32) {
        self.shape.pos = Vector::new(self.shape.pos.x - pixel_speed, self.shape.pos.y);
    }

    fn has_fallen_to_the_unknown(self: &mut Self) -> bool {
        if self.shape.x() + (crate::WIDTH as f32) < 0.0 || self.shape.y() > crate::HEIGHT as f32 {
            println!("Runner is dead ! ");
            return true;
        }

        false
    }

    pub fn draw(self: &Self, window: &mut Window) {
        window.draw(&self.shape, Col(self.color));
    }
}
