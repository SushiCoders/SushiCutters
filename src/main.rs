#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, unsafe_code)]
// Amethyst prefers `Default::default()`
#![allow(clippy::default_trait_access)]
// Amethyst's fault
#![allow(clippy::multiple_crate_versions)]
// Will possibly change module structure/naming later
#![allow(clippy::module_name_repetitions)]

use amethyst::audio::AudioBundle;
use amethyst::input::InputBundle;
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod components;
mod input;
mod scenes;
mod states;
mod sushi_cutters;
mod systems;
mod util;

use crate::input::bindings::InputBindingTypes;
use crate::states::initial_state;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let bindings_path = config_dir.join("bindings.ron");

    let input_bundle =
        InputBundle::<InputBindingTypes>::new().with_bindings_from_file(bindings_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderDebugLines::default()),
        )?
        .with_bundle(input_bundle)?
        .with(
            systems::PlayerControlSystem,
            "player_control",
            &["input_system"],
        )
        .with(systems::VelocitySystem, "velocity_system", &[])
        .with_bundle(TransformBundle::new().with_dep(&["player_control", "velocity_system"]))?
        .with_bundle(UiBundle::<InputBindingTypes>::new())?
        .with_bundle(AudioBundle::default())?
        .with(
            systems::CollisionsSystem::default(),
            "collisions_system",
            &["transform_system"],
        )
        .with(
            systems::BorderSystem,
            "border_system",
            &["transform_system"],
        )
        .with(
            systems::KillAfterSystem,
            "kill_after_system",
            &["collisions_system"],
        )
        .with_system_desc(
            systems::ScoreSystemDesc,
            "score_system",
            &["collisions_system"],
        )
        .with(
            systems::DamageSystem,
            "damage_system",
            &["collisions_system"],
        )
        .with(
            systems::CollisionDebugSystem,
            "collision_debug",
            &["collisions_system", "input_system"],
        );

    let mut game = Application::build(assets_dir, initial_state())?
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 144)
        .build(game_data)?;
    game.run();

    Ok(())
}
