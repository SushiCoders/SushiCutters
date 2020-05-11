use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct KillAfterCollision;

impl Component for KillAfterCollision {
    type Storage = DenseVecStorage<Self>;
}

pub struct KillAfterTime;

impl Component for KillAfterTime {
    type Storage = DenseVecStorage<Self>;
}
