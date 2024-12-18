use crate::{
    context::{ContextJudgement, LinearContext},
    negative::Formula as NegativeFormula,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositiveAtom {
    pub val: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Formula {
    Atom(PositiveAtom),
    One,
    Tensor(Box<Formula>, Box<Formula>),
    Zero,
    Plus(Box<Formula>, Box<Formula>),
    NegV(Box<Formula>),
    Shift(Box<NegativeFormula>),
}

impl Formula {
    pub fn tensor(l: Formula, r: Formula) -> Formula {
        Formula::Tensor(Box::new(l), Box::new(r))
    }

    pub fn plus(l: Formula, r: Formula) -> Formula {
        Formula::Plus(Box::new(l), Box::new(r))
    }

    pub fn negv(f: Formula) -> Formula {
        Formula::NegV(Box::new(f))
    }

    pub fn shift(f: NegativeFormula) -> Formula {
        Formula::Shift(Box::new(f))
    }

    pub fn as_atm(self) -> Option<PositiveAtom> {
        if let Formula::Atom(at) = self {
            Some(at)
        } else {
            None
        }
    }

    pub fn as_tensor(self) -> Option<(Formula, Formula)> {
        if let Formula::Tensor(l, r) = self {
            Some((*l, *r))
        } else {
            None
        }
    }

    pub fn as_negv(self) -> Option<Formula> {
        if let Formula::NegV(f) = self {
            Some(*f)
        } else {
            None
        }
    }

    pub fn as_plus(self) -> Option<(Formula, Formula)> {
        if let Formula::Plus(l, r) = self {
            Some((*l, *r))
        } else {
            None
        }
    }

    pub fn as_shift(self) -> Option<NegativeFormula> {
        if let Formula::Shift(f) = self {
            Some(*f)
        } else {
            None
        }
    }

    pub fn support(self) -> Vec<LinearContext> {
        match self {
            Formula::Atom(at) => vec![ContextJudgement::Triv(at).into()],
            Formula::One => vec![LinearContext::default()],
            Formula::Tensor(l, r) => {
                let contexts_left = (*l).support();
                let contexts_right = (*r).support();
                let mut tensor_contexts = vec![];
                for ctx_l in contexts_left.iter() {
                    for ctx_r in contexts_right.iter() {
                        tensor_contexts.push(ctx_l.clone().append(ctx_r.clone()))
                    }
                }
                tensor_contexts
            }
            Formula::Zero => vec![],
            Formula::Plus(l, r) => {
                let mut contexts = (*l).support();
                contexts.extend((*r).support());
                contexts
            }
            Formula::NegV(f) => vec![ContextJudgement::False(*f).into()],
            Formula::Shift(f) => vec![ContextJudgement::True(*f).into()],
        }
    }
}

impl From<PositiveAtom> for Formula {
    fn from(at: PositiveAtom) -> Formula {
        Formula::Atom(at)
    }
}

impl From<String> for Formula {
    fn from(s: String) -> Formula {
        Formula::Atom(PositiveAtom { val: s })
    }
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Formula::Atom(at) => at.fmt(f),
            Formula::One => f.write_str("1"),
            Formula::Tensor(f1, f2) => write!(f, "({f1}) ⊗ ({f2})"),
            Formula::Zero => f.write_str("0"),
            Formula::Plus(f1, f2) => write!(f, "({f1} ⊕ ({f2})"),
            Formula::NegV(f1) => write!(f, "v¬({f1})"),
            Formula::Shift(f1) => write!(f, "↓({f1})"),
        }
    }
}

impl fmt::Display for PositiveAtom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.val)
    }
}
