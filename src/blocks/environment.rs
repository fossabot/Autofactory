use super::*;
use parking_lot::MappedRwLockReadGuard;
use parking_lot::MappedRwLockWriteGuard;
use parking_lot::RwLock;
use parking_lot::RwLockReadGuard;
use parking_lot::RwLockWriteGuard;

use ref_clone::*;
use std::collections::HashMap;
use std::marker::PhantomData;
use types::BlockTypes;

type ReadLock<'a> = MappedRwLockReadGuard<'a, BlockData>;
type WriteLock<'a> = MappedRwLockWriteGuard<'a, BlockData>;

pub type ExternalBlockDataStorage = RwLock<HashMap<BlockLocation, BlockData>>;
#[derive(Debug)]
pub struct BlockDataAccessor<'a, T: RefType> {
    pub location: BlockLocation,
    pub storage: &'a BlockEnvironment,
    _marker: PhantomData<T>,
}

impl<'a, T: RefType> BlockDataAccessor<'a, T> {
    pub fn new(location: BlockLocation, storage: &'a BlockEnvironment) -> Self {
        BlockDataAccessor {
            location,
            storage,
            _marker: PhantomData,
        }
    }
}

impl<'a> BlockDataAccessor<'a, Shared> {
    pub fn access(self) -> ReadLock<'a> {
        self.storage.get(self.location)
    }
}

impl<'a> BlockDataAccessor<'a, Unique> {
    pub fn access(self) -> WriteLock<'a> {
        self.storage.get_mut(self.location)
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

    pub fn get(&self, position: BlockLocation) -> ReadLock {
        match RwLockReadGuard::try_map(self.storage.read(), |x| x.get(&position)) {
            Ok(x) => x,
            Err(_) => {
                let mut write = self.storage.write();
                write.insert(position, [0; 32]);
                RwLockReadGuard::map(self.storage.read(), |x| &x[&position])
            }
        }
    }

    pub fn get_mut(&self, position: BlockLocation) -> WriteLock {
        RwLockWriteGuard::map(self.storage.write(), |x| {
            x.entry(position).or_insert([0; 32])
        })
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
