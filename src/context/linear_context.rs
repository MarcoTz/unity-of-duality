use super::{Context, ContextJudgement};
use crate::{
    negative::formula::{Formula as NegativeFormula, NegativeAtom},
    positive::formula::{Formula as PositiveFormula, PositiveAtom},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LinearContext {
    pub judgements: Vec<ContextJudgement>,
}

impl LinearContext {
    pub fn append(self, other: LinearContext) -> LinearContext {
        let mut new_judgements = self.judgements;
        new_judgements.extend(other.judgements);
        LinearContext {
            judgements: new_judgements,
        }
    }

    pub fn add_triv(self, at: PositiveAtom) -> LinearContext {
        let mut new_judgements = self.judgements;
        new_judgements.push(ContextJudgement::Triv(at));
        LinearContext {
            judgements: new_judgements,
        }
    }

    pub fn add_false(self, f: PositiveFormula) -> LinearContext {
        let mut new_judgements = self.judgements;
        new_judgements.push(ContextJudgement::False(f));
        LinearContext {
            judgements: new_judgements,
        }
    }

    pub fn add_absurd(self, at: NegativeAtom) -> LinearContext {
        let mut new_judgements = self.judgements;
        new_judgements.push(ContextJudgement::Absurd(at));
        LinearContext {
            judgements: new_judgements,
        }
    }

    pub fn add_true(self, f: NegativeFormula) -> LinearContext {
        let mut new_judgements = self.judgements;
        new_judgements.push(ContextJudgement::True(f));
        LinearContext {
            judgements: new_judgements,
        }
    }
}

impl From<LinearContext> for Context {
    fn from(ctx: LinearContext) -> Context {
        Context {
            contexts: vec![ctx],
        }
    }
}

impl From<Vec<LinearContext>> for Context {
    fn from(contexts: Vec<LinearContext>) -> Context {
        Context { contexts }
    }
}

impl fmt::Display for LinearContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            &self
                .judgements
                .iter()
                .map(|judg| format!("{judg}"))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}
