use core::ops::Not;
use quicksilver::geom::{Line, Rectangle, Shape, Vector};
use quicksilver::graphics::Color;

pub const RUNNER_SIZE: u32 = 20; // square in pixels

#[derive(Copy, Clone)]
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

pub struct Runner {
    pub color: Color,
    pub shape: Rectangle,
    pub gravity: Gravity,
    pub dead: bool,
    pub is_falling: bool,
}

impl Runner {
    pub fn default(width: u32, height: u32, color: Color) -> Runner {
        Runner {
            color,
            shape: Rectangle::new(
                ((width - RUNNER_SIZE) / 2, (height - RUNNER_SIZE - 10) / 2),
                (RUNNER_SIZE, RUNNER_SIZE + 10),
            ),
            gravity: Gravity::DOWN,
            dead: false,
            is_falling: false,
        }
    }

    pub fn change_gravity(self: &mut Self) {
        self.gravity = !self.gravity;
    }

    pub fn in_flight(self: &mut Self, rectangles: &[Rectangle]) -> bool {
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

        // la ligne supérieur ou inférieure du runner touche un rect  => pas in flight
        self.is_falling = !(overlaped_rect > 0);

        self.is_falling
    }

    pub fn blocked(self: &Self, rectangles: &[Rectangle]) -> bool {
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

        overlaped_rect > 0
    }

    pub fn is_late(self: &Self, screen_size: Vector) -> bool {
        self.shape.pos.x < screen_size.x / 2.0
    }

    pub fn accelerate(self: &mut Self, pixel_speed: f32) {
        self.shape.pos.x += pixel_speed;
    }

    pub fn fall(self: &mut Self) {
        self.shape.pos = Vector::new(
            self.shape.pos.x,
            self.shape.pos.y + self.gravity.value() as f32,
        );
    }

    pub fn go_back(self: &mut Self, pixel_speed: f32) {
        self.shape.pos = Vector::new(self.shape.pos.x - pixel_speed, self.shape.pos.y);
    }

    pub fn is_alive(self: &mut Self, screen_size: Vector) {
        if self.shape.x() + self.shape.width() < 0.0 || self.shape.y() > screen_size.y {
            self.dead = true;
            println!("Runner is dead ! ");
        }
    }
}
