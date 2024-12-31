use crate::{
    context::Context,
    judgements::{Conclusion, Judgement, JudgementKind},
    negative::Formula,
};

pub struct FalsumAbsurd;

impl Judgement for FalsumAbsurd {
    fn premises(&self) -> Vec<Conclusion> {
        vec![]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Absurd(Context::default(), Formula::Falsum)
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::LinearAbsurd
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if !premises.is_empty() {
            return None;
        }

        let (ctx, form) = conclusion.as_absurd()?;

        if !ctx.contexts.is_empty() {
            return None;
        }

        if let Formula::Falsum = form {
            Some(FalsumAbsurd)
        } else {
            None
        }
    }
}
