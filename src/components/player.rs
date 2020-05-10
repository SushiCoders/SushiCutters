use amethyst::{core::transform::Transform, ecs::prelude::*};

use crate::components::colliders;

pub struct Player {
    pub speed: f32,
}

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}

pub fn initialize_player(world: &mut World) {
    let transform = Transform::default();

    world
        .create_entity()
        .with(transform)
        .with(colliders::CircleCollider { radius: 5.0 })
        .with(Player { speed: 50.0 })
        .build();
}
