use super::formula::{Atom, Formula};
use std::fmt;

pub enum ContextJudgement {
    Absurd(Atom),
    True(Formula),
}

pub struct LinearContext {
    pub judgements: Vec<ContextJudgement>,
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
