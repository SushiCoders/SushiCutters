use amethyst::core::{
    math::{Matrix4, Vector3},
    Transform,
};

pub fn global_translation(transform: &Transform) -> Vector3<f32> {
    let matrix: &Matrix4<f32> = transform.global_matrix();
    let translation = matrix.column(3);
    translation.xyz()
}
