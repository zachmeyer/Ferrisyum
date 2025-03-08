use super::traits::Identifiable;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct CommonState(u8);

impl CommonState {
    pub const UNLOCKED: Self = Self(1);
    pub const COLLECTED: Self = Self(1);
    pub const LOCKED: Self = Self(2);
    pub const UNCOLLECTED: Self = Self(2);
}

impl Identifiable for CommonState {
    fn to_id(&self) -> usize {
        self.0 as usize
    }
}
