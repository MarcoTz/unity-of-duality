pub struct Pos;
pub struct Neg;

pub trait Polarity {
    fn is_pos(self) -> bool;
}

impl Polarity for Pos {
    fn is_pos(self) -> bool {
        true
    }
}

impl Polarity for Neg {
    fn is_pos(self) -> bool {
        false
    }
}
