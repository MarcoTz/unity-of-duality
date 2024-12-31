use std::fmt;

pub mod absurd;
pub mod conclusion;
pub mod context;
pub mod contra;
pub mod fls;
pub mod shift;
pub mod triv;
pub mod tru;
pub use conclusion::Conclusion;

pub enum JudgementKind {
    LinearTriv,
    NonLinearTriv,
    False,
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
            JudgementKind::LinearTriv => f.write_str("Δ ⇒ P triv"),
            JudgementKind::NonLinearTriv => f.write_str("Γ ⊢ P triv"),
            JudgementKind::False => f.write_str("Γ ⊢ P false"),
            JudgementKind::LinearAbsurd => f.write_str("Δ ⇒ P absurd"),
            JudgementKind::NonLinearAbsurd => f.write_str("Γ ⊢ P absurd"),
            JudgementKind::True => f.write_str("Γ ⊢ P true"),
            JudgementKind::Context => f.write_str("Γ ⊢ Δ"),
            JudgementKind::Contra => f.write_str("Γ ⊢ contra"),
        }
    }
}
