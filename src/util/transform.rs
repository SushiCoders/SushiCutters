use amethyst::core::{
    math::{Matrix4, Vector2},
    Transform,
};

pub fn global_translation(transform: &Transform) -> Vector2<f32> {
    let matrix: &Matrix4<f32> = transform.global_matrix();
    let translation = matrix.column(3);
    translation.xy()
}
