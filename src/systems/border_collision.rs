use amethyst::{
    core::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{CircleCollider, Enemy, Velocity},
    sushi_cutters::{ARENA_HEIGHT, ARENA_WIDTH},
    util::transform::global_translation,
};

pub struct BorderSystem;

impl<'s> System<'s> for BorderSystem {
    type SystemData = (
        ReadStorage<'s, Enemy>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, CircleCollider>,
        ReadStorage<'s, Transform>,
    );
    fn run(&mut self, (enemies, mut velocities, colliders, transforms): Self::SystemData) {
        // Leaving enemy in as a marker trait because otherwise this might affect entities in an unintended way
        // TODO: Rewrite using colliders rather than using the arena.
        for (_, velocity, collider, transform) in
            (&enemies, &mut velocities, &colliders, &transforms).join()
        {
            let translation = global_translation(&transform);
            let enemy_x = translation.x;
            let enemy_y = translation.y;

            let (x, y) = (velocity.value.x, velocity.value.y);
            if (y.is_sign_negative() && enemy_y <= collider.radius)
                || (y.is_sign_positive() && enemy_y >= ARENA_HEIGHT - collider.radius)
            {
                velocity.value.y = -y;
            }
            if (x.is_sign_negative() && enemy_x <= collider.radius)
                || (x.is_sign_positive() && enemy_x >= ARENA_WIDTH - collider.radius)
            {
                velocity.value.x = -x;
            }
        }
    }
}
