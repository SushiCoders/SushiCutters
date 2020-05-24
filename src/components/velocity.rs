use amethyst::core::math::Vector3;
use amethyst::ecs::prelude::*;

pub struct Velocity {
    pub value: Vector3<f32>,
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}
