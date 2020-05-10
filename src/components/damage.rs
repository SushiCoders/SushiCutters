use amethyst::ecs::prelude::{Component, DenseVecStorage};

/// This component is paired with a collider and inflicts
/// damage on anything that damages any entity
/// with a health component and a collider
pub struct Damage {
    pub amount: f32,
}

impl Component for Damage {
    type Storage = DenseVecStorage<Self>;
}

/// Amount of health an entity has
#[derive(Clone)]
pub struct Health {
    pub amount: f32,
}

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}
