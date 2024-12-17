use super::formula::{Atom, Formula};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextJudgement {
    Triv(Atom),
    False(Formula),
}

impl ContextJudgement {
    pub fn as_triv(self) -> Option<Atom> {
        if let ContextJudgement::Triv(at) = self {
            Some(at)
        } else {
            None
        }
    }

    pub fn as_false(self) -> Option<Formula> {
        if let ContextJudgement::False(f) = self {
            Some(f)
        } else {
            None
        }
    }
}

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

    pub fn add_triv(self, at: Atom) -> LinearContext {
        let mut new_judgements = self.judgements;
        new_judgements.push(ContextJudgement::Triv(at));
        LinearContext {
            judgements: new_judgements,
        }
    }

    pub fn add_false(self, f: Formula) -> LinearContext {
        let mut new_judgements = self.judgements;
        new_judgements.push(ContextJudgement::False(f));
        LinearContext {
            judgements: new_judgements,
        }
    }
}

impl fmt::Display for ContextJudgement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContextJudgement::Triv(at) => write!(f, "{at} triv"),
            ContextJudgement::False(at) => write!(f, "{at} false"),
        }
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
