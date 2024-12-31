use super::{Context, ContextJudgement};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct LinearContext {
    pub judgements: Vec<ContextJudgement>,
}

impl LinearContext {
    pub fn add(self, judg: ContextJudgement) -> LinearContext {
        let mut judgements = self.judgements;
        judgements.insert(0, judg);
        LinearContext { judgements }
    }

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
