use amethyst::{
    ecs::{prelude::*, Entity},
    prelude::*,
    ui::{UiEvent, UiEventType, UiFinder},
};

use crate::{input::bindings::InputBindingTypes, resources::prefabs::UiPrefabRegistry};

#[derive(Default)]
pub struct SceneSelectState {
    root: Option<Entity>,
    quit: Option<Entity>,
    basic: Option<Entity>,
    enemies: Option<Entity>,
}

impl State<GameData<'static, 'static>, StateEvent<InputBindingTypes>> for SceneSelectState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let scene_select_prefab_resource = data
            .world
            .read_resource::<UiPrefabRegistry>()
            .find(data.world, "pause_menu");
        if let Some(scene_select_prefab) = scene_select_prefab_resource {
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
        _: StateData<GameData>,

        event: StateEvent<InputBindingTypes>,
    ) -> Trans<GameData<'static, 'static>, StateEvent<InputBindingTypes>> {
        match event {
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.quit {
                    Trans::Quit
                }
                // TODO: Handle scenes
                else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn update(
        &mut self,
        data: StateData<GameData>,
    ) -> Trans<GameData<'static, 'static>, StateEvent<InputBindingTypes>> {
        // Update the world so any UI elements are loaded before we look for them
        // Seems like root loading is lazy
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
