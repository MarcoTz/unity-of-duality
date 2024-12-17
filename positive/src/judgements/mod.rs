use std::fmt;

pub mod atom_triv;
pub mod conclusion;
pub mod ctx_empty;
pub mod ctx_false;
pub mod ctx_triv;
pub mod form_contra;
pub mod form_false;
pub mod form_triv;
pub mod neg_triv;
pub mod one_triv;
pub mod plus_left_triv;
pub mod plus_right_triv;
pub mod tensor_triv;

pub use conclusion::Conclusion;

pub enum JudgementKind {
    LinearTriv,
    NonLinearTriv,
    False,
    Context,
    Contra,
}

pub trait Judgement: Sized {
    fn premises(&self) -> Vec<Conclusion>;
    fn conclusion(&self) -> Conclusion;
    fn kind(&self) -> JudgementKind;
    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self>;
}

impl fmt::Display for JudgementKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JudgementKind::LinearTriv => f.write_str("Δ ⇒ P triv"),
            JudgementKind::NonLinearTriv => f.write_str("Γ ⊢ P triv"),
            JudgementKind::False => f.write_str("Γ ⊢ P false"),
            JudgementKind::Context => f.write_str("Γ ⊢ Δ"),
            JudgementKind::Contra => f.write_str("Γ ⊢ contra"),
        }
    }
}
