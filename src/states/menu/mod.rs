use quicksilver::geom::Shape;
use quicksilver::prelude::*;

use crate::model::runner::*;
use crate::state_manager::*;

pub struct MenuState {
    pub data: GameData,
    numbers: Vec<Number>,
    title: Image,
    sentence: Image,
}

impl MenuState {
    pub fn new(data: GameData) -> Self {
        let general_font = Font::from_slice(include_bytes!("../../static/Prototype.ttf")).unwrap();

        let title_font = Font::from_slice(include_bytes!("../../static/game_over.ttf")).unwrap();

        // create numbers

        let mut numbers = Vec::new();

        for i in 1..7 {
            numbers.push(Number {
                index: i,
                image: general_font
                    .render(
                        format!("{}", i).as_str(),
                        &FontStyle::new(50.0, Color::WHITE),
                    )
                    .unwrap(),
                image_hover: general_font
                    .render(
                        format!("{}", i).as_str(),
                        &FontStyle::new(60.0, Color::from_hex("ff5151")),
                    )
                    .unwrap(),
                hover: false,
            })
        }

        // create Title

        let title = title_font
            .render("GRAVITY RUNNER", &FontStyle::new(100.0, Color::WHITE))
            .unwrap();
        // draw sentence

        let sentence = general_font
            .render(
                "Select number of players :",
                &FontStyle::new(40.0, Color::WHITE),
            )
            .unwrap();

        // create Sentence

        MenuState {
            data,
            numbers: numbers,
            title,
            sentence,
        }
    }
}

impl GameState for MenuState {
    fn get_data(self: &Self) -> GameData {
        self.data.clone()
    }
    fn draw(self: &Self, window: &mut Window) {
        // draw title

        window.draw(&self.title.area().with_center((400, 150)), Img(&self.title));

        // draw sentence

        window.draw(
            &self.sentence.area().with_center((400, 400)),
            Img(&self.sentence),
        );

        // draw numbers

        self.numbers.iter().for_each(|number| {
            number.draw(window);
        })
    }

    fn update(self: &mut Self) -> Option<GameTransition> {
        None
    }
    fn handle_event(&mut self, event: &Event) -> Option<GameTransition> {
        let mut transition: Option<GameTransition> = None;

        if let Event::Key(key, ButtonState::Pressed) = event {
            if key == &Key::Return {
                return Some(GameTransition::MenuToRun);
            }
        } else if let Event::MouseMoved(coord) = event {
            self.numbers.iter_mut().for_each(|number| {
                number.hover = number.get_area().contains(coord.into_vector());
            });
        } else if let Event::MouseButton(MouseButton::Left, ButtonState::Pressed) = event {
            let number: Option<u32> = self
                .numbers
                .iter()
                .filter(|number| number.hover)
                .map(|number| number.index)
                .take(1)
                .next();

            if let Some(index) = number {
                for i in 0..index {
                    let runner = Runner::default(300, 300 + 50 * i, i);
                    self.data.runners.push(runner);
                    println!("sdfsdf");
                    println!("{:?}", self.data.runners);
                }

                return Some(GameTransition::MenuToRun);
            }
        }

        None
    }
}

struct Number {
    index: u32,
    image: Image,
    image_hover: Image,
    hover: bool,
}

impl Number {
    fn get_area(self: &Self) -> Rectangle {
        self.image
            .area()
            .with_center((400, 450))
            .translate((50.0 * (self.index as f32 - 3.5), 0))
    }

    fn draw(self: &Self, window: &mut Window) {
        if self.hover {
            window.draw(&self.get_area(), Img(&self.image_hover));
        } else {
            window.draw(&self.get_area(), Img(&self.image));
        }
    }
}
