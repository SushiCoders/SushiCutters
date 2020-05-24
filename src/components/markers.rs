use amethyst::ecs::prelude::{Component, DenseVecStorage, NullStorage};

// This is just a marker with no data so it can be a null storage
#[derive(Default)]
pub struct KillAfterCollision;

impl Component for KillAfterCollision {
    type Storage = NullStorage<Self>;
}

pub struct KillAfterTime {
    pub time: f64,
}

impl Component for KillAfterTime {
    type Storage = DenseVecStorage<Self>;
}
