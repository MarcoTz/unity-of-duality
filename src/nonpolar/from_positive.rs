use super::formula::Formula;
use crate::positive::formula::Formula as FormulaPos;

impl From<FormulaPos> for Formula {
    fn from(pos: FormulaPos) -> Formula {
        match pos {
            FormulaPos::Atom(at) => Formula::Atom(at.val),
            FormulaPos::One => Formula::True,
            FormulaPos::NegV(f) => Formula::inv((*f).into()),
            FormulaPos::Tensor(l, r) => Formula::and((*l).into(), (*r).into()),
            FormulaPos::Zero => Formula::False,
            FormulaPos::Plus(l, r) => Formula::or((*l).into(), (*r).into()),
        }
    }
}
