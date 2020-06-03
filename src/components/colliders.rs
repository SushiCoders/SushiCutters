use amethyst::ecs::{
    hibitset::BitSet,
    prelude::{Component, DenseVecStorage, Entity, Join},
    storage::UnprotectedStorage,
    world::Index,
};

#[derive(Clone)]
pub struct CircleCollider {
    pub radius: f32,
}

impl Component for CircleCollider {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone)]
pub struct BoxCollider {
    pub width: f32,
    pub height: f32,
}

impl Component for BoxCollider {
    type Storage = DenseVecStorage<Self>;
}

/// The Data that makes up a collision
#[derive(Debug)]
pub struct CollisionData;

/// This component is present whenever there is a collision involving this entity
///
/// Entities is the entities that are in range of the collider
pub struct Collisions {
    entries: DenseVecStorage<CollisionData>,
    bitset: BitSet,
}

impl Collisions {
    pub fn insert(&mut self, entity: Entity, data: CollisionData) {
        let id = entity.id();

        // Safety: This is safe because it is constrained by the bitset
        #[allow(unsafe_code)]
        unsafe {
            self.entries.insert(id, data);
        }
        self.bitset.add(id);
    }

    pub const fn mask(&self) -> &BitSet {
        &self.bitset
    }
}

impl Default for Collisions {
    fn default() -> Self {
        Self {
            entries: DenseVecStorage::default(),
            bitset: BitSet::new(),
        }
    }
}

impl<'s> Join for &'s Collisions {
    type Type = &'s CollisionData;
    type Value = &'s DenseVecStorage<CollisionData>;
    type Mask = &'s BitSet;

    #[allow(unsafe_code)]
    /// Safety: all insertions also modify the bitset
    unsafe fn open(self) -> (Self::Mask, Self::Value) {
        (&self.bitset, &self.entries)
    }

    #[allow(unsafe_code)]
    /// Safety: all insertions also modify the bitset
    unsafe fn get(value: &mut Self::Value, id: Index) -> Self::Type {
        value.get(id)
    }
}

impl Component for Collisions {
    type Storage = DenseVecStorage<Self>;
}
