use super::*;

pub type ExternalBlockDataStorage = HashMap<BlockLocation, BlockData>;
pub struct BlockDataAccessor<'a, T> {
    location: BlockLocation,
    storage: &'a ExternalBlockDataStorage,
    _marker: PhantomData<T>,
}

impl<'a, T> BlockDataAccessor<'a, T> {
    pub fn access(&self) -> T {
        unsafe { std::mem::transmute_copy(&self.storage[&self.location]) }
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
pub struct BlockEnvironment {
    storage: ExternalBlockDataStorage,
}

impl BlockEnvironment {
    pub fn create_at<T>(
        &mut self,
        position: BlockLocation,
        ty: &dyn InitializableBlockType<T>,
        rotation: Rotation,
        stress: Stress,
    ) -> Block {
        let id = ty.id();
        let block = Block {
            block_type: id,
            rotation,
            stress,
        };
        let data = BlockTypes[id].new(block);
        self.storage.insert(position, data);
        block
    }

    pub fn append_mesh(
        &self,
        (position, block): PositionedBlock,
        transform: Transform3D<f32>,
        mesh: &mut Mesh,
    ) {
        let block_type = &BlockTypes[block.block_type];
        block_type.append_mesh(
            block,
            BlockDataAccessor::new(position, &self.storage),
            transform,
            mesh,
        )
    }

    pub fn new() -> Self {
        BlockEnvironment {
            storage: HashMap::new(),
        }
    }
}
