use crate::blocks::*;
use crate::rendering::Mesh;

pub trait DefaultBlockType<T> {
    fn get_vertices() -> (Vec<Vertex>, Vec<u16>);
}

impl<T, S> BlockType<S> for T
where
    T: DefaultBlockType<S> + std::fmt::Debug,
{
    fn append_mesh(&self, _: &S, transform: Transform3D<f32>, mesh: &mut Mesh) {
        let (vertex, index) = Self::get_vertices();
        let start_pos_len = mesh.positions.len();
        for x in index {
            mesh.index.push(x + start_pos_len as u16);
        }
        for a in vertex {
            let Vertex { position, normal } = a;
            mesh.positions
                .push(transform.transform_point3d(position).unwrap());
            mesh.normals.push(normal);
        }
    }
}
