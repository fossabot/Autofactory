use super::*;

pub type ExternalBlockDataStorage = HashMap<BlockLocation, BlockData>;
pub struct BlockDataAccessor<'a, T> {
    location: BlockLocation,
    storage: &'a ExternalBlockDataStorage,
    _marker: PhantomData<T>,
}

impl<'a, T> BlockDataAccessor<'a, T> {
    pub fn access(&self) -> T {
        unsafe {
            std::mem::transmute_copy(&self.storage[&self.location])
        }
    }

    pub fn new(location: BlockLocation, storage: &'a ExternalBlockDataStorage) -> Self {
        BlockDataAccessor {
            location,
            storage,
            _marker: PhantomData,
        }
    }
}


#[derive(Clone, Debug)]
pub struct BlockEnvironment<'a> {
    storage: ExternalBlockDataStorage,
    block_types: &'a BlockTypes,
}

impl<'a> BlockEnvironment<'a> {
    pub fn create_at(
        &mut self,
        position: BlockLocation,
        id: BlockTypeId,
        rotation: Rotation,
        stress: Stress,
    ) -> Block {
        let block = Block {
            block_type: id,
            rotation,
            stress,
        };
        let data = self.block_types[id].new(block);
        self.storage.insert(position, data);
        block
    }

    pub fn append_mesh(
        &self,
        (position, block): PositionedBlock,
        transform: Transform3D<f32>,
        mesh: &mut Mesh,
    ) {
        let block_type = self.block_types[block.block_type];
        block_type.append_mesh(
            block,
            BlockDataAccessor::new(position, &self.storage),
            transform,
            mesh,
        )
    }

    pub fn new(block_types: &'a BlockTypes) -> Self {
        BlockEnvironment {
            storage: HashMap::new(),
            block_types,
        }
    }
}
