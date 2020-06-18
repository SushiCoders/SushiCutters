use crate::{
    components::initialize_player,
    scenes::SceneInitializer,
    sushi_cutters::{ARENA_HEIGHT, ARENA_WIDTH},
};

use amethyst::{core::transform::Transform, ecs::prelude::*, prelude::*, renderer::Camera};

#[cfg(feature = "benchmark")]
use crate::util::frame_bench::FrameBench;

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

pub struct RunningState {
    initializer: Option<SceneInitializer>,
}

impl RunningState {
    pub fn new(initializer: SceneInitializer) -> Self {
        Self {
            initializer: Some(initializer),
        }
    }
}

impl SimpleState for RunningState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        #[cfg(feature = "benchmark")]
        world.insert(FrameBench::default());

        initialize_camera(world);

        if let Some(init) = self.initializer {
            init(world);
        }

        initialize_player(world);
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        #[cfg(feature = "benchmark")]
        {
            let bench = _data.world.read_resource::<FrameBench>();
            log::info!("{}", *bench);
            bench.save_to_file().unwrap();
        }
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        #[cfg(feature = "benchmark")]
        {
            use amethyst::core::Time;
            let mut bench = _data.world.write_resource::<FrameBench>();
            let time: Time = *_data.world.read_resource::<Time>();
            // We could use the absolute frametime and frame number
            // but by aggregating it ourselves we know that the counter
            // only starts counting AFTER everything is initialized
            bench.advance_frame(time.delta_time().as_secs_f64());

            if time.absolute_time_seconds() > 30_f64 {
                return SimpleTrans::Quit;
            }
        }

        SimpleTrans::None
    }
}
