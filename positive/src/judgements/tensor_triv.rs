use super::{Conclusion, Judgement, JudgementKind};
use crate::{context::Context, formula::Formula, linear_context::LinearContext};

pub struct TensorTriv {
    context_left: LinearContext,
    formula_left: Formula,
    context_right: LinearContext,
    formula_right: Formula,
}
impl Judgement for TensorTriv {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::Triv(self.context_left.clone().into(), self.formula_left.clone()),
            Conclusion::Triv(
                self.context_right.clone().into(),
                self.formula_right.clone(),
            ),
        ]
    }
    fn conclusion(&self) -> Conclusion {
        let context_left: Context = self.context_left.clone().into();
        Conclusion::Triv(
            context_left.append(self.context_right.clone().into()),
            Formula::tensor(self.formula_left.clone(), self.formula_right.clone()),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::LinearTriv
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if premises.len() != 2 {
            return None;
        }

        let premise_left = premises.first().unwrap();
        let premise_right = premises.get(1).unwrap();

        let (ctx_left, form_left) = premise_left.clone().as_triv()?;
        let ctx_left_lin = ctx_left.as_linear()?;
        let (ctx_right, form_right) = premise_right.clone().as_triv()?;
        let ctx_right_lin = ctx_right.as_linear()?;
        let ctx_combined = ctx_left_lin.clone().append(ctx_right_lin.clone());

        let (conc_ctx, conc_form) = conclusion.as_triv()?;
        let conc_ctx_lin = conc_ctx.as_linear()?;
        let (l, r) = conc_form.as_tensor()?;

        if conc_ctx_lin != ctx_combined {
            return None;
        };
        if l != form_left {
            return None;
        };
        if r != form_right {
            return None;
        };

        Some(TensorTriv {
            context_left: ctx_left_lin,
            context_right: ctx_right_lin,
            formula_left: l,
            formula_right: r,
        })
    }
}
