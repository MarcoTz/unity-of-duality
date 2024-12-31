use crate::{
    context::LinearContext,
    judgement::{Conclusion, Judgement, JudgementKind},
    terms::Term,
    types::Type,
};

pub struct PlusRight {
    context: LinearContext,
    term: Term,
    ty_left: Type,
    ty_right: Type,
}

impl Judgement for PlusRight {
    fn premises(&self) -> Vec<Conclusion> {
        vec![Conclusion::Val(
            self.context.clone().into(),
            self.term.clone(),
            self.ty_right.clone(),
        )]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Val(
            self.context.clone().into(),
            Term::inr(self.term.clone()),
            Type::plus(self.ty_left.clone(), self.ty_right.clone()),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::LinearVal
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if premises.len() != 1 {
            return None;
        }

        let premise = premises.first().unwrap();
        let (ctx, t, ty) = premise.clone().as_val()?;
        let ctx_lin = ctx.as_linear()?;

        let (conc_ctx, conc_t, conc_ty) = conclusion.as_val()?;
        let conc_ctx_lin = conc_ctx.as_linear()?;
        let conc_inner = conc_t.as_inr()?;
        let (left, right) = conc_ty.as_plus()?;

        if ctx_lin != conc_ctx_lin {
            return None;
        }

        if t != conc_inner {
            return None;
        }

        if ty != right {
            return None;
        }
        Some(PlusRight {
            context: ctx_lin,
            term: t,
            ty_left: left,
            ty_right: right,
        })
    }
}
