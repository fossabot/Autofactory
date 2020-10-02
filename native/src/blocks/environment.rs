use super::*;
use ref_clone::*;
use ref_clone_derive::*;
use std::collections::HashMap;
use std::ops::Index;
use std::ops::IndexMut;
use types::BlockTypes;

pub type ExternalBlockDataStorage = HashMap<BlockLocation, BlockData>;
#[derive(Debug)]
pub struct BlockDataAccessor<'a, T: RefType> {
    // TODO: FIX AND REPLACE BLOCKTYPE
    pub location: BlockLocation,
    pub storage: Ref<'a, BlockEnvironment, T>,
}

impl<'a, T: RefType> BlockDataAccessor<'a, T> {
    pub fn access_ref(self) -> Ref<'a, BlockData, T> {
        self.storage.index_ref(self.location)
    }

    pub fn new(location: BlockLocation, storage: Ref<'a, BlockEnvironment, T>) -> Self {
        BlockDataAccessor { location, storage }
    }
}

impl<'a> BlockDataAccessor<'a, Shared> {
    pub fn access(self) -> &'a BlockData {
        &self.storage.as_ref()[self.location]
    }
}

impl<'a> BlockDataAccessor<'a, Unique> {
    pub fn access(mut self) -> &'a mut BlockData {
        &mut self.storage.as_mut()[self.location]
    }

    pub fn rewrite(self, block: &mut Block, ty: BlockTypes, rotation: Rotation, stress: Stress) {
        *block = Block {
            block_type: ty,
            rotation,
            stress,
        };
        ty.create(*block, self);
    }
}

#[RefAccessors]
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
        ty.create(block, BlockDataAccessor::new(position, Ref::new(self)));
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
            BlockDataAccessor::new(position, Ref::new(self)),
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

#[allow(clippy::needless_lifetimes)]
impl IndexRef<BlockLocation> for BlockEnvironment {
    type Output = BlockData;
    fn index_ref<'a, S: RefType>(
        self: Ref<'a, Self, S>,
        position: BlockLocation,
    ) -> Ref<'a, BlockData, S> {
        // Perfectly Safe (tm) don't worry about it.
        unsafe { Ref::__new_unsafe(&self.__value().storage[&position]) }
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
