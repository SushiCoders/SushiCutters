use amethyst::ecs::prelude::*;

use crate::components::{Collisions, Damage, Health};

pub struct DamageSystem;

impl<'s> System<'s> for DamageSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Collisions>,
        ReadStorage<'s, Damage>,
        WriteStorage<'s, Health>,
    );

    /// Applies all damages from entities with damage to entities with health
    ///
    /// If an entities health reaches below zero then remove it from the game
    /// This can resonably be split into two separate systems if there are other sources
    /// Of damage
    fn run(&mut self, (entities, collisions, damages, mut healths): Self::SystemData) {
        for (collision_entries, damage) in (&collisions, &damages).join() {
            for collision in &collision_entries.entries {
                // If the collidee has a health component reduce it by damage units
                if let Some(health) = healths.get_mut(collision.entity) {
                    health.amount -= damage.amount;

                    // If the health of the target is less than 0 delet
                    if health.amount <= 0.0 {
                        println!("{:?} kicked the bucket", collision.entity);
                        entities
                            .delete(collision.entity)
                            .expect("Something wrong happened");
                    }
                }
            }
        }
    }
}
