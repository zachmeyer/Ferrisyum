//
//
/// A trait encapsulating the ability to convert an object to a `usize` id
pub(crate) trait Identifiable {
    fn to_id(&self) -> usize;
}

/// A trait encapsulating the ability to generate an object from a `usize` id
/// 
/// ***Must also impl the `Identifiable` trait***
pub(crate) trait IdentifiableFrom: Identifiable {
    fn from_id(id: usize) -> Self;
}

/// A trait encapsulating the ability to convert an object both ***to** and ***from*** a `char` id
/// 
/// ***Must also impl the `ToIdentifiableChar` trait***
pub(crate) trait IdentifiableChar: ToIdentifiableChar {
    fn from_char_id(id: char) -> Self;
}

/// A trait encapsulating the ability to convert an object both ***to** and ***from*** a `char` id
pub(crate) trait ToIdentifiableChar {
    fn to_char_id(&self) -> char;
}
