pub mod conclusion;
pub mod value;
pub use conclusion::Conclusion;

pub enum JudgementKind {
    LinearVal,
}

pub trait Judgement: Sized {
    fn premises(&self) -> Vec<Conclusion>;
    fn conclusion(&self) -> Conclusion;
    fn kind(&self) -> JudgementKind;
    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self>;
}