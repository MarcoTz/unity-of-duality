use crate::{
    context::LinearContext,
    judgements::{Conclusion, Judgement, JudgementKind},
    negative::formula::Formula,
};

pub struct AndAbsurdRight {
    formula_left: Formula,
    formula_right: Formula,
    context_right: LinearContext,
}

impl Judgement for AndAbsurdRight {
    fn premises(&self) -> Vec<Conclusion> {
        vec![Conclusion::Absurd(
            self.context_right.clone().into(),
            self.formula_right.clone(),
        )]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Absurd(
            self.context_right.clone().into(),
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

        if form != r {
            return None;
        }

        Some(AndAbsurdRight {
            formula_left: l,
            formula_right: r,
            context_right: ctx_lin,
        })
    }
}
