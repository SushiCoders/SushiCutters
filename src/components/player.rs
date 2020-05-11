use amethyst::{core::transform::Transform, ecs::prelude::*};

use crate::components::colliders;

/// Component that represents a player controlled entity
/// Uses the x and y axes defined in the bindings config for movement
///
/// speed is the entities movement speed
/// nextAttack is the next time the player can attack, used for attack cooldown
pub struct Player {
    pub speed: f32,
    pub next_attack: f64,
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
        .with(Player {
            speed: 50.0,
            next_attack: 0.0,
        })
        .build();
}
