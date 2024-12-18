use super::{Conclusion, Judgement, JudgementKind};
use crate::{formula::Atom, linear_context::ContextJudgement};

pub struct AtomAbsurd {
    pub atom: Atom,
}

impl Judgement for AtomAbsurd {
    fn premises(&self) -> Vec<Conclusion> {
        vec![]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Absurd(
            ContextJudgement::Absurd(self.atom.clone()).into(),
            self.atom.clone().into(),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::LinearAbsurd
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if !premises.is_empty() {
            return None;
        }
        let (ctx, form) = conclusion.as_absurd()?;
        let conc_at = form.as_atom()?;
        let ctx_lin = ctx.as_linear()?;
        if ctx_lin.judgements.len() != 1 {
            return None;
        }
        let judgement = ctx_lin.judgements.first().unwrap();
        let ctx_at = judgement.clone().as_absurd()?;

        if ctx_at != conc_at {
            return None;
        }

        Some(AtomAbsurd { atom: ctx_at })
    }
}
