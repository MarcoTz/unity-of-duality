use crate::{
    context::{ContextJudgement, LinearContext},
    positive::Formula as PositiveFormula,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegativeAtom {
    pub val: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Formula {
    Atom(NegativeAtom),
    Falsum,
    And(Box<Formula>, Box<Formula>),
    Truth,
    Par(Box<Formula>, Box<Formula>),
    NegN(Box<Formula>),
    Shift(Box<PositiveFormula>),
}

impl Formula {
    pub fn and(l: Formula, r: Formula) -> Formula {
        Formula::And(Box::new(l), Box::new(r))
    }

    pub fn par(l: Formula, r: Formula) -> Formula {
        Formula::Par(Box::new(l), Box::new(r))
    }

    pub fn negn(f: Formula) -> Formula {
        Formula::NegN(Box::new(f))
    }

    pub fn shift(f: PositiveFormula) -> Formula {
        Formula::Shift(Box::new(f))
    }

    pub fn as_atom(self) -> Option<NegativeAtom> {
        if let Formula::Atom(at) = self {
            Some(at)
        } else {
            None
        }
    }

    pub fn as_and(self) -> Option<(Formula, Formula)> {
        if let Formula::And(l, r) = self {
            Some((*l, *r))
        } else {
            None
        }
    }

    pub fn as_par(self) -> Option<(Formula, Formula)> {
        if let Formula::Par(l, r) = self {
            Some((*l, *r))
        } else {
            None
        }
    }

    pub fn as_neg(self) -> Option<Formula> {
        if let Formula::NegN(f) = self {
            Some(*f)
        } else {
            None
        }
    }

    pub fn as_shift(self) -> Option<PositiveFormula> {
        if let Formula::Shift(f) = self {
            Some(*f)
        } else {
            None
        }
    }

    pub fn support(self) -> Vec<LinearContext> {
        match self {
            Formula::Atom(at) => vec![ContextJudgement::Absurd(at).into()],
            Formula::Falsum => vec![LinearContext::default()],
            Formula::Par(l, r) => {
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
            Formula::Truth => vec![],
            Formula::And(l, r) => {
                let mut contexts = (*l).support();
                contexts.extend((*r).support());
                contexts
            }
            Formula::NegN(f) => vec![ContextJudgement::True(*f).into()],
            Formula::Shift(f) => vec![ContextJudgement::False(*f).into()],
        }
    }
}

impl From<NegativeAtom> for Formula {
    fn from(at: NegativeAtom) -> Formula {
        Formula::Atom(at)
    }
}

impl From<String> for Formula {
    fn from(s: String) -> Formula {
        Formula::Atom(NegativeAtom { val: s })
    }
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Formula::Atom(at) => at.fmt(f),
            Formula::Falsum => f.write_str("⊥"),
            Formula::And(l, r) => write!(f, "({l}) & ({r})"),
            Formula::Truth => f.write_str("⊤"),
            Formula::Par(l, r) => write!(f, "({l}) ⅋ ({r})"),
            Formula::NegN(f1) => write!(f, "n¬{f1}"),
            Formula::Shift(f1) => write!(f, "↑({f1})"),
        }
    }
}

impl fmt::Display for NegativeAtom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.val)
    }
}
