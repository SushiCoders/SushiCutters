use amethyst::{
    core::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Enemy, Velocity},
    sushi_cutters::{ARENA_HEIGHT, ARENA_WIDTH},
};

pub struct BorderSystem;

impl<'s> System<'s> for BorderSystem {
    type SystemData = (
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Transform>,
    );
    fn run(&mut self, (mut enemies, mut velocities, transforms): Self::SystemData) {
        for (enemy, velocity, transform) in (&mut enemies, &mut velocities, &transforms).join() {
            let enemy_x = transform.translation().x;
            let enemy_y = transform.translation().y;
            let (x, y) = (velocity.velocity.x, velocity.velocity.y);
            if (y.is_sign_negative() && enemy_y <= enemy.radius)
                || (y.is_sign_positive() && enemy_y >= ARENA_HEIGHT - enemy.radius)
            {
                velocity.velocity.y = -y;
            }
            if (x.is_sign_negative() && enemy_x <= enemy.radius)
                || (x.is_sign_positive() && enemy_x >= ARENA_WIDTH - enemy.radius)
            {
                velocity.velocity.x = -x;
            }
        }
    }
}
