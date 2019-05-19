use crate::model::runner::*;
use crate::state_manager::GameState;
use crate::state_manager::GameTransition;
use quicksilver::prelude::*;

use crate::state_manager::GameData;

pub struct RunState {
    pub data: GameData,
}

impl GameState for RunState {
    fn draw(self: &Self, window: &mut Window) {
        // draw world
        self.data.world.draw(window);

        // draw runners

        self.data
            .runners
            .iter()
            .filter(|runner| !runner.is_dead)
            .for_each(|runner| {
                runner.draw(window);
            });
    }

    fn update(self: &mut Self) -> Option<GameTransition> {
        // update position of world
        self.data.world.update_position();

        // update position of runners

        let old_runners = &self.data.runners;

        self.data.runners = old_runners
            .iter()
            .filter(|runner| !runner.is_dead)
            .map(|old_runner| {
                let mut runner = old_runner.clone();

                let other_runners: Vec<Runner> = old_runners
                    .iter()
                    .cloned()
                    .filter(|r| r != old_runner)
                    .collect();

                runner.update_position(&other_runners, &self.data.world.rectangles);

                runner
            })
            .collect();

        None
    }

    fn handle_event(&mut self, event: &Event) -> Option<GameTransition> {
        if let Event::Key(key, ButtonState::Pressed) = event {
            if key == &Key::Escape {
                return Some(GameTransition::RunToMenu);
            }
       
            for runner in &mut self.data.runners {
                if !runner.is_dead && !runner.is_falling && key == &runner.key {
                    runner.change_gravity();
                }
            }
        }

        None
    }

    fn get_data(self: &Self) -> GameData {
        self.data.clone()
    }
}
