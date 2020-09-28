use super::*;
use environment::*;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(BlockTypes)]
pub trait BlockType: Debug {
    fn create(&self, block: Block, accessor: BlockDataAccessor);

    /// Appends the block's mesh to the global Mesh.
    ///
    /// All values in the block should be normalized to [-0.5 to 0.5] assuming that the translation and rotation are not applied.
    /// The translation is applied first, then the rotation.
    fn append_mesh(
        &self,
        block: Block,
        accessor: BlockDataAccessor,
        transform: Transform3D<f32>,
        mesh: &mut Mesh,
    );
}

pub trait SimpleBlockType {
    fn get_vertices() -> &'static (Vec<Vertex>, Vec<u32>);
}

impl<T> BlockType for T where T: SimpleBlockType + Debug {
    fn create(&self, _: Block, _: BlockDataAccessor) {}

    fn append_mesh(
        &self,
        _: Block,
        _: BlockDataAccessor,
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
}