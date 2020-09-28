use super::*;
use enum_dispatch::enum_dispatch;
use environment::*;

pub mod example;
use example::*;
pub mod vacuum;
use vacuum::*;

#[enum_dispatch]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BlockTypes {
    Example,
    Vacuum,
}

impl Default for BlockTypes {
    fn default() -> Self { Self::Vacuum(Vacuum) }
}