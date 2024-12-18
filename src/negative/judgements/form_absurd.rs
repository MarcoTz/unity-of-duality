use super::{Conclusion, Judgement, JudgementKind};
use crate::{
    context::{Context, LinearContext},
    negative::formula::Formula,
};

pub struct FormAbsurd {
    premise_context: LinearContext,
    premise_formula: Formula,
    context: Context,
}

impl Judgement for FormAbsurd {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::Absurd(
                self.premise_context.clone().into(),
                self.premise_formula.clone(),
            ),
            Conclusion::Ctx(self.context.clone(), self.premise_context.clone()),
        ]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Absurd(self.context.clone(), self.premise_formula.clone())
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::NonLinearAbsurd
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if premises.len() != 2 {
            return None;
        }

        let premise_left = premises.first().unwrap();
        let (ctx_left, form_left) = premise_left.clone().as_absurd()?;
        let ctx_left_lin = ctx_left.as_linear()?;

        let premise_right = premises.get(1).unwrap();
        let (ctx, ctx_lin) = premise_right.clone().as_ctx()?;

        let (conc_ctx, conc_form) = conclusion.as_absurd()?;

        if ctx_left_lin != ctx_lin {
            return None;
        }

        if ctx != conc_ctx {
            return None;
        }

        if conc_form != form_left {
            return None;
        }

        Some(FormAbsurd {
            premise_context: ctx_lin,
            premise_formula: form_left,
            context: ctx,
        })
    }
}
