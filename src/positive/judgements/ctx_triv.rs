use super::{Conclusion, Judgement, JudgementKind};
use crate::positive::{
    context::Context,
    formula::Atom,
    linear_context::{ContextJudgement, LinearContext},
};

pub struct CtxTriv {
    atom: Atom,
    linear_context: LinearContext,
    context: Context,
}

impl Judgement for CtxTriv {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::Contains(
                self.context.clone(),
                ContextJudgement::Triv(self.atom.clone()),
            ),
            Conclusion::Ctx(self.context.clone(), self.linear_context.clone()),
        ]
    }
    fn conclusion(&self) -> Conclusion {
        Conclusion::Ctx(
            self.context.clone(),
            self.linear_context.clone().add_triv(self.atom.clone()),
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
        let (ctx_left, judgement) = premise_left.clone().as_contains()?;
        let at_triv = judgement.as_triv()?;

        let premise_right = premises.get(1).unwrap();
        let (ctx_right, ctx_linear) = premise_right.clone().as_ctx()?;

        let (conc_ctx, conc_ctx_lin) = conclusion.as_ctx()?;

        if ctx_left != ctx_right {
            return None;
        }

        if conc_ctx != ctx_left {
            return None;
        }

        if ctx_linear.clone().add_triv(at_triv.clone()) != conc_ctx_lin {
            return None;
        }

        Some(CtxTriv {
            atom: at_triv,
            linear_context: ctx_linear,
            context: conc_ctx,
        })
    }
}
