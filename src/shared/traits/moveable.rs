// > CRATE
use crate::shared::{
    Column, Row, 
    MoveDirection, 
    TileProperties,
    traits::Positionable
};

/// A trait encapsulating the ability to move throughout the `World`
/// 
/// ***Must also impl the `Positionable` trait***
pub trait Moveable: Positionable {
    fn new_row(&self) -> Row;
    fn new_col(&self) -> Column;
    fn translate(&mut self, direction: MoveDirection) -> &Self;
    fn translate_into(&mut self);
}
