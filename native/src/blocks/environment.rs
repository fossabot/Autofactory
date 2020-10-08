use super::*;
use parking_lot::RwLock;
use ref_clone::*;
use ref_clone_derive::*;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Index;
use std::ops::IndexMut;
use types::BlockTypes;

pub type ExternalBlockDataStorage = RwLock<HashMap<BlockLocation, BlockData>>;
#[derive(Debug)]
pub struct BlockDataAccessor<'a, T: RefType> {
    // TODO: FIX AND REPLACE BLOCKTYPE
    pub location: BlockLocation,
    pub storage: &'a BlockEnvironment,
    _marker: PhantomData<T>,
}

impl<'a, T: RefType> BlockDataAccessor<'a, T> {
    pub fn access_ref(self) -> Ref<'a, BlockData, T> {
        unsafe { Ref::__new_unsafe(&*self.storage.get_mut(self.location)) }
    }

    pub fn location(&self) -> BlockLocation {
        self.location
    }

    pub fn new(location: BlockLocation, storage: &'a BlockEnvironment) -> Self {
        BlockDataAccessor {
            location,
            storage,
            _marker: PhantomData,
        }
    }
}

impl<'a> BlockDataAccessor<'a, Shared> {
    pub fn access(self) -> &'a BlockData {
        &self.storage[self.location]
    }
}

impl<'a> BlockDataAccessor<'a, Unique> {
    pub fn access(mut self) -> &'a mut BlockData {
        &mut self.storage[self.location]
    }

    pub fn rewrite(self, block: &mut Block, ty: BlockTypes, rotation: Rotation, stress: Stress) {
        *block = Block {
            block_type: ty,
            rotation,
            stress,
        };
        ty.create(*block, self);
    }

    pub fn create(self, block: &mut Block, ty: BlockTypes, rotation: Rotation, stress: Stress) {
        self.rewrite(block, ty, rotation, stress)
    }
}

#[RefAccessors]
#[derive(Debug, Default)]
pub struct BlockEnvironment {
    storage: ExternalBlockDataStorage,
}

impl BlockEnvironment {
    pub fn create_at(
        &self,
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
        ty.create(block, BlockDataAccessor::new(position, self));
        block
    }

    pub fn get(&self, position: BlockLocation) -> &BlockData {
        match self.storage.read().get(&position) {
            Some(x) => x,
            None => {
                let write = self.storage.write();
                write.entry(position).or_insert([0; 32])
            }
        }
    }

    pub fn get_mut(&self, position: BlockLocation) -> &mut BlockData {
        let write = self.storage.write();
        match write.get_mut(&position) {
            Some(x) => x,
            None => write.entry(position).or_insert([0; 32]),
        }
    }

    pub fn append_mesh(
        &self,
        (position, block): PositionedBlock,
        transform: Transform3D<f32>,
        mesh: &mut Mesh,
    ) {
        block.block_type.append_mesh(
            block,
            BlockDataAccessor::new(position, self),
            transform,
            mesh,
        );
    }

    pub fn new() -> Self {
        BlockEnvironment {
            storage: RwLock::new(HashMap::new()),
        }
    }
}

impl Index<BlockLocation> for BlockEnvironment {
    type Output = BlockData;
    fn index(&self, position: BlockLocation) -> &BlockData {
        self.get(position)
    }
}

impl IndexMut<BlockLocation> for BlockEnvironment {
    fn index_mut(&mut self, position: BlockLocation) -> &mut BlockData {
        self.get_mut(position)
    }
}

impl Clone for BlockEnvironment {
    fn clone(&self) -> Self {
        BlockEnvironment {
            storage: RwLock::new(self.storage.read().clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chunkstorage::*;
    use storage::*;
    use types::test::Test;
    #[test]
    fn test_data() {
        let mut storage = ChunkBlockStorage::new(BlockEnvironment::new());
        let (block, accessor) = storage.get_opt_env_mut(Point3D::new(0, 0, 0)).unwrap();
        accessor.rewrite(block, Test.into(), Default::default(), Default::default());
        let (block, accessor) = storage.get_opt_env_mut(Point3D::new(0, 0, 0)).unwrap();
        assert_eq!(block.block_type.do_thing(*block, accessor), "Test: Zero");
        let (block, accessor) = storage.get_opt_env_mut(Point3D::new(0, 0, 0)).unwrap();
        assert_eq!(block.block_type.do_thing(*block, accessor), "Test: One");
    }
}
