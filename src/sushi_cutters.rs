///! Core `SushiCutters` module
///! There is a bit of code that was taken from the pong example which will be phased out in time
use amethyst::{
    assets::Loader,
    core::transform::Transform,
    ecs::prelude::*,
    prelude::*,
    renderer::Camera,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

extern crate rand;

use crate::components::{initialize_enemies, initialize_player, CircleCollider, Health};
use crate::systems::score_ui::ScoreText;

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
        initialize_score(world);
    }
}

fn initialize_score(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "Fira/FiraSans-Regular.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let score_transform = UiTransform::new(
        "score".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        0_f32,
        0_f32,
        0_f32,
        200_f32,
        50_f32,
    );
    let player_score_entity = world
        .create_entity()
        .with(score_transform)
        .with(UiText::new(
            font,
            ScoreText::format_from_str(0),
            [1_f32; 4],
            50_f32,
        ))
        .build();
    world.insert(ScoreText {
        player_score_entity,
    })
}
