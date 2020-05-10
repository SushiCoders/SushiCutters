use amethyst::{
    core::{Time, Transform},
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
};

use crate::components::Player;
use nalgebra::Vector3;

pub struct PlayerMovementSystem;

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    // There should only really be one player but if we want the
    // player to control multiple things this could be a way to do it
    //
    // This is also a simplistic view of how movement should be handled
    // Subject to change in the future
    // The goal with this one is to get something up and running
    //
    // This should also be moved to a fixed update dispatcher
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn run(&mut self, (mut transforms, players, input, time): Self::SystemData) {
        for (transform, player) in (&mut transforms, &players).join() {
            // Custom bindings might be better for the future but right now
            // this is good enough
            // https://book.amethyst.rs/stable/input/how_to_define_custom_control_bindings.html
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
        }
    }
}
