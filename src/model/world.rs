use quicksilver::geom::Rectangle;
use quicksilver::graphics::Color;
use quicksilver::lifecycle::Window;
use quicksilver::prelude::Col;

pub const SQUARE_SIZE: u32 = 20; // in pixels
pub const SQUARE_SPEED: f64 = 10.0; // 10 carré qui disparait par second


#[derive(Clone)]
pub struct World {
    pub label: String,
    pub matrix: Vec<Vec<u32>>, // a list of squares
    pub rectangles: Vec<Rectangle>,
    position: f32,
}

impl World {
    pub fn get_simple_world(window_height: u32, world_length: u32) -> World {
        /*
         * We create two lines
         */
        let rectangles = Vec::new();

        let mut matrix = Vec::new();

        let mut k = 0;

        for _ in 0..world_length {
            matrix.push(vec![2 + k, window_height - 2 - k]);

            if k < 5 {
                k += 1;
            } else {
                k = 0;
            }
        }

        World {
            label: String::from("basic world"),
            matrix,
            rectangles,
            position: 0.0,
        }
    }

    // function called 60 times by second
    pub fn update_position(self: &mut Self) {
        // 200 pixels par second
        self.position += crate::PIXEL_SPEED;

        let index_rect = (self.position as u32 - self.position as u32 % SQUARE_SIZE) / SQUARE_SIZE;

        self.rectangles = Vec::new();

        for i in 0..(crate::WIDTH / SQUARE_SIZE) + 1 {
            let x: i32 = (i * SQUARE_SIZE) as i32 - (self.position as u32 % SQUARE_SIZE) as i32;

            for sqare_pos_y in &self.matrix[(index_rect + i) as usize] {
                let y = sqare_pos_y * SQUARE_SIZE;

                let rect = Rectangle::new((x, y), (SQUARE_SIZE, SQUARE_SIZE));

                self.rectangles.push(rect);
            }
        }
    }

    pub fn draw(self: &Self, window: &mut Window) {
        self.rectangles.iter().for_each(|rectangle| {
            window.draw(rectangle, Col(Color::RED));
        });
    }
}
