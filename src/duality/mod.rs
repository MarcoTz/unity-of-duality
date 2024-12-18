pub mod context;
pub mod formula;
pub mod judgement;

pub trait Dual {
    type Target;
    fn dual(self) -> Self::Target;
}
