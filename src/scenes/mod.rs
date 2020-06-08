use crate::components::{CircleCollider, Health};
use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::*,
};

use crate::components::enemy;
use crate::sushi_cutters::{ARENA_HEIGHT, ARENA_WIDTH};

pub type SceneInitializer = fn(&mut World);

pub struct Scene {
    pub name: &'static str,
    pub initializer: SceneInitializer,
}

pub const SCENES: [Scene; 2] = [
    Scene {
        name: "basic",
        initializer: initialize_raw_colliders as SceneInitializer,
    },
    Scene {
        name: "enemies",
        initializer: initialize_enemies as SceneInitializer,
    },
];

fn create_test_colliders(world: &mut World, transforms: Vec<Transform>) {
    let health = Health { amount: 10.0 };
    let collider = CircleCollider {
        radius: CIRCLE_SIZE,
    };

    for transform in transforms {
        world
            .create_entity()
            .with(collider.clone())
            .with(health.clone())
            .with(transform)
            .build();
    }
}

const CIRCLE_SIZE: f32 = 4.0_f32;
pub fn initialize_raw_colliders(world: &mut World) {
    let y = ARENA_HEIGHT / 2.0;

    let transforms = vec![
        Transform::from(Vector3::from([CIRCLE_SIZE, y, 0.0])),
        Transform::from(Vector3::from([ARENA_WIDTH - CIRCLE_SIZE, y, 0.0])),
    ];

    create_test_colliders(world, transforms);
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
