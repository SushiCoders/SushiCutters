use crate::components::{CircleCollider, Health};
use amethyst::{core::transform::Transform, ecs::prelude::*};

use crate::components::enemy;
use crate::sushi_cutters::{ARENA_HEIGHT, ARENA_WIDTH};

pub type SceneInitializer = fn(&mut World);

pub struct Scene {
    pub name: String,
    pub initializer: SceneInitializer,
}

pub type Scenes = Vec<Scene>;

pub fn scenes() -> Scenes {
    let mut s = Vec::new();

    // Typecast is necesary because functions contain their name
    // as part of their function signature so the compiler gets
    // upset unless you cast the pointers
    s.push(Scene {
        name: String::from("basic"),
        initializer: initialise_raw_colliders as SceneInitializer,
    });

    s.push(Scene {
        name: String::from("enemies"),
        initializer: initialize_enemies as SceneInitializer,
    });

    s
}

const CIRCLE_SIZE: f32 = 4.0_f32;
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

pub fn initialize_enemies(world: &mut World) {
    use rand::distributions::{Distribution, Uniform};
    let mut rng = rand::thread_rng();
    let enemy_count = Uniform::new(1, 20);
    let direction = Uniform::new(-1.0, 1.0);
    let velocity = Uniform::new(f32::EPSILON, 50.0);
    let enemy_x = Uniform::new(
        enemy::HITCIRCLE_RADIUS,
        ARENA_WIDTH - enemy::HITCIRCLE_RADIUS,
    );
    let enemy_y = Uniform::new(
        enemy::HITCIRCLE_RADIUS,
        ARENA_HEIGHT - enemy::HITCIRCLE_RADIUS,
    );
    for _ in 1..=enemy_count.sample(&mut rng) {
        enemy::spawn_enemy(
            world,
            direction.sample(&mut rng) * enemy_x.sample(&mut rng),
            direction.sample(&mut rng) * enemy_y.sample(&mut rng),
            velocity.sample(&mut rng),
            velocity.sample(&mut rng),
        );
    }
}
