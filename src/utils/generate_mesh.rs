use bevy::prelude::*;
use bevy::render::mesh::{self, PrimitiveTopology};

pub fn create() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let points = vec![[0., 0., 0.], [1., 2., 1.], [2., 0., 0.]];
    let points_order = vec![0, 2, 1];

    let points_length = points.len() as usize;

    // Positions of the vertices
    // See https://bevy-cheatbook.github.io/features/coords.html
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);

    // In this example, normals and UVs don't matter,
    // so we just use the same value for all of them
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; points_length]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; points_length]);

    // A triangle using vertices 0, 2, and 1.
    // Note: order matters. [0, 1, 2] will be flipped upside down, and you won't see it from behind!
    mesh.set_indices(Some(mesh::Indices::U32(points_order)));

    return mesh;
}
