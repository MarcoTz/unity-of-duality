use super::Dual;
use negative::formula::Formula as FormNeg;
use positive::formula::Formula as FormPos;

impl Dual for FormNeg {
    type Target = FormPos;
    fn dual(self) -> Self::Target {
        match self {
            FormNeg::Atom(at) => FormPos::Atom(at),
            FormNeg::Falsum => FormPos::Zero,
            FormNeg::And(l, r) => FormPos::plus((*l).dual(), (*r).dual()),
            FormNeg::Truth => FormPos::One,
            FormNeg::Par(l, r) => FormPos::tensor((*l).dual(), (*r).dual()),
            FormNeg::NegN(f) => FormPos::negv((*f).dual()),
        }
    }
}

impl Dual for FormPos {
    type Target = FormNeg;
    fn dual(self) -> Self::Target {
        match self {
            FormPos::Atom(at) => FormNeg::Atom(at),
            FormPos::One => FormNeg::Truth,
            FormPos::Tensor(l, r) => FormNeg::par((*l).dual(), (*r).dual()),
            FormPos::Zero => FormNeg::Falsum,
            FormPos::Plus(l, r) => FormNeg::and((*l).dual(), (*r).dual()),
            FormPos::NegV(f) => FormNeg::negn((*f).dual()),
        }
    }
}
