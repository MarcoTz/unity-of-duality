use crate::{
    context::ContextJudgement,
    judgements::{Conclusion, Judgement, JudgementKind},
    negative::Formula as Neg,
    positive::Formula,
};

pub struct ShiftPos {
    formula: Formula,
}

impl Judgement for ShiftPos {
    fn premises(&self) -> Vec<Conclusion> {
        vec![]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Absurd(
            ContextJudgement::False(self.formula.clone()).into(),
            Neg::Shift(Box::new(self.formula.clone())),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::LinearAbsurd
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if !premises.is_empty() {
            return None;
        }
        let (ctx, form_neg) = conclusion.as_absurd()?;
        let ctx_lin = ctx.as_linear()?;
        if ctx_lin.judgements.len() != 1 {
            return None;
        }
        let judg = ctx_lin.judgements.first().unwrap();
        let form = judg.clone().as_false()?;
        let inner = form_neg.as_shift()?;
        if form != inner {
            return None;
        }
        Some(ShiftPos { formula: inner })
    }
}
