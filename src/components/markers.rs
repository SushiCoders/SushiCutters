use amethyst::ecs::prelude::{Component, DenseVecStorage, NullStorage};

// This is just a marker with no data so it can be a null storage
#[derive(Default)]
pub struct KillAfterCollision;

impl Component for KillAfterCollision {
    type Storage = NullStorage<Self>;
}

// Timings should probably be using durations
// But I didn't feel like learning something new
pub struct KillAfterTime {
    pub time: f64,
}

impl Component for KillAfterTime {
    type Storage = DenseVecStorage<Self>;
}
