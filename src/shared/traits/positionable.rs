use crate::shared::{Column, Row};

pub trait Positionable {
    fn row(&self) -> Row;
    fn col(&self) -> Column;
}
