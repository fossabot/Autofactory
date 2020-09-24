use super::*;
use euclid::default::Point3D;
use ref_clone::*;
use ref_clone_derive::*;

pub trait BlockStorage {
    fn get_opt_ref<'a, T: RefType>(
        self: Ref<'a, Self, T>,
        coords: Point3D<i64>,
    ) -> Option<Ref<'a, Block, T>>
    where
        Self: Sized;

    fn get_opt(&self, coords: Point3D<i64>) -> Option<&Block>
    where
        Self: Sized,
    {
        Self::get_opt_ref(Ref::new(self), coords).map(|x| x.as_ref())
    }

    fn get_opt_mut(&mut self, coords: Point3D<i64>) -> Option<&mut Block>
    where
        Self: Sized,
    {
        Self::get_opt_ref(Ref::new(self), coords).map(|x| x.as_mut())
    }
}

pub trait InternalEnvironmentBlockStorage<'a>: BlockStorage {
    fn new(block_types: &'a BlockTypes) -> Self;
}

pub trait ExternalEnvironmentBlockStorage<'a>: BlockStorage {
    fn new(env: BlockEnvironment<'a>) -> Self;
}

pub trait UniqueEnvironmentBlockStorage: BlockStorage {
    fn get_env(&self) -> BlockEnvironment;
}

pub trait UnboundedBlockStorage: BlockStorage {
    fn get_mut<T>(&mut self, coords: Point3D<i64>) -> &mut Block;
}

pub mod chunkstorage;
pub mod octreestorage;
