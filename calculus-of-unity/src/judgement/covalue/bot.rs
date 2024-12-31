use crate::{
    context::Context,
    coterms::Coterm,
    cotypes::Cotype,
    judgement::{Conclusion, Judgement, JudgementKind},
};

pub struct CovalBot;

impl Judgement for CovalBot {
    fn premises(&self) -> Vec<Conclusion> {
        vec![]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Coval(Context::default(), Coterm::Counit, Cotype::Bot)
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::LinearCoval
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if !premises.is_empty() {
            return None;
        }

        let (ctx, cot, ty) = conclusion.as_coval()?;
        if !ctx.contexts.is_empty() {
            return None;
        }
        if !matches!(cot, Coterm::Counit) {
            return None;
        }
        if !matches!(ty, Cotype::Bot) {
            return None;
        }
        Some(CovalBot)
    }
}
