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

pub const SCENES: [Scene; 4] = [
    Scene {
        name: "basic",
        initializer: initialize_raw_colliders as SceneInitializer,
    },
    Scene {
        name: "enemies",
        initializer: initialize_enemies_rand as SceneInitializer,
    },
    Scene {
        name: "enemies_bench",
        initializer: initialize_enemies_bench as SceneInitializer,
    },
    Scene {
        name: "enemies_scaled",
        initializer: initialize_enemies_scaled as SceneInitializer,
    },
];

pub fn get_scene(scene_name: &str) -> Option<&'static Scene> {
    SCENES.iter().find(|x| x.name == scene_name)
}

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

pub fn initialize_enemies_rand(world: &mut World) {
    use rand::distributions::{Distribution, Uniform};
    let enemy_count = Uniform::new(1, 20).sample(&mut rand::thread_rng());
    initialize_enemies(world, enemy_count, 4.0);
}

pub fn initialize_enemies_bench(world: &mut World) {
    let enemy_count = get_enemy_count();

    log::info!("`enemies_bench` starting with {} enemies", enemy_count);

    initialize_enemies(world, enemy_count, 4.0);
}

pub fn initialize_enemies_scaled(world: &mut World) {
    let enemy_count = get_enemy_count();
    let area_scale = get_area_scale();

    log::info!(
        "`enemies_scaled` starting with {} enemies and {} scale",
        enemy_count,
        area_scale
    );

    let area = ARENA_WIDTH * ARENA_HEIGHT;
    // We want circles to cover 80% of the area

    #[allow(clippy::cast_precision_loss)]
    let radius = ((area * area_scale) / (enemy_count as f32 * std::f32::consts::PI)).sqrt();

    initialize_enemies(world, enemy_count, radius);
}

fn get_variable<F: std::str::FromStr>(variable: &str, default: F) -> F {
    if let Ok(value) = std::env::var(variable) {
        if let Ok(value) = value.parse() {
            value
        } else {
            log::warn!("Invalid {}: '{}'", variable, value);
            default
        }
    } else {
        default
    }
}

fn get_enemy_count() -> usize {
    get_variable("ENEMY_COUNT", 5000)
}

fn get_area_scale() -> f32 {
    get_variable("AREA_SCALE", 0.8)
}

fn initialize_enemies(world: &mut World, count: usize, radius: f32) {
    use rand::distributions::{Distribution, Uniform};
    let mut rng = rand::thread_rng();
    let direction = Uniform::new(-1.0, 1.0);
    let velocity = Uniform::new(f32::EPSILON, 12.5 * radius);
    let enemy_x = Uniform::new(radius, ARENA_WIDTH - radius);
    let enemy_y = Uniform::new(radius, ARENA_HEIGHT - radius);
    for _ in 1..=count {
        enemy::spawn_enemy(
            world,
            enemy_x.sample(&mut rng),
            enemy_y.sample(&mut rng),
            direction.sample(&mut rng) * velocity.sample(&mut rng),
            direction.sample(&mut rng) * velocity.sample(&mut rng),
            radius,
        );
    }
}
