#![allow(clippy::type_repetition_in_bounds)]

use amethyst::{
    core::{math::Vector2, Transform},
    ecs::prelude::*,
};
use rayon::prelude::*;
use std::mem;

use crate::components::{BoxCollider, CircleCollider, CollisionData, Collisions};
use crate::util::transform::global_translation;

#[cfg(feature = "benchmark")]
use crate::util::frame_bench::FrameBench;

type CacheRow<'s> = (Entity, &'s CircleCollider, Vector2<f32>);

const STARTING_CAPACITY: usize = 500;

#[derive(Default)]
pub struct CollisionsSystem {
    collision_pool: Vec<Collisions>,
    allocator: (usize, usize, usize),
}

impl Drop for CollisionsSystem {
    fn drop(&mut self) {
        if self.allocator.0 != 0 {
            // SAFETY: Using the parts of an existing Vec is safe
            // We allocate it once then reuse the memory that we allocated
            #[allow(unsafe_code)]
            unsafe {
                // Build a vec and let it drop itself
                Vec::from_raw_parts(
                    self.allocator.0 as *mut CacheRow,
                    self.allocator.1,
                    self.allocator.2,
                );
            }
        }
    }
}

#[derive(SystemData)]
pub struct CollisionsSystemData<'a> {
    entities: Entities<'a>,
    boxes: ReadStorage<'a, BoxCollider>,
    circles: ReadStorage<'a, CircleCollider>,
    transforms: ReadStorage<'a, Transform>,
    collisions: WriteStorage<'a, Collisions>,

    #[cfg(feature = "benchmark")]
    bench: Write<'a, FrameBench>,
}

impl<'s> System<'s> for CollisionsSystem {
    type SystemData = CollisionsSystemData<'s>;

    fn run(
        &mut self,
        CollisionsSystemData {
            entities,
            boxes,
            circles,
            transforms,
            mut collisions,

            #[cfg(feature = "benchmark")]
            mut bench,
        }: Self::SystemData,
    ) {
        // We want this to last the whole scope so we must store it as a variable
        #[cfg(feature = "benchmark")]
        let _scope = bench.time_scope("Collisions".to_string());

        // Clear all collisions from the previous frame
        // And add them into the collision pool
        for x in collisions.drain().join() {
            self.collision_pool.push(x);
        }

        // Build a new cache or reuse the old one
        let mut cache = if self.allocator.0 == 0 {
            mem::ManuallyDrop::new(Vec::with_capacity(STARTING_CAPACITY))
        } else {
            // SAFETY: Using the parts of an existing Vec is safe
            // We allocate it once then reuse the memory that we allocated
            #[allow(unsafe_code)]
            unsafe {
                mem::ManuallyDrop::new(Vec::from_raw_parts(
                    self.allocator.0 as *mut CacheRow,
                    self.allocator.1,
                    self.allocator.2,
                ))
            }
        };

        // Check whether a ball collided, and bounce off accordingly.
        //
        // We also check for the velocity of the ball every time, to prevent multiple collisions
        // from occurring.
        for (circle_entity, circle, circle_transform) in (&entities, &circles, &transforms).join() {
            let translation = global_translation(circle_transform);
            let circle_x = translation.x;
            let circle_y = translation.y;

            // Cache just the translation instead of the full transform
            cache.push((circle_entity, circle, translation));

            // Bounce at the paddles.
            for (box_entity, box_col, box_transform) in (&entities, &boxes, &transforms).join() {
                let translation = global_translation(box_transform);
                let half_box = Vector2::new(box_col.width / 2_f32, box_col.height / 2_f32);
                let top_left = translation - half_box;
                let box_x = top_left.x;
                let box_y = top_left.y;

                // To determine whether the ball has collided with a paddle, we create a larger
                // rectangle around the current one, by subtracting the ball radius from the
                // lowest coordinates, and adding the ball radius to the highest ones. The ball
                // is then within the paddle if its center is within the larger wrapper
                // rectangle.
                if point_in_rect(
                    circle_x,
                    circle_y,
                    box_x - circle.radius,
                    box_y - circle.radius,
                    box_x + box_col.width + circle.radius,
                    box_y + box_col.height + circle.radius,
                ) {
                    // Add a collision to both the circles collisions and the boxes collisions
                    add_collision(
                        &mut self.collision_pool,
                        &mut collisions,
                        circle_entity,
                        box_entity,
                    );
                    add_collision(
                        &mut self.collision_pool,
                        &mut collisions,
                        box_entity,
                        circle_entity,
                    );
                }
            }
        }

        let pool = &mut self.collision_pool;

        // Pull circle data once and only once then iterate over it
        // Having the data cached is a lot cheaper than joining on it
        cache
            .par_iter()
            .enumerate()
            .map(|(i, circle)| handle_circle_row(&cache[..], i, circle))
            .flatten()
            .collect::<Vec<(&Entity, &Entity)>>()
            .into_iter()
            .for_each(|(entity_a, entity_b)| {
                add_collision(pool, &mut collisions, *entity_a, *entity_b);
                add_collision(pool, &mut collisions, *entity_b, *entity_a);
            });

        // Clear the cache to drop all the values
        // This will probably be optimized out since afaik since references
        // and entities don't have anything that they need to drop yet
        cache.clear();

        let p = cache.as_mut_ptr() as usize;
        let len = cache.len();
        let cap = cache.capacity();

        self.allocator = (p, len, cap);
    }
}

fn handle_circle_row<'s>(
    cache: &'s [CacheRow],
    index: usize,
    (ref circle_entity, circle, circle_translation): &'s CacheRow,
) -> Vec<(&'s Entity, &'s Entity)> {
    let mut results = Vec::new();
    for (other_entity, other_circle, other_translation) in (*cache)[index + 1..].iter() {
        if in_circle(
            circle.radius,
            *circle_translation,
            other_circle.radius,
            *other_translation,
        ) {
            results.push((circle_entity, other_entity))
        }
    }

    results
}

// A point is in a box when its coordinates are smaller or equal than the top
// right and larger or equal than the bottom left.
fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}

fn in_circle(
    circle_radius: f32,
    circle_translation: Vector2<f32>,
    other_radius: f32,
    other_translation: Vector2<f32>,
) -> bool {
    (circle_radius + other_radius).powi(2)
        >= (circle_translation - other_translation).norm_squared()
}

/// Add a collision from one entity to another
///
/// If there is no collision component then add one with the collision
fn add_collision(
    pool: &mut Vec<Collisions>,
    collisions: &mut WriteStorage<Collisions>,
    source: Entity,
    target: Entity,
) {
    let component = collisions.get_mut(target);
    if let Some(c) = component {
        c.insert(source, CollisionData);
    } else {
        // If there are extra elements in the pool then reset the element
        // and use it or create a new one
        let mut c = if let Some(mut c) = pool.pop() {
            c.reset();
            c
        } else {
            Collisions::default()
        };

        c.insert(source, CollisionData);

        collisions.insert(target, c).unwrap();
    }
}
