use amethyst::input::{InputHandler, StringBindings};
use amethyst::{
    core::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, Write},
    renderer::debug_drawing::DebugLines,
    renderer::palette::Srgba,
};

use nalgebra::geometry::Point3;

use crate::components::{BoxCollider, CircleCollider, Collisions};

pub struct CollisionDebugState {
    show: bool,
    down: bool,
}

impl Default for CollisionDebugState {
    fn default() -> Self {
        Self {
            show: true,
            down: false,
        }
    }
}

pub struct CollisionDebugSystem;

impl<'s> System<'s> for CollisionDebugSystem {
    type SystemData = (
        ReadStorage<'s, BoxCollider>,
        ReadStorage<'s, CircleCollider>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Collisions>,
        Write<'s, DebugLines>,
        Write<'s, CollisionDebugState>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (boxes, circles, transforms, collisions, mut debug, mut state, input): Self::SystemData,
    ) {
        let button_down = input.action_is_down("toggle_colliders").unwrap();

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
            let circle_x = transform.translation().x;
            let circle_y = transform.translation().y;
            let circle_point = Point3::new(circle_x, circle_y, 0.0);

            let color = if let Some(_) = collision { red } else { green };

            debug.draw_circle(circle_point, circle.radius, 20, color);
        }

        for (box_collider, transform, collision) in (&boxes, &transforms, collisions.maybe()).join()
        {
            let box_x = transform.translation().x - box_collider.width / 2f32;
            let box_y = transform.translation().y - box_collider.height / 2f32;
            let box_start = Point3::new(box_x, box_y, 0.0);
            let box_end = Point3::new(box_x + box_collider.width, box_y + box_collider.height, 0.0);

            let color = if let Some(_) = collision { red } else { green };

            debug.draw_box(box_start, box_end, color);
        }
    }
}
