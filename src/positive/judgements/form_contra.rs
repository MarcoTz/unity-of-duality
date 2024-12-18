use super::{Conclusion, Judgement, JudgementKind};
use crate::positive::{context::Context, formula::Formula, linear_context::ContextJudgement};

pub struct FormContra {
    formula: Formula,
    context: Context,
}
impl Judgement for FormContra {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::Contains(
                self.context.clone(),
                ContextJudgement::False(self.formula.clone()),
            ),
            Conclusion::Triv(self.context.clone(), self.formula.clone()),
        ]
    }
    fn conclusion(&self) -> Conclusion {
        Conclusion::Contra(self.context.clone())
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::Contra
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if premises.len() != 2 {
            return None;
        }
        let premise_left = premises.first().unwrap();
        let (ctx_left, judgement) = premise_left.clone().as_contains()?;
        let form_left = judgement.as_false()?;

        let premise_right = premises.get(1).unwrap();
        let (ctx_right, form_right) = premise_right.clone().as_triv()?;

        let conc_ctx = conclusion.as_contra()?;

        if ctx_left != ctx_right {
            return None;
        }
        if conc_ctx != ctx_left {
            return None;
        }
        if form_left != form_right {
            return None;
        }

        Some(FormContra {
            formula: form_left,
            context: conc_ctx,
        })
    }
}
