use super::{Conclusion, Judgement, JudgementKind};
use crate::context::{Context, LinearContext};

pub struct CtxEmpty {
    context: Context,
}
impl Judgement for CtxEmpty {
    fn premises(&self) -> Vec<Conclusion> {
        vec![]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Ctx(self.context.clone(), LinearContext::default())
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::Context
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if !premises.is_empty() {
            return None;
        }
        let (ctx_left, ctx_right) = conclusion.as_ctx()?;
        if ctx_right != LinearContext::default() {
            return None;
        };

        Some(CtxEmpty { context: ctx_left })
    }
}
