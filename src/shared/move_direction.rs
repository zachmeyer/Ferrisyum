// > SUPER
use super::extlib::NAMatrix3;

/// The row (y) direction within the grid as `i64`
type MoveRow = i64;

/// The col (x) direction within the grid as `i64`
type MoveCol = i64;

/// The speed of movement as an `Option<i64>`
type MoveSpeed = Option<i64>;

/// ## MoveDirection
///
/// A tuple struct indicating the `MoveRow` and `MoveCol` direction, as well as the `MoveSpeed`
///
/// #### Version: 0.0.1
///
/// #### Author: [Zach Meyer / SmlfrySamuri](https://github.com/zachmeyer)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveDirection(MoveRow, MoveCol, MoveSpeed);

impl MoveDirection {
    pub const UP: Self = Self(-1, 0, Some(1));
    pub const DOWN: Self = Self(1, 0, Some(1));
    pub const LEFT: Self = Self(0, -1, Some(1));
    pub const RIGHT: Self = Self(0, 1, Some(1));

    pub fn translate(&self) -> NAMatrix3<i64> {
        let ty: i64;
        let tx: i64;

        if let Some(spd) = self.2 {
            ty = self.0 * spd;
            tx = self.1 * spd;
        } else {
            ty = self.0;
            tx = self.1;
        }

        NAMatrix3::new(1, 0, tx, 0, 1, ty, 0, 0, 1)
    }
}
