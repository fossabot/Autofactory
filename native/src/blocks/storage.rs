use super::*;
use euclid::default::Point3D;
use ref_clone::*;
use ref_clone_derive::*;

pub trait BlockStorage {
    fn get_opt_ref<'a, T : RefType>(this: Ref<'a, Self, T>, coords: Point3D<i64>) -> Option<Ref<'a, Block, T>> where Self: Sized;

    fn get_opt(&self, coords: Point3D<i64>) -> Option<Block> where Self: Sized {
        Self::get_opt_ref(Ref::new(self), coords).map(|x| x.as_ref())
    }

    fn get_opt_mut(&mut self, coords: Point3D<i64>) -> Option<&mut Block> where Self: Sized {
        Self::get_opt_ref(Ref::new(self), coords).map(|x| x.as_mut())
    }

    fn get_env(&self) -> BlockEnvironment<Point3D<i64>>;
}

pub trait InternalEnvironmentBlockStorage : BlockStorage {
    fn new(block_types: &[&'static dyn BlockType<BlockData>; size_of::<BlockTypeId>()]) -> Self;
}

pub trait ExternalEnvironmentBlockStorage : BlockStorage {
    fn new(env: BlockEnvironment<Point3D<i64>>) -> Self;
}

pub trait UnboundedBlockStorage: BlockStorage {
    fn get_mut<T>(&mut self, coords: Point3D<i64>) -> &mut Block;
}

pub mod chunkstorage;
pub mod octreestorage;
