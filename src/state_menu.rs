use quicksilver::input::ButtonState;
use quicksilver::input::Key;
use quicksilver::lifecycle::Event;
use quicksilver::lifecycle::Window;

use crate::state_manager::*;

pub struct MenuState {
    pub data: GameData,
}

impl GameState for MenuState {
    fn get_data(self: &Self) -> GameData {
        self.data.clone()
    }
    fn draw(self: &Self, window: &mut Window) {}

    fn update(self: &mut Self) -> Option<GameTransition> {
        None
    }
    fn handle_event(&mut self, event: &Event) -> Option<GameTransition> {
        if let Event::Key(key, ButtonState::Pressed) = event {
            if key == &Key::Return {
               return  Some(GameTransition::MenuToRun);
            }
        }

        None
    }
}
