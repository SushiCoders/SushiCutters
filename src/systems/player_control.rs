use amethyst::{
    core::{math::Vector3, Parent, Time, Transform},
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
};
use std::process;

use crate::components::{BoxCollider, Damage, KillAfterCollision, KillAfterTime, Player};

pub struct PlayerControlSystem;

type PlayerControlSystemData<'s> = (
    // Actual system
    WriteStorage<'s, Transform>,
    WriteStorage<'s, Player>,
    Read<'s, InputHandler<StringBindings>>,
    Read<'s, Time>,
    // Swing generation
    Entities<'s>,
    WriteStorage<'s, BoxCollider>,
    WriteStorage<'s, Damage>,
    WriteStorage<'s, KillAfterCollision>,
    WriteStorage<'s, KillAfterTime>,
    WriteStorage<'s, Parent>,
);

// These constants should be attached to either the player or a config file
const ATTACK_DAMAGE: f32 = 1.0;
const COLLIDER_TIMEOUT: f64 = 0.1;
const TIME_TO_ATTACK: f64 = 0.4;
const ATTACK_BOX_SIZE: [f32; 2] = [10.0, 10.0];

struct Swing {
    position: Vector3<f32>,
    damage: f32,
    lifetime: f64,
    dimensions: [f32; 2],
    parent: Entity,
}

impl<'s> System<'s> for PlayerControlSystem {
    type SystemData = PlayerControlSystemData<'s>;

    // There should only really be one player but if we want the
    // player to control multiple things this could be a way to do it
    //
    // This is also a simplistic view of how movement should be handled
    // Subject to change in the future
    // The goal with this one is to get something up and running
    //
    // This should also be moved to a fixed update dispatcher
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn run(
        &mut self,
        (
            // Player movement and input
            mut transforms,
            mut players,
            input,
            time,
            // generating the swing
            entities,
            mut colliders,
            mut damages,
            mut kac,
            mut kat,
            mut parents,
        ): Self::SystemData,
    ) {
        let mut swings = Vec::new();
        for (entity, transform, player) in (&entities, &mut transforms, &mut players).join() {
            // Custom bindings might be better for the future but right now
            // this is good enough
            // https://book.amethyst.rs/stable/input/how_to_define_custom_control_bindings.html
            if let Some(should_quit) = input.action_is_down("quit"){
                should_quit && process::exit(0);
            }
            let x_movement = input.axis_value("x_axis").unwrap_or(0.);
            let y_movement = input.axis_value("y_axis").unwrap_or(0.);
            // Normalizing a vector of length 0 will result in a panic
            // Not very rusty but we have to check to make sure the movement isn't
            // (0.0, 0.0)
            if x_movement != 0.0 || y_movement != 0.0 {
                // Normalize so that diagonals aren't faster than cardinals
                // Then multiply by speed and finally by the timestep
                // To make sure the game isn't tied to the framerate
                //
                // Maybe should refactor to use try_normalize instead
                let movement = Vector3::from([x_movement, y_movement, 0.]).normalize()
                    * player.speed
                    // When this is moved over to fixed dispatch
                    // This should be time.fixed_seconds
                    * time.delta_seconds();
                transform.prepend_translation(movement);
            }

            // This has to be two nested ifs because of the way that let matching works
            if let Some(attack) = input.action_is_down("attack") {
                // If the button is down and the time to next attack is less than
                // what is in the struct
                if attack && player.next_attack <= time.absolute_time_seconds() {
                    player.next_attack = time.absolute_time_seconds() + TIME_TO_ATTACK;

                    let attack_location = Vector3::from([
                        transform.translation().x,
                        transform.translation().y + ATTACK_BOX_SIZE[1] + 0.1,
                        0.0,
                    ]);

                    // Add a new swing to the vec
                    // Can't generate a new entity while we are already joining
                    // entity and transform so that is done in a loop afterwords
                    swings.push(Swing {
                        position: attack_location,
                        damage: ATTACK_DAMAGE,
                        lifetime: COLLIDER_TIMEOUT,
                        dimensions: ATTACK_BOX_SIZE,
                        parent: entity,
                    })
                }
            }
        }

        // For each swing even generated this frame build the swing entity
        for swing in swings {
            let mut transform = Transform::default();
            transform.set_translation(swing.position);

            let collider = BoxCollider {
                width: swing.dimensions[0],
                height: swing.dimensions[1],
            };

            // Right now I can't get parent to work properly
            // Will look back into it
            let parent = Parent::new(swing.parent);

            entities
                .build_entity()
                .with(parent, &mut parents)
                .with(transform, &mut transforms)
                .with(collider, &mut colliders)
                .with(
                    Damage {
                        amount: swing.damage,
                    },
                    &mut damages,
                )
                .with(KillAfterCollision, &mut kac)
                .with(
                    KillAfterTime {
                        time: time.absolute_time_seconds() + swing.lifetime,
                    },
                    &mut kat,
                )
                .build();
        }
    }
}
