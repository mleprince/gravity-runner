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

        let mut k = 0;

        for i in 0..world_length {
            matrix.push(vec![(2+k, Color::RED), (window_height - 2 - k , Color::RED)]);

            if k < 5 {
                k += 1;
            } else {
                k = 0;
            }
        }

        World {
            label: String::from("basic world"),
            matrix,
        }
    }
}
