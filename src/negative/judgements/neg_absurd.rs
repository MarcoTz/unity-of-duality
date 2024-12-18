use super::{Conclusion, Judgement, JudgementKind};
use crate::negative::{formula::Formula, linear_context::ContextJudgement};

pub struct NegAbsurd {
    formula: Formula,
}

impl Judgement for NegAbsurd {
    fn premises(&self) -> Vec<Conclusion> {
        vec![]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Absurd(
            ContextJudgement::True(self.formula.clone()).into(),
            Formula::negn(self.formula.clone()),
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
        let ctx_lin = ctx.as_linear()?;
        if ctx_lin.judgements.len() != 1 {
            return None;
        }
        let judgement = ctx_lin.judgements.first().unwrap();
        let ctx_form = judgement.clone().as_true()?;
        let form_inner = form.as_neg()?;
        if ctx_form != form_inner {
            return None;
        }

        Some(NegAbsurd { formula: ctx_form })
    }
}
