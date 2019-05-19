use crate::states::run::RunState;
use crate::states::menu::MenuState;
use crate::model::runner::*;
use crate::model::world::*;
use quicksilver::lifecycle::Event;
use quicksilver::lifecycle::State;
use quicksilver::lifecycle::Window;
use quicksilver::Result;

use quicksilver::graphics::Color;

// enum GameStateType {
//     Menu,
//     Loading,
//     Game,
//     Score,
// }

pub enum GameTransition {
    MenuToRun,
    RunToMenu
}

#[derive(Clone)]
pub struct GameData {
    pub world: World,
    pub runners: Vec<Runner>,
}

pub struct StateManager {
    current_state: Box<GameState>
}

impl State for StateManager {
    fn new() -> Result<StateManager> {
        let game_data = GameData {
            world: World::get_simple_world(600 / SQUARE_SIZE, 100000),
            runners: Vec::new(),
        };

        Ok(StateManager {
            current_state: Box::new(crate::states::menu::MenuState::new(game_data))
        })
    }

    fn draw(self: &mut Self, window: &mut Window) -> Result<()> {
        window.clear(Color::from_hex("0A1B40"))?;

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
            },
            Some(GameTransition::RunToMenu) => {
                println!("Go to menu !!");
                self.current_state = Box::new(MenuState::new(self.current_state.get_data()));
            },
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
