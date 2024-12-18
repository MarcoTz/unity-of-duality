use super::{Conclusion, Judgement, JudgementKind};
use crate::{formula::Formula, linear_context::LinearContext};

pub struct ParAbsurd {
    formula_left: Formula,
    formula_right: Formula,
    context_left: LinearContext,
    context_right: LinearContext,
}

impl Judgement for ParAbsurd {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::Absurd(self.context_left.clone().into(), self.formula_left.clone()),
            Conclusion::Absurd(
                self.context_right.clone().into(),
                self.formula_right.clone(),
            ),
        ]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Absurd(
            self.context_left
                .clone()
                .append(self.context_right.clone())
                .into(),
            Formula::par(self.formula_left.clone(), self.formula_right.clone()),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::LinearAbsurd
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if premises.len() != 2 {
            return None;
        }

        let premise_left = premises.first().unwrap();
        let (ctx_left, form_left) = premise_left.clone().as_absurd()?;
        let ctx_left_lin = ctx_left.as_linear()?;
        let premise_right = premises.get(1).unwrap();
        let (ctx_right, form_right) = premise_right.clone().as_absurd()?;
        let ctx_right_lin = ctx_right.as_linear()?;

        let (conc_ctx, conc_form) = conclusion.as_absurd()?;
        let conc_ctx_lin = conc_ctx.as_linear()?;

        if conc_ctx_lin != ctx_left_lin.clone().append(ctx_right_lin.clone()) {
            return None;
        }
        let (l, r) = conc_form.as_par()?;

        if form_left != l {
            return None;
        }

        if form_right != r {
            return None;
        }

        Some(ParAbsurd {
            formula_left: l,
            formula_right: r,
            context_left: ctx_left_lin,
            context_right: ctx_right_lin,
        })
    }
}
