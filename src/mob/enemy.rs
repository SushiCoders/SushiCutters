use crate::{components::Health, mob::HITCIRCLE_RADIUS};
use amethyst::{
    core::transform::Transform,
    ecs::{
        prelude::{Component, DenseVecStorage},
        World,
    },
    prelude::*,
};
pub struct Enemy {
    pub radius: f32,
    pub velocity: (f32, f32),
}
impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}

pub fn spawn_enemy(world: &mut World, enemy_x: f32, enemy_y: f32, x_vel: f32, y_vel: f32) {
    println!("Spawning an enemy");
    let mut t = Transform::default();
    t.set_translation_xyz(enemy_x, enemy_y, 0.0);
    world
        .create_entity()
        .with(Enemy {
            radius: HITCIRCLE_RADIUS,
            velocity: (x_vel, y_vel),
        })
        .with(Health { amount: 10.0 })
        .with(t)
        .build();
}
