use super::Dual;
use negative::{
    context::Context as NegContext,
    linear_context::{ContextJudgement as NegJudgement, LinearContext as NegLinContext},
};
use positive::{
    context::Context as PosContext,
    linear_context::{ContextJudgement as PosJudgement, LinearContext as PosLinContext},
};

impl Dual for PosContext {
    type Target = NegContext;
    fn dual(self) -> Self::Target {
        NegContext {
            bindings: self.bindings.into_iter().map(|bnd| bnd.dual()).collect(),
        }
    }
}

impl Dual for NegContext {
    type Target = PosContext;
    fn dual(self) -> Self::Target {
        PosContext {
            bindings: self.bindings.into_iter().map(|bnd| bnd.dual()).collect(),
        }
    }
}

impl Dual for PosLinContext {
    type Target = NegLinContext;
    fn dual(self) -> Self::Target {
        NegLinContext {
            judgements: self.judgements.into_iter().map(|jdg| jdg.dual()).collect(),
        }
    }
}

impl Dual for NegLinContext {
    type Target = PosLinContext;
    fn dual(self) -> Self::Target {
        PosLinContext {
            judgements: self.judgements.into_iter().map(|jdg| jdg.dual()).collect(),
        }
    }
}

impl Dual for PosJudgement {
    type Target = NegJudgement;
    fn dual(self) -> Self::Target {
        match self {
            PosJudgement::Triv(at) => NegJudgement::Absurd(at),
            PosJudgement::False(f) => NegJudgement::True(f.dual()),
        }
    }
}

impl Dual for NegJudgement {
    type Target = PosJudgement;
    fn dual(self) -> Self::Target {
        match self {
            NegJudgement::True(f) => PosJudgement::False(f.dual()),
            NegJudgement::Absurd(at) => PosJudgement::Triv(at),
        }
    }
}
