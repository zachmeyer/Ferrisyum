use crate::shared::{Column, Row};

/// A trait encapsulating the ability to be positioned within the `World`
/// 
/// * `Row` as `usize`
/// * `Col` as `usize`
pub trait Positionable {
    fn row(&self) -> Row;
    fn col(&self) -> Column;
}
