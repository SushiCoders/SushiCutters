///! Core `SushiCutters` module
///! There is a bit of code that was taken from the pong example which will be phased out in time
extern crate rand;
use crate::{components::initialize_player, scenes};
use amethyst::{core::transform::Transform, ecs::prelude::*, prelude::*, renderer::Camera};

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
    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // Typecast is necesary because functions contain their name
        // as part of their function signature so the compiler gets
        // upset unless you cast the pointers
        let initializer = if cfg!(feature = "enemies") {
            Some(scenes::initialize_enemies as fn(&mut World))
        } else {
            Some(scenes::initialise_raw_colliders as fn(&mut World))
        };

        SimpleTrans::Push(Box::new(SushiCutters { initializer }))
    }
}

#[derive(Default)]
pub struct SushiCutters {
    initializer: Option<fn(&mut World)>,
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
