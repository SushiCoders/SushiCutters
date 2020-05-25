use super::{CircleCollider, Health, Velocity};
use crate::sushi_cutters::{ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::{
        prelude::{Component, NullStorage},
        World,
    },
    prelude::*,
};

pub const HITCIRCLE_RADIUS: f32 = 4.0;

#[derive(Default)]
pub struct Enemy;
impl Component for Enemy {
    type Storage = NullStorage<Self>;
}

pub fn spawn_enemy(world: &mut World, enemy_x: f32, enemy_y: f32, x_vel: f32, y_vel: f32) {
    println!("Spawning an enemy");
    let mut t = Transform::default();
    t.set_translation_xyz(enemy_x, enemy_y, 0.0);
    world
        .create_entity()
        .with(CircleCollider {
            radius: HITCIRCLE_RADIUS,
        })
        .with(Enemy)
        .with(Velocity {
            value: Vector3::new(x_vel, y_vel, 0.0),
        })
        .with(Health { amount: 10.0 })
        .with(t)
        .build();
}

pub fn initialize_enemies(world: &mut World) {
    use rand::distributions::{Distribution, Uniform};
    let mut rng = rand::thread_rng();
    let enemy_count = Uniform::new(1, 20);
    let direction = Uniform::new(-1.0, 1.0);
    let velocity = Uniform::new(f32::EPSILON, 50.0);
    let enemy_x = Uniform::new(HITCIRCLE_RADIUS, ARENA_WIDTH - HITCIRCLE_RADIUS);
    let enemy_y = Uniform::new(HITCIRCLE_RADIUS, ARENA_HEIGHT - HITCIRCLE_RADIUS);
    for _ in 1..=enemy_count.sample(&mut rng) {
        spawn_enemy(
            world,
            direction.sample(&mut rng) * enemy_x.sample(&mut rng),
            direction.sample(&mut rng) * enemy_y.sample(&mut rng),
            velocity.sample(&mut rng),
            velocity.sample(&mut rng),
        );
    }
}
