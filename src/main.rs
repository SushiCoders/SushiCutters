use amethyst::audio::AudioBundle;
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod components;
mod sushi_cutters;
mod systems;

use crate::sushi_cutters::SushiCutters;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let bindings_path = config_dir.join("bindings.ron");

    use amethyst::input::{InputBundle, StringBindings};
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_path)?;

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
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with(
            systems::PlayerControlSystem,
            "player_control",
            &["input_system"],
        )
        .with(
            systems::CollisionsSystem,
            "collisions_system",
            &["player_control"],
        )
        .with(
            systems::KillAfterSystem,
            "kill_after_system",
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

    let mut game = Application::new(assets_dir, SushiCutters::default(), game_data)?;
    game.run();

    Ok(())
}
