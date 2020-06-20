// Truncation is fine for score text
#![allow(clippy::cast_possible_truncation)]

use crate::components::{Collisions, Damage, Health, Player};

use amethyst::{
    assets::Loader,
    core::SystemDesc,
    ecs::prelude::*,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

#[derive(Default)]
pub struct Score {
    pub player_score: u32,
}

struct ScoreText {
    pub player_score_entity: Entity,
}

impl ScoreText {
    fn format_score(score: u32) -> String {
        format!("Score: {}", score)
    }
}

#[derive(Default, Debug)]
pub struct ScoreSystemDesc;

// Initialize the UI entities required for the score system
impl<'a, 'b> SystemDesc<'a, 'b, ScoreSystem> for ScoreSystemDesc {
    fn build(self, world: &mut World) -> ScoreSystem {
        <ScoreSystem as System<'_>>::SystemData::setup(world);

        let font = world.read_resource::<Loader>().load(
            "fonts/FiraSans-Regular.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );
        let score_transform = UiTransform::new(
            "score".to_string(),
            Anchor::TopMiddle,
            Anchor::TopMiddle,
            0_f32,
            0_f32,
            0_f32,
            100e20_f32,
            50_f32,
        );
        let player_score_entity = world
            .create_entity()
            .with(score_transform)
            .with(UiText::new(
                font,
                ScoreText::format_score(0),
                [1_f32; 4],
                50_f32,
            ))
            .build();

        ScoreSystem {
            text: ScoreText {
                player_score_entity,
            },
        }
    }
}

pub struct ScoreSystem {
    text: ScoreText,
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
    );

    #[allow(clippy::cast_sign_loss)] // Handled by checking sign
    fn run(
        &mut self,
        (collisions, damages, healths, players, mut ui_text, mut scores): Self::SystemData,
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
                if let Some(text) = ui_text.get_mut(self.text.player_score_entity) {
                    text.text = ScoreText::format_score(scores.player_score);
                }
            }
        }
    }
}
