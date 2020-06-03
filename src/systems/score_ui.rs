// Truncation is fine for score text
#![allow(clippy::cast_possible_truncation)]

use amethyst::{ecs::prelude::*, ui::UiText};

pub struct ScoreUISystem;

use crate::components::{Collisions, Damage, Health, Player};

#[derive(Default)]
pub struct Score {
    pub player_score: u32,
}

pub struct ScoreText {
    pub player_score_entity: Entity,
}

impl ScoreText {
    pub fn format_from_str(score: u32) -> String {
        format!("Score: {}", score)
    }
}

impl<'s> System<'s> for ScoreUISystem {
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
        (collisions, damages, mut healths, players, mut ui_text, mut scores, score_text): Self::SystemData,
    ) {
        for (collision_entries, damage) in (&collisions, &damages).join() {
            for collision in &collision_entries.entries {
                if !players.contains(collision.entity) {
                    if let Some(_health) = healths.get_mut(collision.entity) {
                        scores.player_score += if damage.amount.is_sign_negative() {
                            0
                        } else {
                            damage.amount as u32
                        };
                        if let Some(text) = ui_text.get_mut(score_text.player_score_entity) {
                            text.text = ScoreText::format_from_str(scores.player_score);
                        }
                    }
                }
            }
        }
    }
}
