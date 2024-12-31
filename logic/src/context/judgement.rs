use super::{Context, LinearContext};
use crate::{
    negative::{Formula as NegativeFormula, NegativeAtom},
    positive::{Formula as PositiveFormula, PositiveAtom},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextJudgement {
    Triv(PositiveAtom),
    False(PositiveFormula),
    Absurd(NegativeAtom),
    True(NegativeFormula),
}

impl ContextJudgement {
    pub fn as_triv(self) -> Option<PositiveAtom> {
        if let ContextJudgement::Triv(at) = self {
            Some(at)
        } else {
            None
        }
    }

    pub fn as_false(self) -> Option<PositiveFormula> {
        if let ContextJudgement::False(f) = self {
            Some(f)
        } else {
            None
        }
    }

    pub fn as_absurd(self) -> Option<NegativeAtom> {
        if let ContextJudgement::Absurd(at) = self {
            Some(at)
        } else {
            None
        }
    }

    pub fn as_true(self) -> Option<NegativeFormula> {
        if let ContextJudgement::True(form) = self {
            Some(form)
        } else {
            None
        }
    }
}

impl From<ContextJudgement> for Context {
    fn from(judgement: ContextJudgement) -> Context {
        Context {
            contexts: vec![judgement.into()],
        }
    }
}

impl From<Vec<ContextJudgement>> for LinearContext {
    fn from(judgements: Vec<ContextJudgement>) -> LinearContext {
        LinearContext { judgements }
    }
}

impl From<ContextJudgement> for LinearContext {
    fn from(judgement: ContextJudgement) -> LinearContext {
        LinearContext {
            judgements: vec![judgement],
        }
    }
}

impl fmt::Display for ContextJudgement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContextJudgement::Absurd(at) => write!(f, "{at} absurd"),
            ContextJudgement::True(form) => write!(f, "{form} true"),
            ContextJudgement::Triv(at) => write!(f, "{at} triv"),
            ContextJudgement::False(at) => write!(f, "{at} false"),
        }
    }
}
