use bevy::{
    math::Vec3,
    render::{
        color::Color, mesh::{Indices, Mesh},
        render_resource::PrimitiveTopology,
    },
    transform::components::Transform,
};
use lyon_tessellation::{self, FillVertex, FillVertexConstructor, StrokeVertex, StrokeVertexConstructor};

use crate::Convert;


/// A vertex with all the necessary attributes to be inserted into a Bevy
/// [`Mesh`](bevy::render::mesh::Mesh).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

/// The index type of a Bevy [`Mesh`](bevy::render::mesh::Mesh).
pub(crate) type IndexType = u32;

/// Lyon's [`VertexBuffers`] generic data type defined for [`Vertex`].
pub(crate) type VertexBuffers = lyon_tessellation::VertexBuffers<Vertex, IndexType>;

impl Convert<Mesh> for VertexBuffers {
    fn convert(self) -> Mesh {
        let mut positions = Vec::with_capacity(self.vertices.len());
        let mut colors = Vec::with_capacity(self.vertices.len());

        self.vertices.iter().for_each(|v| {
            positions.push(v.position);
            colors.push(v.color);
        });

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(self.indices.clone())));
        mesh.set_attribute(
            Mesh::ATTRIBUTE_POSITION,
            positions
        );
        mesh.set_attribute(
            Mesh::ATTRIBUTE_COLOR,
            colors
        );

        mesh
    }
}

/// Zero-sized type used to implement various vertex construction traits from Lyon.
pub(crate) struct VertexConstructor {
    pub(crate) color: Color,
    pub(crate) transform: Transform,
}

/// Enables the construction of a [`Vertex`] when using a `FillTessellator`.
impl FillVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        let vertex = vertex.position();
        let pos = self.transform * Vec3::new(
            vertex.x,
            vertex.y,
            0.0,
        );

        Vertex {
            position: [pos.x, pos.y, pos.z],
            color: self.color.as_linear_rgba_f32(),
        }
    }
}

/// Enables the construction of a [`Vertex`] when using a `StrokeTessellator`.
impl StrokeVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        let vertex = vertex.position();
        let pos = self.transform * Vec3::new(
            vertex.x,
            vertex.y,
            0.0,
        );

        Vertex {
            position: [pos.x, pos.y, pos.z],
            color: self.color.as_linear_rgba_f32(),
        }
    }
}

pub(crate) trait BufferExt<A> {
    fn extend_one(&mut self, item: A);
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T);
}

impl BufferExt<VertexBuffers> for VertexBuffers {

    fn extend_one(&mut self, item: VertexBuffers) {
        let offset = self.vertices.len() as u32;

        self.vertices.extend(&item.vertices);
        self.indices.extend(item.indices.iter().map(|i| i + offset));
    }

    fn extend<T: IntoIterator<Item = VertexBuffers>>(&mut self, iter: T) {
        let mut offset = self.vertices.len() as u32;

        for buf in iter {
            self.vertices.extend(&buf.vertices);
            self.indices.extend(buf.indices.iter().map(|i| i + offset));

            offset += buf.vertices.len() as u32;
        }
    }
}