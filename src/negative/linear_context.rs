use super::formula::{Atom, Formula};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextJudgement {
    Absurd(Atom),
    True(Formula),
}

impl ContextJudgement {
    pub fn as_absurd(self) -> Option<Atom> {
        if let ContextJudgement::Absurd(at) = self {
            Some(at)
        } else {
            None
        }
    }

    pub fn as_true(self) -> Option<Formula> {
        if let ContextJudgement::True(form) = self {
            Some(form)
        } else {
            None
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
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

    pub fn add_absurd(self, at: Atom) -> LinearContext {
        let mut new_judgements = self.judgements;
        new_judgements.push(ContextJudgement::Absurd(at));
        LinearContext {
            judgements: new_judgements,
        }
    }

    pub fn add_true(self, f: Formula) -> LinearContext {
        let mut new_judgements = self.judgements;
        new_judgements.push(ContextJudgement::True(f));
        LinearContext {
            judgements: new_judgements,
        }
    }
}

impl From<ContextJudgement> for LinearContext {
    fn from(judgement: ContextJudgement) -> LinearContext {
        LinearContext {
            judgements: vec![judgement],
        }
    }
}

impl From<Vec<ContextJudgement>> for LinearContext {
    fn from(judgements: Vec<ContextJudgement>) -> LinearContext {
        LinearContext { judgements }
    }
}

impl fmt::Display for ContextJudgement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContextJudgement::Absurd(at) => write!(f, "{at} absurd"),
            ContextJudgement::True(form) => write!(f, "{form} true"),
        }
    }
}

impl fmt::Display for LinearContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            &self
                .judgements
                .iter()
                .map(|judg| format!("{}", judg))
                .collect::<Vec<String>>()
                .join(","),
        )
    }
}
