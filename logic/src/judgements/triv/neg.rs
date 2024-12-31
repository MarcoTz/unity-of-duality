use crate::{
    context::ContextJudgement,
    judgements::{Conclusion, Judgement, JudgementKind},
    positive::Formula,
};

pub struct NegTriv {
    formula: Formula,
}

impl Judgement for NegTriv {
    fn premises(&self) -> Vec<Conclusion> {
        vec![]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Triv(
            ContextJudgement::False(self.formula.clone()).into(),
            Formula::negv(self.formula.clone()),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::LinearTriv
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if !premises.is_empty() {
            return None;
        }

        let (conc_ctx, conc_form) = conclusion.as_triv()?;
        let conc_ctx_lin = conc_ctx.as_linear()?;
        if conc_ctx_lin.judgements.len() != 1 {
            return None;
        }
        let ctx_judgement = conc_ctx_lin.judgements.first().unwrap();
        let ctx_form = ctx_judgement.clone().as_false()?;

        let conc_inner = conc_form.as_negv()?;
        if ctx_form == conc_inner {
            Some(NegTriv { formula: ctx_form })
        } else {
            None
        }
    }
}
