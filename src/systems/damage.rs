#![allow(clippy::cast_possible_truncation)]

// Truncation is fine for score text
use amethyst::{ecs::prelude::*, ui::UiText};

use crate::components::Player;
use crate::{
    components::{Collisions, Damage, Health},
    sushi_cutters::{Score, ScoreText},
};

pub struct DamageSystem;

impl<'s> System<'s> for DamageSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Collisions>,
        ReadStorage<'s, Damage>,
        WriteStorage<'s, Health>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, UiText>,
        Write<'s, Score>,
        ReadExpect<'s, ScoreText>,
    );

    /// Applies all damages from entities with damage to entities with health
    ///
    /// If an entities health reaches below zero then remove it from the game
    /// This can resonably be split into two separate systems if there are other sources
    /// Of damage
    fn run(
        &mut self,
        (entities, collisions, damages, mut healths, players, mut ui_text, mut scores, score_text): Self::SystemData,
    ) {
        for (collision_entries, damage) in (&collisions, &damages).join() {
            for collision in &collision_entries.entries {
                // If the collidee has a health component reduce it by damage units
                if let Some(health) = healths.get_mut(collision.entity) {
                    if !players.contains(collision.entity) {
                        scores.player_score += damage.amount as i32;
                    }
                    health.amount -= damage.amount;
                    println!(
                        "{:?} took {} damage ({} health left)",
                        collision.entity, damage.amount, health.amount
                    );
                    // If the health of the target is less than 0 then delet this
                    if health.amount <= 0.0 {
                        if !players.contains(collision.entity) {
                            scores.player_score += 10;
                        }
                        println!("{:?} kicked the bucket", collision.entity);
                        entities
                            .delete(collision.entity)
                            .expect("Something wrong happened");
                    }
                    if let Some(text) = ui_text.get_mut(score_text.player_score_entity) {
                        text.text = format!("Score: {}", scores.player_score).to_string();
                    }
                }
            }
        }
    }
}
