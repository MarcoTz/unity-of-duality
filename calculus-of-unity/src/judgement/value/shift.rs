use crate::{
    context::ContextJudgement,
    cotypes::Cotype,
    judgement::{Conclusion, Judgement, JudgementKind},
    terms::Term,
    types::Type,
    Covar,
};

pub struct ValShift {
    covar: Covar,
    ty: Cotype,
}

impl Judgement for ValShift {
    fn premises(&self) -> Vec<Conclusion> {
        vec![]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Val(
            ContextJudgement::Expression(self.covar.clone(), self.ty.clone()).into(),
            Term::Covar(self.covar.clone()),
            Type::shift(self.ty.clone()),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::LinearVal
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if !premises.is_empty() {
            return None;
        }
        let (ctx, t, ty) = conclusion.as_val()?;
        let ctx_lin = ctx.as_linear()?;
        let judg = ctx_lin.as_judgement()?;
        let (ctx_cv, ctx_ty) = judg.as_exp()?;

        let cv = t.as_covar()?;
        let coty = ty.as_shift()?;

        if ctx_cv != cv {
            return None;
        }
        if coty != ctx_ty {
            return None;
        }

        Some(ValShift {
            covar: ctx_cv,
            ty: coty,
        })
    }
}
