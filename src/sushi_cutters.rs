///! Core `SushiCutters` module
///! There is a bit of code that was taken from the pong example which will be phased out in time
extern crate rand;
use crate::{components::initialize_player, scenes, scenes::SceneInitializer};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::*,
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::Camera,
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
pub struct SceneSelect {
    scenes: Option<scenes::Scenes>,
    keys: Option<Vec<String>>,
}

impl SimpleState for SceneSelect {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        self.scenes = Some(scenes::scenes());

        println!("Please select a scene (screen must be in focus)");

        if let Some(s) = self.scenes.as_ref() {
            let s: &scenes::Scenes = s;

            // Store a copy of keys so that the order stays the same
            // when we need to use the keys later
            let keys: Vec<String> = s.keys().cloned().collect();
            for (index, key) in keys.iter().enumerate() {
                println!("{}: {}", index, key);
            }
            self.keys = Some(keys);
        }
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let s = self.scenes.as_ref().unwrap();
        let keys = self.keys.as_ref().unwrap();

        if let StateEvent::Window(event) = &event {
            if is_key_down(event, VirtualKeyCode::Key0) {
                // Copy the value out of the map so that we don't have
                // to deal with references
                let initializer = s.get(&keys[0]).copied();
                return SimpleTrans::Push(Box::new(SushiCutters { initializer }));
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
