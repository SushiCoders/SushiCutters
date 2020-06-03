// Truncation is fine for score text
#![allow(clippy::cast_possible_truncation)]

use amethyst::{ecs::prelude::*, ui::UiText};

pub struct ScoreSystem;

use crate::components::{Collisions, Damage, Health, Player};

#[derive(Default)]
pub struct Score {
    pub player_score: u32,
}

pub struct ScoreText {
    pub player_score_entity: Entity,
}

impl ScoreText {
    pub fn format_score(score: u32) -> String {
        format!("Score: {}", score)
    }
}

impl<'s> System<'s> for ScoreSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'s, Collisions>,
        ReadStorage<'s, Damage>,
        WriteStorage<'s, Health>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, UiText>,
        Write<'s, Score>,
        ReadExpect<'s, ScoreText>,
    );

    #[allow(clippy::cast_sign_loss)] // Handled by checking sign
    fn run(
        &mut self,
        (collisions, damages, healths, players, mut ui_text, mut scores, score_text): Self::SystemData,
    ) {
        for (collision_entries, damage) in (&collisions, &damages).join() {
            // For each collision that isn't with a player with an entity with a health
            // Mask looks at just the bitmask without pulling up the actual component
            // Possibly saving some execution time
            for (_, _, _) in (collision_entries.mask(), !players.mask(), healths.mask()).join() {
                scores.player_score += if damage.amount.is_sign_negative() {
                    0
                } else {
                    damage.amount as u32
                };
                if let Some(text) = ui_text.get_mut(score_text.player_score_entity) {
                    text.text = ScoreText::format_score(scores.player_score);
                }
            }
        }
    }
}
