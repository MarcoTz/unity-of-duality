pub mod conclusion;
pub mod cont;
pub mod covalue;
pub mod stmt;
pub mod subst;
pub mod value;

pub use conclusion::Conclusion;

pub enum JudgementKind {
    LinearVal,
    NonLinearVal,
    LinearCoval,
    Cont,
    Subst,
    Statement,
}

pub trait Judgement: Sized {
    fn premises(&self) -> Vec<Conclusion>;
    fn conclusion(&self) -> Conclusion;
    fn kind(&self) -> JudgementKind;
    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self>;
}
