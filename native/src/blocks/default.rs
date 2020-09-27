use super::*;
use crate::rendering::Mesh;
use environment::BlockDataAccessor;

pub struct DefaultBlockData;

pub trait DefaultBlock {
    fn get_vertices() -> &'static (Vec<Vertex>, Vec<u32>);
}

impl<T> BlockType<DefaultBlockData> for T
where
    T: DefaultBlock + Debug + Sync,
{
    fn append_mesh(
        &self,
        _: Block,
        _: BlockDataAccessor<DefaultBlockData>,
        transform: Transform3D<f32>,
        mesh: &mut Mesh,
    ) {
        let (vertex, index) = Self::get_vertices();
        let start_pos_len = mesh.positions.len();
        for x in index {
            mesh.index.push(x + start_pos_len as u32);
        }
        for a in vertex {
            let Vertex { position, normal } = a;
            mesh.positions
                .push(transform.transform_point3d(*position).unwrap());
            mesh.normals.push(*normal);
        }
    }
    fn create(&self, _: Block) -> DefaultBlockData {
        DefaultBlockData
    }
}

crate::assert_block_size!(DefaultBlockData);
