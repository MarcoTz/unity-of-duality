use crate::{
    context::ContextJudgement,
    judgements::{Conclusion, Judgement, JudgementKind},
    negative::Formula,
    positive::Formula as Pos,
};

pub struct ShiftNeg {
    formula: Formula,
}

impl Judgement for ShiftNeg {
    fn premises(&self) -> Vec<Conclusion> {
        vec![]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Triv(
            ContextJudgement::True(self.formula.clone()).into(),
            Pos::Shift(Box::new(self.formula.clone())),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::LinearTriv
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if !premises.is_empty() {
            return None;
        }

        let (ctx, form_pos) = conclusion.as_triv()?;
        let ctx_lin = ctx.as_linear()?;
        if ctx_lin.judgements.len() != 1 {
            return None;
        }
        let judg = ctx_lin.judgements.first().unwrap();
        let form = judg.clone().as_true()?;
        let inner = form_pos.as_shift()?;
        if form != inner {
            return None;
        }
        Some(ShiftNeg { formula: inner })
    }
}
