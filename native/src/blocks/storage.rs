use super::*;
use ref_clone::*;
use ref_clone_derive::*;

pub trait BlockStorage {
    fn get_opt_env_ref<'a, T: RefType>(
        self: Ref<'a, Self, T>,
        coords: Point3D<i64>,
    ) -> Option<(Ref<'a, Block, T>, BlockDataAccessor<'a, T>)>;

    fn get_opt_env(&self, coords: Point3D<i64>) -> Option<(&Block, BlockDataAccessor<Shared>)> {
        Ref::new(self)
            .get_opt_env_ref(coords)
            .map(|x| (x.0.as_ref(), x.1))
    }

    fn get_opt_env_mut(
        &mut self,
        coords: Point3D<i64>,
    ) -> Option<(&mut Block, BlockDataAccessor<Unique>)> {
        Ref::new(self)
            .get_opt_env_ref(coords)
            .map(|mut x| (x.0.as_mut(), x.1))
    }

    fn get_opt_ref<'a, T: RefType>(
        self: Ref<'a, Self, T>,
        coords: Point3D<i64>,
    ) -> Option<Ref<'a, Block, T>> {
        self.get_opt_env_ref(coords).map(|x| x.0)
    }

    fn get_opt(&self, coords: Point3D<i64>) -> Option<&Block> {
        Self::get_opt_ref(Ref::new(self), coords).map(|x| x.as_ref())
    }

    fn get_opt_mut(&mut self, coords: Point3D<i64>) -> Option<&mut Block> {
        Self::get_opt_ref(Ref::new(self), coords).map(|mut x| x.as_mut())
    }

    type Iter<'a, T: RefType>: Iterator<Item = (BlockDataAccessor<'a, T>, Ref<'a, Block, T>)>;

    fn iter_ref<'a, T: RefType>(self: Ref<'a, Self, T>) -> Self::Iter<'a, T>;

    fn iter<'a>(&'a self) -> StorageIterator<'a, Shared, Self::Iter<'a, Shared>> {
        StorageIterator(Ref::new(self).iter_ref())
    }

    fn iter_mut<'a>(&'a mut self) -> StorageIterator<'a, Unique, Self::Iter<'a, Unique>> {
        StorageIterator(Ref::new(self).iter_ref())
    }
}

#[repr(transparent)]
pub struct StorageIterator<'a, T: RefType, S>(S)
where
    S: Iterator<Item = (BlockDataAccessor<'a, T>, Ref<'a, Block, T>)>;

impl<'a, S> Iterator for StorageIterator<'a, Shared, S>
where
    S: Iterator<Item = (BlockDataAccessor<'a, Shared>, Ref<'a, Block, Shared>)>,
{
    type Item = (BlockDataAccessor<'a, Shared>, &'a Block);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| (x.0, x.1.as_ref()))
    }
}

impl<'a, S> Iterator for StorageIterator<'a, Unique, S>
where
    S: Iterator<Item = (BlockDataAccessor<'a, Unique>, Ref<'a, Block, Unique>)>,
{
    type Item = (BlockDataAccessor<'a, Unique>, &'a mut Block);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| (x.0, x.1.as_mut()))
    }
}

pub trait InternalEnvironmentBlockStorage: BlockStorage {
    fn new() -> Self;
}

pub trait ExternalEnvironmentBlockStorage: BlockStorage {
    fn new(env: BlockEnvironment) -> Self;
}

#[allow(clippy::needless_lifetimes)] // Lifetimes are needed.
pub trait UniqueEnvironmentBlockStorage: BlockStorage {
    fn get_env_ref<'a, T: RefType>(self: Ref<'a, Self, T>) -> Ref<'a, BlockEnvironment, T>;

    fn get_env(&self) -> &BlockEnvironment {
        Ref::new(self).get_env_ref().as_ref()
    }

    fn get_env_mut(&mut self) -> &mut BlockEnvironment {
        Ref::new(self).get_env_ref().as_mut()
    }
}

pub trait UnboundedBlockStorage: BlockStorage {
    fn get_mut(&mut self, coords: Point3D<i64>) -> (&mut Block, BlockDataAccessor<Unique>);
}

pub mod chunkstorage;
pub mod octreestorage;
