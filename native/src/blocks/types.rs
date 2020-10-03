use super::*;
use enum_dispatch::enum_dispatch;
use ref_clone::Unique;
pub mod example;
use example::*;
pub mod vacuum;
use vacuum::*;
pub mod test;
use test::*;

#[enum_dispatch]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BlockTypes {
    Example,
    Vacuum,
    Test,
}

impl Default for BlockTypes {
    fn default() -> Self {
        Self::Vacuum(Vacuum)
    }
}
