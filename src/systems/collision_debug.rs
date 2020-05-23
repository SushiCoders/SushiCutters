use amethyst::input::InputHandler;
use amethyst::{
    core::{
        math::{geometry::Point3, Vector3},
        Transform,
    },
    ecs::prelude::{Join, Read, ReadStorage, System, Write},
    renderer::debug_drawing::DebugLines,
    renderer::palette::Srgba,
};

use crate::input::bindings::{ActionBinding, InputBindingTypes};

use crate::components::{BoxCollider, CircleCollider, Collisions};
use crate::util::transform::global_translation;

pub struct CollisionDebugState {
    show: bool,
    down: bool,
}

// This may be removed or specified in a config file
// I just want the colliders to be shown by default for now
impl Default for CollisionDebugState {
    fn default() -> Self {
        Self {
            show: true,
            down: false,
        }
    }
}

pub struct CollisionDebugSystem;

#[allow(clippy::type_complexity)]
impl<'s> System<'s> for CollisionDebugSystem {
    type SystemData = (
        ReadStorage<'s, BoxCollider>,
        ReadStorage<'s, CircleCollider>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Collisions>,
        Write<'s, DebugLines>,
        Write<'s, CollisionDebugState>,
        Read<'s, InputHandler<InputBindingTypes>>,
    );

    fn run(
        &mut self,
        (boxes, circles, transforms, collisions, mut debug, mut state, input): Self::SystemData,
    ) {
        let button_down = input
            .action_is_down(&ActionBinding::ToggleColliders)
            .unwrap();

        if button_down & !state.down {
            state.show = !state.show;
            state.down = true;
        }

        if !button_down {
            state.down = false;
        }

        // Only show the colliders if we want them to be displayed
        if !state.show {
            return;
        }

        let red = Srgba::new(0.7, 0.2, 0.2, 1.0);
        let green = Srgba::new(0.2, 0.7, 0.2, 1.0);
        for (circle, transform, collision) in (&circles, &transforms, collisions.maybe()).join() {
            let circle_point = Point3::from(global_translation(transform));

            let color = if collision.is_some() { red } else { green };

            debug.draw_circle(circle_point, circle.radius, 20, color);
        }

        for (box_collider, transform, collision) in (&boxes, &transforms, collisions.maybe()).join()
        {
            let translation = global_translation(transform);
            let half_box = Vector3::new(box_collider.width / 2f32, box_collider.height / 2f32, 0.0);

            let box_start = Point3::from(translation - half_box);
            let box_end = Point3::from(translation + half_box);

            let color = if collision.is_some() { red } else { green };

            debug.draw_box(box_start, box_end, color);
        }
    }
}
