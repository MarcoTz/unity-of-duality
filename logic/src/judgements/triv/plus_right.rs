use crate::{
    context::LinearContext,
    judgements::{Conclusion, Judgement, JudgementKind},
    positive::Formula,
};

pub struct PlusRightTriv {
    right_context: LinearContext,
    right_formula: Formula,
    left_formula: Formula,
}

impl Judgement for PlusRightTriv {
    fn premises(&self) -> Vec<Conclusion> {
        vec![Conclusion::Triv(
            self.right_context.clone().into(),
            self.right_formula.clone(),
        )]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Triv(
            self.right_context.clone().into(),
            Formula::plus(self.left_formula.clone(), self.right_formula.clone()),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::LinearTriv
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if premises.len() != 1 {
            return None;
        }

        let premise = premises.first().unwrap();
        let (premise_ctx, premise_form) = premise.clone().as_triv()?;
        let premise_ctx_lin = premise_ctx.as_linear()?;

        let (conc_ctx, conc_form) = conclusion.as_triv()?;
        let conc_ctx_lin = conc_ctx.as_linear()?;
        let (l, r) = conc_form.as_plus()?;

        if premise_ctx_lin != conc_ctx_lin {
            return None;
        };

        if r != premise_form {
            return None;
        };

        Some(PlusRightTriv {
            right_context: premise_ctx_lin,
            right_formula: r,
            left_formula: l,
        })
    }
}
