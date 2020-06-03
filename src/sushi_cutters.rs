///! Core `SushiCutters` module
///! There is a bit of code that was taken from the pong example which will be phased out in time
extern crate rand;
use crate::{components::initialize_player, scenes, scenes::SceneInitializer};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::*,
    prelude::*,
    renderer::Camera,
    winit::{Event, WindowEvent},
};

// Maybe make these into a resouce?
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

#[derive(Default)]
pub struct SceneSelect;

impl SimpleState for SceneSelect {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Please select a scene (screen must be in focus)");

        for (index, scene) in scenes::SCENES.iter().enumerate() {
            println!("{}: {}", index, scene.name);
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
                            let initializer = Some(s[num].initializer);
                            return SimpleTrans::Switch(Box::new(SushiCutters { initializer }));
                        } else {
                            println!("{} is out of bounds", num);
                        }
                    }
                }
            }
        }

        SimpleTrans::None
    }
}

#[derive(Default)]
pub struct SushiCutters {
    initializer: Option<SceneInitializer>,
}

impl SimpleState for SushiCutters {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_camera(world);

        if let Some(init) = self.initializer {
            init(world);
        }

        initialize_player(world);
    }
}
