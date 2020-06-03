//! Stolen almost in it's entirety from pong
use amethyst::{
    core::{math::Vector3, Transform},
    ecs::{
        hibitset::BitSet,
        prelude::{Entities, Entity, Join, ReadStorage, System, WriteStorage},
    },
};

use crate::components::{BoxCollider, CircleCollider, CollisionData, Collisions};
use crate::util::transform::global_translation;

pub struct CollisionsSystem;

impl<'s> System<'s> for CollisionsSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, BoxCollider>,
        ReadStorage<'s, CircleCollider>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Collisions>,
    );

    fn run(&mut self, (entities, boxes, circles, transforms, mut collisions): Self::SystemData) {
        // Clear all collisions from the previous frame
        collisions.clear();

        // Create a new bitset to prevent rechecking an entity that was already checked
        let mut checked = BitSet::new();

        // Check whether a ball collided, and bounce off accordingly.
        //
        // We also check for the velocity of the ball every time, to prevent multiple collisions
        // from occurring.
        for (circle_entity, circle, circle_transform) in (&entities, &circles, &transforms).join() {
            // Add the current entity to the checked set
            checked.add(circle_entity.id());

            let translation = global_translation(circle_transform);
            let circle_x = translation.x;
            let circle_y = translation.y;

            // Bounce at the paddles.
            for (box_entity, box_col, box_transform) in (&entities, &boxes, &transforms).join() {
                let translation = global_translation(box_transform);
                let half_box = Vector3::new(box_col.width / 2_f32, box_col.height / 2_f32, 0.0);
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
                    add_collision(&mut collisions, circle_entity, box_entity);
                    add_collision(&mut collisions, box_entity, circle_entity);
                }
            }

            // Use a join exclude to exclude all other circles we have
            // already seen
            // Tested and verified to prevent double collisions
            // Also massively reduces computation time
            for (other_entity, other_circle, other_transform, _) in
                (&entities, &circles, &transforms, !&checked).join()
            {
                if other_transform != circle_transform {
                    let other_translation = global_translation(other_transform);
                    let other_radius = other_circle.radius;
                    if in_circle(
                        other_radius,
                        &other_translation,
                        circle.radius,
                        &translation,
                    ) {
                        add_collision(&mut collisions, circle_entity, other_entity);
                        add_collision(&mut collisions, other_entity, circle_entity);
                    }
                }
            }
        }
    }
}

// A point is in a box when its coordinates are smaller or equal than the top
// right and larger or equal than the bottom left.
fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}

fn in_circle(
    player_radius: f32,
    player_translation: &Vector3<f32>,
    circle_radius: f32,
    circle_translation: &Vector3<f32>,
) -> bool {
    (player_radius + circle_radius).powi(2)
        >= (player_translation - circle_translation).norm_squared()
}

/// Add a collision from one entity to another
///
/// If there is no collision component then add one with the collision
fn add_collision(collisions: &mut WriteStorage<Collisions>, source: Entity, target: Entity) {
    let component = collisions.get_mut(target);
    if let Some(c) = component {
        c.entries.push(CollisionData { entity: source });
    } else {
        collisions
            .insert(
                target,
                Collisions {
                    entries: vec![CollisionData { entity: source }],
                },
            )
            .unwrap();
    }
}
