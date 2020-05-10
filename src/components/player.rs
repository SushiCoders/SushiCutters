use amethyst::{core::transform::Transform, ecs::prelude::*};

use crate::components::colliders;
use crate::components::Damage;

/// Component that represents a player controlled entity
/// Uses the x and y axes defined in the bindings config for movement
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
        .with(colliders::BoxCollider {
            width: 10.0,
            height: 10.0,
        })
        .with(Damage { amount: 1.0 })
        .with(Player { speed: 50.0 })
        .build();
}
