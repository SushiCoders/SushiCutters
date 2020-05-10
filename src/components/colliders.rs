use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};

pub struct CircleCollider {
    pub radius: f32,
}

impl Component for CircleCollider {
    type Storage = DenseVecStorage<Self>;
}

pub struct BoxCollider {
    pub width: f32,
    pub height: f32,
}

impl Component for BoxCollider {
    type Storage = DenseVecStorage<Self>;
}

/// The Data that makes up a collision
#[derive(Debug)]
pub struct CollisionData {
    pub entity: Entity,
}

/// This component is present whenever there is a collision involving this entity
///
/// Entities is the entities that are in range of the collider
#[derive(Debug)]
pub struct Collisions {
    pub entries: Vec<CollisionData>,
}

impl Component for Collisions {
    type Storage = DenseVecStorage<Self>;
}
