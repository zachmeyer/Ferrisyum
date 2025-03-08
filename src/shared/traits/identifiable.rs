//
// Identifiable
//
pub(crate) trait Identifiable {
    fn to_id(&self) -> usize;
}

//
// IdentifiableChar
//
pub(crate) trait IdentifiableChar {
    fn to_char_id(&self) -> char;
    fn from_char_id(id: char) -> Self;
}

pub(crate) trait ToIdentifiableChar {
    fn to_char_id(&self) -> char;
}
