use amethyst::ecs::prelude::*;

pub struct HealthIndicatorsSystem;

impl<'s> System<'s> for HealthIndicatorsSystem {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {}
}
