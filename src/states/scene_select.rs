use crate::scenes;

use amethyst::{
    prelude::*,
    winit::{Event, WindowEvent},
};

use log::{info, warn};
#[derive(Default)]
pub struct SceneSelectState;

impl SimpleState for SceneSelectState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        info!("Please select a scene (screen must be in focus)");

        for (index, scene) in scenes::SCENES.iter().enumerate() {
            info!("{}: {}", index, scene.name);
        }

        // TODO: Generate UI
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        // TODO: Clean up UI
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let s = scenes::SCENES;

        // TODO: Use UI instead of a keyboard input
        // Right now this is limited to only 10 scenes
        if let StateEvent::Window(event) = &event {
            if let Event::WindowEvent { ref event, .. } = event {
                if let WindowEvent::ReceivedCharacter(c) = event {
                    if let Some(num) = c.to_digit(10) {
                        let num: usize = num as usize;
                        if num < s.len() {
                            let initializer = s[num].initializer;
                            return SimpleTrans::Switch(Box::new(
                                super::running::RunningState::new(initializer),
                            ));
                        } else {
                            warn!("{} is out of bounds", num);
                        }
                    }
                }
            }
        }

        SimpleTrans::None
    }
}
