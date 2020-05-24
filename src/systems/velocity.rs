use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::Velocity;

pub struct VelocitySystem;

impl<'s> System<'s> for VelocitySystem {
    type SystemData = (
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (velocities, mut locals, time): Self::SystemData) {
        for (velocity, local) in (&velocities, &mut locals).join() {
            local.prepend_translation(velocity.value * time.delta_seconds());
        }
    }
}
