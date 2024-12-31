use super::{Context, ContextJudgement};

#[derive(Clone, PartialEq, Eq)]
pub struct LinearContext {
    pub judgements: Vec<ContextJudgement>,
}

impl LinearContext {
    pub fn as_judgement(self) -> Option<ContextJudgement> {
        if self.judgements.len() == 1 {
            self.judgements.first().cloned()
        } else {
            None
        }
    }

    pub fn combine(self, other: LinearContext) -> LinearContext {
        let mut new_judgements = self.judgements;
        new_judgements.extend(other.judgements);
        LinearContext {
            judgements: new_judgements,
        }
    }
}

impl From<LinearContext> for Context {
    fn from(lin: LinearContext) -> Context {
        Context {
            contexts: vec![lin],
        }
    }
}
