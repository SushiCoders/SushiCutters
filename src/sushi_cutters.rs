///! Core `SushiCutters` module
///! There is a bit of code that was taken from the pong example which will be phased out in time
use crate::{
    components::initialize_player, scenes, scenes::SceneInitializer, systems::score::ScoreText,
};

#[cfg(feature = "benchmark")]
use crate::util::frame_bench::FrameBench;

use amethyst::{
    assets::Loader,
    core::transform::Transform,
    ecs::prelude::*,
    prelude::*,
    renderer::Camera,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
    winit::{Event, WindowEvent},
};
use log::{info, warn};

extern crate rand;

// Maybe make these into a resource?
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub fn initialize_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

pub fn get_scene_cli() -> Option<SushiCutters> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        None
    } else {
        let scene_name = &args[1];

        if let Some(scene) = scenes::get_scene(scene_name) {
            Some(SushiCutters::new(scene.initializer))
        } else {
            panic!("`{}` is an invalid scene name!", scene_name);
        }
    }
}

#[derive(Default)]
pub struct SceneSelect;

impl SimpleState for SceneSelect {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        initialize_score(data.world);

        info!("Please select a scene (screen must be in focus)");

        for (index, scene) in scenes::SCENES.iter().enumerate() {
            info!("{}: {}", index, scene.name);
        }

        // TODO: Generate UI
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        // TODO: Clean up UI
    }

    // This is a bit messy really should be done in a different way
    // but there is no way to transition directly from on_start
    // There should be a way to not call this every frame though
    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(state) = get_scene_cli() {
            SimpleTrans::Switch(Box::new(state))
        } else {
            SimpleTrans::None
        }
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let s = scenes::SCENES;

        // TODO: Use UI instead of a keyboard input
        // Right now this is limited to only 10 scenes
        if let StateEvent::Window(event) = &event {
            if let Event::WindowEvent { ref event, .. } = event {
                if let WindowEvent::ReceivedCharacter(c) = event {
                    if let Some(num) = c.to_digit(10) {
                        let num: usize = num as usize;
                        if num < s.len() {
                            let initializer = s[num].initializer;
                            return SimpleTrans::Switch(Box::new(SushiCutters::new(initializer)));
                        } else {
                            warn!("{} is out of bounds", num);
                        }
                    }
                }
            }
        }

        SimpleTrans::None
    }
}

pub struct SushiCutters {
    initializer: Option<SceneInitializer>,

    #[cfg(feature = "benchmark")]
    frame_bench: FrameBench,
}

impl SushiCutters {
    fn new(initializer: SceneInitializer) -> Self {
        Self {
            #[cfg(feature = "benchmark")]
            frame_bench: FrameBench::default(),

            initializer: Some(initializer),
        }
    }
}

impl SimpleState for SushiCutters {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialize_camera(world);

        if let Some(init) = self.initializer {
            init(world);
        }

        initialize_player(world);
    }

    fn on_stop(&mut self, _: StateData<'_, GameData<'_, '_>>) {
        #[cfg(feature = "benchmark")]
        log::info!("{}", self.frame_bench);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        #[cfg(feature = "benchmark")]
        {
            use amethyst::core::Time;
            let time: Time = *_data.world.read_resource::<Time>();
            // We could use the absolute frametime and frame number
            // but by aggregating it ourselves we know that the counter
            // only starts counting AFTER everything is initialized
            self.frame_bench
                .advance_frame(time.delta_time().as_secs_f64());

            if time.absolute_time_seconds() > 30_f64 {
                return SimpleTrans::Quit;
            }
        }

        SimpleTrans::None
    }
}

fn initialize_score(world: &mut World) {
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
        200_f32,
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
    world.insert(ScoreText {
        player_score_entity,
    })
}
