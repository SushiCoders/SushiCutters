#![allow(clippy::type_repetition_in_bounds)]

use amethyst::{
    core::{math::Vector2, Transform},
    ecs::prelude::*,
};

use crate::components::{BoxCollider, CircleCollider, CollisionData, Collisions};
use crate::util::transform::global_translation;

#[cfg(feature = "benchmark")]
use crate::util::frame_bench::FrameBench;

#[derive(Default)]
pub struct CollisionsSystem {
    collision_pool: Vec<Collisions>,
    allocator: bumpalo::Bump,
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

        // Create a new cache on the top of the prereserved memory
        let mut cache = bumpalo::collections::Vec::new_in(&self.allocator);

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

        // Pull circle data once and only once then iterate over it
        // Having the data cached is a lot cheaper than joining on it
        for (i, (circle_entity, circle, circle_translation)) in cache.iter().enumerate() {
            for (other_entity, other_circle, other_translation) in cache[i + 1..].iter() {
                if in_circle(
                    circle.radius,
                    *circle_translation,
                    other_circle.radius,
                    *other_translation,
                ) {
                    add_collision(
                        &mut self.collision_pool,
                        &mut collisions,
                        *circle_entity,
                        *other_entity,
                    );
                    add_collision(
                        &mut self.collision_pool,
                        &mut collisions,
                        *other_entity,
                        *circle_entity,
                    );
                }
            }
        }

        // Clear the cache to drop all the values
        // This will probably be optimized out since afaik since references
        // and entities don't have anything that they need to drop yet
        cache.clear();
        // Drop needs to be explicit so that we can reset the allocator pointer
        drop(cache);
        self.allocator.reset();
    }
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
