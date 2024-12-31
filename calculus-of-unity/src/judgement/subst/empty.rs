use crate::{
    context::{Context, LinearContext},
    judgement::{Conclusion, Judgement, JudgementKind},
    substitution::Substitution,
};
pub struct SubstEmpty {
    context: Context,
}

impl Judgement for SubstEmpty {
    fn premises(&self) -> Vec<Conclusion> {
        vec![]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Subst(
            self.context.clone(),
            Substitution::default(),
            LinearContext::default(),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::Subst
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if !premises.is_empty() {
            return None;
        }

        let (ctx, subst, ctx_lin) = conclusion.as_subst()?;

        if !subst.0.is_empty() {
            return None;
        }
        if ctx_lin.judgements.len() != 0 {
            return None;
        }

        Some(SubstEmpty { context: ctx })
    }
}
