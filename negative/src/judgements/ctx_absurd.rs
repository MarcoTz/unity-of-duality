use super::{Conclusion, Judgement, JudgementKind};
use crate::{
    context::Context,
    formula::Atom,
    linear_context::{ContextJudgement, LinearContext},
};

pub struct CtxAbsurd {
    atom: Atom,
    context: Context,
    linear_context: LinearContext,
}

impl Judgement for CtxAbsurd {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::Contains(
                self.context.clone(),
                ContextJudgement::Absurd(self.atom.clone()),
            ),
            Conclusion::Ctx(self.context.clone(), self.linear_context.clone()),
        ]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Ctx(
            self.context.clone(),
            self.linear_context.clone().add_absurd(self.atom.clone()),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::Context
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if premises.len() != 2 {
            return None;
        }

        let premise_left = premises.first().unwrap();
        let (ctx_left, judg) = premise_left.clone().as_contains()?;
        let at = judg.as_absurd()?;

        let premise_right = premises.get(1).unwrap();
        let (ctx_right, ctx_lin) = premise_right.clone().as_ctx()?;

        let (conc_ctx, conc_ctx_lin) = conclusion.as_ctx()?;

        if ctx_left != ctx_right {
            return None;
        }
        if conc_ctx != ctx_left {
            return None;
        }
        if ctx_lin.clone().add_absurd(at.clone()) != conc_ctx_lin {
            return None;
        }
        Some(CtxAbsurd {
            atom: at,
            context: conc_ctx,
            linear_context: ctx_lin,
        })
    }
}
