use super::{Conclusion, Judgement, JudgementKind};
use crate::{
    context::{Context, LinearContext},
    positive::formula::Formula,
};

pub struct FormTriv {
    formula: Formula,
    context: Context,
    context_premise: LinearContext,
}

impl Judgement for FormTriv {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::Triv(self.context_premise.clone().into(), self.formula.clone()),
            Conclusion::Ctx(self.context.clone(), self.context_premise.clone()),
        ]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Triv(self.context.clone(), self.formula.clone())
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::NonLinearTriv
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if premises.len() != 2 {
            return None;
        }
        let conc_left = premises.first().unwrap();
        let (ctx_left, form_left) = conc_left.clone().as_triv()?;
        let ctx_left_lin = ctx_left.as_linear()?;

        let conc_right = premises.get(1).unwrap();
        let (ctx_right, ctx_right_lin) = conc_right.clone().as_ctx()?;

        let (conc_ctx, conc_form) = conclusion.as_triv()?;

        if ctx_right_lin != ctx_left_lin {
            return None;
        }

        if form_left != conc_form {
            return None;
        }

        if conc_ctx != ctx_right {
            return None;
        }

        Some(FormTriv {
            formula: conc_form,
            context: conc_ctx,
            context_premise: ctx_right_lin,
        })
    }
}
