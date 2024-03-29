use amethyst::core::Time;
use amethyst::ecs::prelude::*;

use crate::components::{
    markers::{KillAfterCollision, KillAfterTime},
    Collisions,
};

pub struct KillAfterSystem;

// This system handles kill after collide and kill after time
// should be separated into two separate systems
impl<'s> System<'s> for KillAfterSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, KillAfterCollision>,
        WriteStorage<'s, KillAfterTime>,
        ReadStorage<'s, Collisions>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, ka_col, ka_time, collisions, time): Self::SystemData) {
        // Kill all entities that have been collided with and have
        // the kill after collision marker
        for (entity, _, _) in (&entities, ka_col.mask(), collisions.mask()).join() {
            // Should Handle unwrap some day
            entities.delete(entity).unwrap();
        }

        // Kill all entities that have lived their lifetime
        for (entity, timing) in (&entities, &ka_time).join() {
            if timing.time <= time.absolute_time_seconds() {
                entities.delete(entity).unwrap();
            }
        }
    }
}
