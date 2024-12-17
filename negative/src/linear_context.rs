use super::formula::{Atom, Formula};

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
