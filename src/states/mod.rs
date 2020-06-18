use crate::scenes;
use amethyst::prelude::*;

mod running;
mod scene_select;

pub fn initial_state() -> impl SimpleState {
    LoadingState
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
pub struct LoadingState;

impl SimpleState for LoadingState {
    // This is called once to decide which scene selection method we use
    // Either scene select or load a scene from cli
    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(state) = get_scene_cli() {
            SimpleTrans::Switch(Box::new(state))
        } else {
            SimpleTrans::Switch(Box::new(scene_select::SceneSelectState))
        }
    }
}
