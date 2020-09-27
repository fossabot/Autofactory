use super::*;
use std::ops::IndexMut;

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

#[derive(Clone, Debug, Default)]
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
        let data = BlockTypes[id].create(block);
        self.storage.insert(position, data);
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
