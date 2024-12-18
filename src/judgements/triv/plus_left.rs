use crate::{
    context::LinearContext,
    judgements::{Conclusion, Judgement, JudgementKind},
    positive::formula::Formula,
};

pub struct PlusLeftTriv {
    left_context: LinearContext,
    left_formula: Formula,
    right_formula: Formula,
}

impl Judgement for PlusLeftTriv {
    fn premises(&self) -> Vec<Conclusion> {
        vec![Conclusion::Triv(
            self.left_context.clone().into(),
            self.left_formula.clone(),
        )]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Triv(
            self.left_context.clone().into(),
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

        if l != premise_form {
            return None;
        };

        Some(PlusLeftTriv {
            left_context: premise_ctx_lin,
            left_formula: l,
            right_formula: r,
        })
    }
}
