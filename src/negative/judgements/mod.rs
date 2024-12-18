use std::fmt;
pub mod conclusion;
pub use conclusion::Conclusion;

pub mod and_absurd_left;
pub mod and_absurd_right;
pub mod atom_absurd;
pub mod ctx_absurd;
pub mod ctx_empty;
pub mod ctx_true;
pub mod falsum_absurd;
pub mod form_absurd;
pub mod form_contra;
pub mod form_true;
pub mod neg_absurd;
pub mod par_absurd;

pub enum JudgementKind {
    LinearAbsurd,
    NonLinearAbsurd,
    True,
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
            JudgementKind::LinearAbsurd => f.write_str("Δ ⇒ P absurd"),
            JudgementKind::NonLinearAbsurd => f.write_str("Γ ⊢ P absurd"),
            JudgementKind::True => f.write_str("Γ ⊢ P true"),
            JudgementKind::Context => f.write_str("Γ ⊢ Δ"),
            JudgementKind::Contra => f.write_str("Γ ⊢ contra"),
        }
    }
}
