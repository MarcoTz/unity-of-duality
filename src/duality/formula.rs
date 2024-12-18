use super::Dual;
use crate::{
    negative::{Formula as FormNeg, NegativeAtom},
    positive::{Formula as FormPos, PositiveAtom},
};

impl Dual for FormNeg {
    type Target = FormPos;
    fn dual(self) -> Self::Target {
        match self {
            FormNeg::Atom(at) => FormPos::Atom(at.dual()),
            FormNeg::Falsum => FormPos::Zero,
            FormNeg::And(l, r) => FormPos::plus((*l).dual(), (*r).dual()),
            FormNeg::Truth => FormPos::One,
            FormNeg::Par(l, r) => FormPos::tensor((*l).dual(), (*r).dual()),
            FormNeg::NegN(f) => FormPos::negv((*f).dual()),
            FormNeg::Shift(f) => FormPos::shift((*f).dual()),
        }
    }
}

impl Dual for FormPos {
    type Target = FormNeg;
    fn dual(self) -> Self::Target {
        match self {
            FormPos::Atom(at) => FormNeg::Atom(at.dual()),
            FormPos::One => FormNeg::Truth,
            FormPos::Tensor(l, r) => FormNeg::par((*l).dual(), (*r).dual()),
            FormPos::Zero => FormNeg::Falsum,
            FormPos::Plus(l, r) => FormNeg::and((*l).dual(), (*r).dual()),
            FormPos::NegV(f) => FormNeg::negn((*f).dual()),
            FormPos::Shift(f) => FormNeg::shift((*f).dual()),
        }
    }
}

impl Dual for PositiveAtom {
    type Target = NegativeAtom;
    fn dual(self) -> Self::Target {
        NegativeAtom { val: self.val }
    }
}

impl Dual for NegativeAtom {
    type Target = PositiveAtom;
    fn dual(self) -> Self::Target {
        PositiveAtom { val: self.val }
    }
}
