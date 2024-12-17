use super::{Conclusion, Judgement, JudgementKind};
use crate::{context::Context, formula::Formula, linear_context::LinearContext};

pub struct CtxFalse {
    formula: Formula,
    linear_context: LinearContext,
    context: Context,
}

impl Judgement for CtxFalse {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::False(self.context.clone(), self.formula.clone()),
            Conclusion::Ctx(self.context.clone(), self.linear_context.clone()),
        ]
    }
    fn conclusion(&self) -> Conclusion {
        Conclusion::Ctx(
            self.context.clone(),
            self.linear_context.clone().add_false(self.formula.clone()),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::Context
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if premises.len() != 2 {
            return None;
        }

        let premise_left = premises.first().unwrap();
        let (ctx_left, form_left) = premise_left.clone().as_false()?;

        let premise_right = premises.get(1).unwrap();
        let (ctx_right, ctx_right_lin) = premise_right.clone().as_ctx()?;

        let (ctx_conc, ctx_conc_lin) = conclusion.as_ctx()?;

        if ctx_left != ctx_right {
            return None;
        }
        if ctx_conc != ctx_left {
            return None;
        }

        if ctx_conc_lin != ctx_right_lin.clone().add_false(form_left.clone()) {
            return None;
        }

        Some(CtxFalse {
            context: ctx_conc,
            formula: form_left,
            linear_context: ctx_right_lin,
        })
    }
}
