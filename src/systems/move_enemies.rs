use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::mob::enemy::Enemy;

#[derive(SystemDesc)]
pub struct MoveEnemiesSystem;

impl<'s> System<'s> for MoveEnemiesSystem {
    type SystemData = (
        ReadStorage<'s, Enemy>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (enemy, mut locals, time): Self::SystemData) {
        for (enemy, local) in (&enemy, &mut locals).join() {
            let (x, y) = enemy.velocity;
            local.prepend_translation_x((x + f32::EPSILON) * time.delta_seconds());
            local.prepend_translation_y((y + f32::EPSILON) * time.delta_seconds());
        }
    }
}
