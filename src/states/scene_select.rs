use crate::scenes;

use amethyst::{
    ecs::Entity,
    prelude::*,
    ui::UiFinder,
    winit::{Event, WindowEvent},
};

use crate::resources::prefabs::UiPrefabRegistry;
use log::{info, warn};

#[derive(Default)]
pub struct SceneSelectState {
    root: Option<Entity>,
    quit: Option<Entity>,
    basic: Option<Entity>,
    enemies: Option<Entity>,
}

impl SimpleState for SceneSelectState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        info!("Please select a scene (screen must be in focus)");

        for (index, scene) in scenes::SCENES.iter().enumerate() {
            info!("{}: {}", index, scene.name);
        }
        let scene_select_prefab_resource = data
            .world
            .read_resource::<UiPrefabRegistry>()
            .find(data.world, "pause_menu");
        if let Some(scene_select_prefab) = scene_select_prefab_resource {
            println!("{:?}", scene_select_prefab);
            self.root = Some(data.world.create_entity().with(scene_select_prefab).build());
        }
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(root) = self.root {
            if data.world.delete_entity(root).is_ok() {
                self.root = None;
            }
        }
        self.basic = None;
        self.enemies = None;
        self.quit = None;
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
                            return SimpleTrans::Switch(Box::new(
                                super::running::RunningState::new(initializer),
                            ));
                        } else {
                            warn!("{} is out of bounds", num);
                        }
                    }
                }
            }
        }

        SimpleTrans::None
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(data.world);
        if self.quit.is_none() || self.basic.is_none() || self.enemies.is_none() {
            data.world.exec(|ui_finder: UiFinder<'_>| {
                self.quit = ui_finder.find("exit_button");
                self.basic = ui_finder.find("basic_button");
                self.enemies = ui_finder.find("enemy_button")
            });
        }
        Trans::None
    }
}
