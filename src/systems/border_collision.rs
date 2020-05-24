use amethyst::{
    core::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

use crate::{
    mob::enemy::Enemy,
    sushi_cutters::{ARENA_HEIGHT, ARENA_WIDTH},
};

pub struct BorderSystem;

impl<'s> System<'s> for BorderSystem {
    type SystemData = (WriteStorage<'s, Enemy>, ReadStorage<'s, Transform>);
    fn run(&mut self, (mut enemies, transforms): Self::SystemData) {
        for (enemy, transform) in (&mut enemies, &transforms).join() {
            let enemy_x = transform.translation().x;
            let enemy_y = transform.translation().y;
            let (x, y) = enemy.velocity;
            if (y.is_sign_negative() && enemy_y <= enemy.radius)
                || (y.is_sign_positive() && enemy_y >= ARENA_HEIGHT - enemy.radius)
            {
                enemy.velocity.1 = -y;
            }
            if (x.is_sign_negative() && enemy_x <= enemy.radius)
                || (x.is_sign_positive() && enemy_x >= ARENA_WIDTH - enemy.radius)
            {
                enemy.velocity.0 = -x;
            }
        }
    }
}
