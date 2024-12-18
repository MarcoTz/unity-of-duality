use super::{Conclusion, Judgement, JudgementKind};
use crate::negative::{formula::Formula, linear_context::LinearContext};

pub struct AndAbsurdLeft {
    formula_left: Formula,
    formula_right: Formula,
    context_left: LinearContext,
}

impl Judgement for AndAbsurdLeft {
    fn premises(&self) -> Vec<Conclusion> {
        vec![Conclusion::Absurd(
            self.context_left.clone().into(),
            self.formula_left.clone(),
        )]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Absurd(
            self.context_left.clone().into(),
            Formula::and(self.formula_left.clone(), self.formula_right.clone()),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::LinearAbsurd
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if premises.len() != 1 {
            return None;
        }

        let premise = premises.first().unwrap();
        let (ctx, form) = premise.clone().as_absurd()?;
        let ctx_lin = ctx.as_linear()?;

        let (conc_ctx, conc_form) = conclusion.as_absurd()?;
        let conc_ctx_lin = conc_ctx.as_linear()?;
        let (l, r) = conc_form.as_and()?;

        if ctx_lin != conc_ctx_lin {
            return None;
        }

        if form != l {
            return None;
        }

        Some(AndAbsurdLeft {
            formula_left: l,
            formula_right: r,
            context_left: ctx_lin,
        })
    }
}
