use quicksilver::geom::Rectangle;
use quicksilver::graphics::Color;

pub const RUNNER_SIZE: u32 = 20; // square in pixels

pub enum Gravity {
    UP,
    DOWN,
}

pub struct Runner {
    pub color: Color,
    pub pos_x: u32,
    pub pos_y: u32,
    pub gravity: Gravity,
}

impl Runner {
    pub fn default(width: u32, height: u32, color: Color) -> Runner {
        Runner {
            pos_x: (width - RUNNER_SIZE) / 2,
            pos_y: (height - RUNNER_SIZE) / 2,
            color,
            gravity: Gravity::DOWN,
        }
    }

    pub fn in_flight(self: &Self, collisions: &[Rectangle]) -> bool {

        let mut flight = true;

        for collided_rect in collisions {
            
        }

        flight
    }

    pub fn blocked(self: &Self, collisions: &[Rectangle]) -> bool {
        false
    }

    pub fn fall(self: &Self) {}

    pub fn go_back(self: &mut Self) {}
}
