use super::{Conclusion, Judgement, JudgementKind};
use crate::{context::Context, formula::Formula, linear_context::LinearContext};

pub struct CtxTrue {
    context: Context,
    formula: Formula,
    linear_context: LinearContext,
}

impl Judgement for CtxTrue {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::True(self.context.clone(), self.formula.clone()),
            Conclusion::Ctx(self.context.clone(), self.linear_context.clone()),
        ]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Ctx(
            self.context.clone(),
            self.linear_context.clone().add_true(self.formula.clone()),
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
        let (ctx_left, form) = premise_left.clone().as_true()?;

        let premise_right = premises.get(1).unwrap();
        let (ctx_right, ctx_lin) = premise_right.clone().as_ctx()?;

        let (conc_ctx, conc_ctx_lin) = conclusion.as_ctx()?;

        if ctx_left != ctx_right {
            return None;
        }
        if conc_ctx != ctx_left {
            return None;
        }

        if ctx_lin.clone().add_true(form.clone()) != conc_ctx_lin {
            return None;
        }
        Some(CtxTrue {
            context: conc_ctx,
            formula: form,
            linear_context: ctx_lin,
        })
    }
}
