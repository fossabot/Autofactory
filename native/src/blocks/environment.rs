use super::*;
use types::BlockTypes;
use std::ops::IndexMut;

pub type ExternalBlockDataStorage = HashMap<BlockLocation, BlockData>;
pub struct BlockDataAccessor<'a> { // TODO: FIX AND REPLACE BLOCKTYPE
    location: BlockLocation,
    storage: &'a BlockEnvironment,
}

impl<'a> BlockDataAccessor<'a> {
    pub fn access(&self) -> BlockData {
        self.storage[self.location]
    }

    pub fn new(location: BlockLocation, storage: &'a BlockEnvironment) -> Self {
        BlockDataAccessor {
            location,
            storage,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct BlockEnvironment {
    storage: ExternalBlockDataStorage,
}

impl BlockEnvironment {
    pub fn create_at(
        &mut self,
        position: BlockLocation,
        ty: BlockTypes,
        rotation: Rotation,
        stress: Stress,
    ) -> Block {
        let block = Block {
            block_type: ty,
            rotation,
            stress,
        };
        ty.create(block, BlockDataAccessor::new(position, &self));
        block
    }

    pub fn insert(&mut self, position: BlockLocation, data: BlockData) {
        self.storage.insert(position, data);
    }

    pub fn get(&self, position: &BlockLocation) -> BlockData {
        self.storage[position]
    }

    pub fn append_mesh(
        &self,
        (position, block): PositionedBlock,
        transform: Transform3D<f32>,
        mesh: &mut Mesh,
    ) {
        block.block_type.append_mesh(
            block,
            BlockDataAccessor::new(position, &self),
            transform,
            mesh,
        );
    }

    pub fn new() -> Self {
        BlockEnvironment {
            storage: HashMap::new(),
        }
    }
}

impl Index<BlockLocation> for BlockEnvironment {
    type Output = BlockData;
    fn index(&self, position: BlockLocation) -> &BlockData {
        &self.storage[&position]
    }
}

impl IndexMut<BlockLocation> for BlockEnvironment {
    fn index_mut(&mut self, position: BlockLocation) -> &mut BlockData {
        let entry = self.storage.entry(position);
        entry.or_insert([0; 32])
    }
}
