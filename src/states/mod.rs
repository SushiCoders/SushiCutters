use crate::input::bindings::InputBindingTypes;
use crate::resources::prefabs::UiPrefabRegistry;
use crate::scenes;
use amethyst::{
    assets::{Handle, ProgressCounter},
    prelude::*,
    ui::{UiLoader, UiPrefab},
    utils::application_root_dir,
};
use std::fs::read_dir;

mod running;
mod scene_select;

pub fn initial_state() -> impl State<GameData<'static, 'static>, StateEvent<InputBindingTypes>> {
    LoadingState::default()
}

fn get_scene_cli() -> Option<running::RunningState> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        None
    } else {
        let scene_name = &args[1];

        if let Some(scene) = scenes::get_scene(scene_name) {
            Some(running::RunningState::new(scene.initializer))
        } else {
            panic!("`{}` is an invalid scene name!", scene_name);
        }
    }
}

#[derive(Default)]
pub struct LoadingState {
    prefab_loading_progress: Option<ProgressCounter>,
}

impl State<GameData<'static, 'static>, StateEvent<InputBindingTypes>> for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut progress_counter = ProgressCounter::new();
        {
            let mut ui_prefab_registry = UiPrefabRegistry::default();
            ui_prefab_registry.prefabs =
                read_dir(application_root_dir().unwrap().join("assets/prefabs"))
                    .unwrap()
                    .map(|prefab_dir_entry| {
                        data.world.exec(|loader: UiLoader<'_>| {
                            loader.load(
                                {
                                    format!(
                                        "{}{}",
                                        "prefabs/",
                                        &prefab_dir_entry
                                            .unwrap()
                                            .path()
                                            .file_name()
                                            .unwrap()
                                            .to_str()
                                            .unwrap()
                                    )
                                },
                                &mut progress_counter,
                            )
                        })
                    })
                    .collect::<Vec<Handle<UiPrefab>>>();
            self.prefab_loading_progress = Some(progress_counter);
            data.world.insert(ui_prefab_registry);
        }
    }
    // This is called once to decide which scene selection method we use
    // Either scene select or load a scene from cli
    fn update(
        &mut self,
        data: StateData<GameData>,
    ) -> Trans<GameData<'static, 'static>, StateEvent<InputBindingTypes>> {
        data.data.update(data.world);
        if let Some(counter) = self.prefab_loading_progress.as_ref() {
            if counter.is_complete() {
                return if let Some(state) = get_scene_cli() {
                    Trans::Switch(Box::new(state))
                } else {
                    Trans::Switch(Box::new(scene_select::SceneSelectState::default()))
                };
            }
        }

        Trans::None
    }
}
