use quicksilver::graphics::Color;

pub struct World {
    pub label: String,
    pub matrix: Vec<Vec<(u32, Color)>>, // a list of squares
}

impl World {
    pub fn get_simple_world(window_height: u32, world_length: u32) -> World {
        /*
         * We create two lines
         */
        let mut matrix = Vec::new();

        for i in 0..world_length {
            if i % 2 == 0 {
                matrix.push(vec![(2, Color::RED), (window_height - 2, Color::RED)]);
            } else if i % 3 == 0 {
                matrix.push(vec![(4, Color::RED), (window_height - 2, Color::RED)]);
            } else {
                matrix.push(vec![(3, Color::BLUE), (window_height - 2, Color::BLUE)]);
            }
        }

        World {
            label: String::from("basic world"),
            matrix,
        }
    }
}
