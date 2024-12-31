use crate::{
    context::Context,
    judgement::{Conclusion, Judgement, JudgementKind},
    terms::Term,
    types::Type,
};

pub struct UnitVal;

impl Judgement for UnitVal {
    fn premises(&self) -> Vec<Conclusion> {
        vec![]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Val(Context::default(), Term::Unit, Type::One)
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::LinearVal
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if !premises.is_empty() {
            return None;
        }
        let (ctx, t, ty) = conclusion.as_val()?;

        if !ctx.contexts.is_empty() {
            return None;
        }

        if !matches!(t, Term::Unit) {
            return None;
        }

        if !matches!(ty, Type::One) {
            return None;
        }

        Some(UnitVal)
    }
}
