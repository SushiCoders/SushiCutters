///! Core `SushiCutters` module
///! There is a bit of code that was taken from the pong example which will be phased out in time
use amethyst::{core::transform::Transform, ecs::prelude::*, prelude::*, renderer::Camera};
extern crate rand;
use crate::components::{initialize_enemies, initialize_player, CircleCollider, Health};

// Maybe make these into a resouce?
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

const CIRCLE_SIZE: f32 = 4.0_f32;
/// TEMP: Colliders should always have a purpose/other components
/// Hardcoded for testing purposes
pub fn initialise_raw_colliders(world: &mut World) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_translation_xyz(CIRCLE_SIZE, y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - CIRCLE_SIZE, y, 0.0);

    let health = Health { amount: 10.0 };
    world
        .create_entity()
        .with(CircleCollider {
            radius: CIRCLE_SIZE,
        })
        .with(health.clone())
        .with(right_transform)
        .build();

    world
        .create_entity()
        .with(CircleCollider {
            radius: CIRCLE_SIZE,
        })
        .with(health)
        .with(left_transform)
        .build();
}

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
pub struct SushiCutters;

impl SimpleState for SushiCutters {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_camera(world);
        if cfg!(feature = "enemies") {
            initialize_enemies(world);
        } else {
            initialise_raw_colliders(world);
        }
        initialize_player(world);
    }
}
