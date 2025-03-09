// > SUPER
use super::traits::Identifiable;

/// ## CommonState
///
/// A struct compartmentalizing possible tile states within the world.
///
/// #### Version: 0.0.1
///
/// #### Author: [Zach Meyer / SmlfrySamuri](https://github.com/zachmeyer)
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct CommonState(u8);

// IMPL
impl CommonState {
    pub const UNLOCKED: Self = Self(1);
    pub const COLLECTED: Self = Self(1);
    pub const LOCKED: Self = Self(2);
    pub const UNCOLLECTED: Self = Self(2);
}

impl Identifiable for CommonState {
    /// Implements identifiable for CommonState which returns a `usize` id
    fn to_id(&self) -> usize {
        self.0 as usize
    }
}
