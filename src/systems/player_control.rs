use crate::components::{BoxCollider, Damage, KillAfterCollision, KillAfterTime, Player};
use crate::input::bindings::{ActionBinding, AxisBinding, InputBindingTypes};
use amethyst::{
    core::{math::Vector3, Parent, Time, Transform},
    ecs::{prelude::*, LazyUpdate},
    input::InputHandler,
};

pub struct PlayerControlSystem;

type PlayerControlSystemData<'s> = (
    // Actual system
    Entities<'s>,
    WriteStorage<'s, Transform>,
    WriteStorage<'s, Player>,
    Read<'s, InputHandler<InputBindingTypes>>,
    Read<'s, Time>,
    Read<'s, LazyUpdate>,
);

// These constants should be attached to either the player or a config file
const ATTACK_DAMAGE: f32 = 1.0;
const COLLIDER_TIMEOUT: f64 = 0.1;
const TIME_TO_ATTACK: f64 = 0.4;
const ATTACK_BOX_SIZE: [f32; 2] = [10.0, 10.0];

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
            entities,
            // Player movement and input
            mut transforms,
            mut players,
            input,
            time,
            lazy,
        ): Self::SystemData,
    ) {
        for (entity, transform, player) in (&entities, &mut transforms, &mut players).join() {
            // Custom bindings might be better for the future but right now
            // this is good enough
            // https://book.amethyst.rs/stable/input/how_to_define_custom_control_bindings.html
            let x_movement = input.axis_value(&AxisBinding::Horizontal).unwrap_or(0.);
            let y_movement = input.axis_value(&AxisBinding::Vertical).unwrap_or(0.);

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
            if let Some(attack) = input.action_is_down(&ActionBinding::Attack) {
                // If the button is down and the time to next attack is less than
                // what is in the struct
                if attack && player.next_attack <= time.absolute_time_seconds() {
                    player.next_attack = time.absolute_time_seconds() + TIME_TO_ATTACK;

                    let transform =
                        Transform::from(Vector3::from([0.0, ATTACK_BOX_SIZE[1] + 0.1, 0.0]));

                    let collider = BoxCollider {
                        width: ATTACK_BOX_SIZE[0],
                        height: ATTACK_BOX_SIZE[1],
                    };

                    let parent = Parent::new(entity);

                    // Create the swing at the end of the frame instead of trying
                    // caching it and creating it after this loop
                    lazy.create_entity(&entities)
                        .with(parent)
                        .with(transform)
                        .with(collider)
                        .with(Damage {
                            amount: ATTACK_DAMAGE,
                        })
                        .with(KillAfterCollision)
                        .with(KillAfterTime {
                            time: time.absolute_time_seconds() + COLLIDER_TIMEOUT,
                        })
                        .build();
                }
            }
        }
    }
}
