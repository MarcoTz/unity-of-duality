use super::Dual;
use crate::context::{Context, ContextJudgement, LinearContext};

impl Dual for Context {
    type Target = Context;
    fn dual(self) -> Self::Target {
        Context {
            contexts: self.contexts.into_iter().map(|bnd| bnd.dual()).collect(),
        }
    }
}

impl Dual for LinearContext {
    type Target = LinearContext;
    fn dual(self) -> Self::Target {
        LinearContext {
            judgements: self.judgements.into_iter().map(|jdg| jdg.dual()).collect(),
        }
    }
}

impl Dual for ContextJudgement {
    type Target = ContextJudgement;
    fn dual(self) -> Self::Target {
        match self {
            ContextJudgement::Triv(at) => ContextJudgement::Absurd(at.dual()),
            ContextJudgement::False(f) => ContextJudgement::True(f.dual()),
            ContextJudgement::True(f) => ContextJudgement::False(f.dual()),
            ContextJudgement::Absurd(at) => ContextJudgement::Triv(at.dual()),
        }
    }
}
