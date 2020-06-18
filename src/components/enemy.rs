use super::{CircleCollider, Health, Velocity};
use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::{
        prelude::{Component, NullStorage},
        World,
    },
    prelude::*,
};
use log::trace;

#[derive(Default)]
pub struct Enemy;
impl Component for Enemy {
    type Storage = NullStorage<Self>;
}

pub fn spawn_enemy(
    world: &mut World,
    enemy_x: f32,
    enemy_y: f32,
    x_vel: f32,
    y_vel: f32,
    size: f32,
) {
    trace!("Spawning an enemy");
    let mut t = Transform::default();
    t.set_translation_xyz(enemy_x, enemy_y, 0.0);
    world
        .create_entity()
        .with(CircleCollider { radius: size })
        .with(Enemy)
        .with(Velocity {
            value: Vector3::new(x_vel, y_vel, 0.0),
        })
        .with(Health { amount: 10.0 })
        .with(t)
        .build();
}
