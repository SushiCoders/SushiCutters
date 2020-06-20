use amethyst::{
    assets::{AssetStorage, Handle},
    ecs::World,
    prelude::WorldExt,
    ui::UiPrefab,
};

#[derive(Default)]
pub struct UiPrefabRegistry {
    pub prefabs: Vec<Handle<UiPrefab>>,
}

impl UiPrefabRegistry {
    pub fn find(&self, world: &World, name: &str) -> Option<Handle<UiPrefab>> {
        let storage = world.read_resource::<AssetStorage<UiPrefab>>();
        self.prefabs.iter().find_map(|handle| {
            if storage
                .get(handle)?
                .entities()
                .next()?
                .data()?
                .0 // transform is 0th element of UiPrefab tuple
                .as_ref()?
                .id
                == name
            {
                Some(handle.clone())
            } else {
                None
            }
        })
    }
}
