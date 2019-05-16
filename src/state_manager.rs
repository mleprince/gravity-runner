use crate::runner::Runner;
use crate::world::World;
use quicksilver::input::Key;
use quicksilver::lifecycle::Event;
use quicksilver::lifecycle::State;
use quicksilver::lifecycle::Window;
use quicksilver::Result;

use quicksilver::graphics::Color;

use crate::state_menu::*;
use crate::state_run::*;

// enum GameStateType {
//     Menu,
//     Loading,
//     Game,
//     Score,
// }

pub enum GameTransition {
    MenuToRun,
}

#[derive(Clone)]
pub struct GameData {
    pub world: World,
    pub runners: Vec<Runner>,
}

pub struct StateManager {
    current_state: Box<GameState>,
}

impl State for StateManager {
    fn new() -> Result<StateManager> {
        let gameData = GameData {
            world: World::get_simple_world(600 / crate::world::SQUARE_SIZE, 100000),
            runners: vec![
                Runner::default(300, 350, Color::GREEN, Key::Space),
                Runner::default(300, 400, Color::YELLOW, Key::A),
            ],
        };

        Ok(StateManager {
            current_state: Box::new(MenuState { data: gameData }),
        })
    }

    fn draw(self: &mut Self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        self.current_state.draw(window);

        Ok(())
    }

    fn update(self: &mut Self, _: &mut Window) -> Result<()> {
        self.current_state.update();
        Ok(())
    }
    fn event(&mut self, event: &Event, _: &mut Window) -> Result<()> {
        match self.current_state.handle_event(event) {
            Some(GameTransition::MenuToRun) => {
                println!("Go to game !!!!");

                self.current_state = Box::new(RunState {
                    data: self.current_state.get_data(),
                });
            }
            _ => (),
        }

        Ok(())
    }
}

pub trait GameState {
    fn get_data(self: &Self) -> GameData;
    fn draw(self: &Self, window: &mut Window);
    fn update(self: &mut Self) -> Option<GameTransition>;
    fn handle_event(&mut self, event: &Event) -> Option<GameTransition>;
}
