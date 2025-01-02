use crate::{
    context::ContextJudgement,
    coterms::Coterm,
    cotypes::Cotype,
    judgement::{Conclusion, Judgement, JudgementKind},
    types::Type,
    Covar,
};

pub struct CovalShift {
    covar: Covar,
    ty: Type,
}

impl Judgement for CovalShift {
    fn premises(&self) -> Vec<Conclusion> {
        vec![]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Coval(
            ContextJudgement::Continuation(self.covar.clone(), self.ty.clone()).into(),
            Coterm::Covar(self.covar.clone()),
            Cotype::shift(self.ty.clone()),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::LinearCoval
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if !premises.is_empty() {
            return None;
        }
        let (ctx, t, ty) = conclusion.as_coval()?;
        let ctx_lin = ctx.as_linear()?;
        let judg = ctx_lin.as_judgement()?;
        let (ctx_cv, ctx_ty) = judg.as_cont()?;

        let cv = t.as_covar()?;
        let coty = ty.as_shift()?;

        if ctx_cv != cv {
            return None;
        }
        if coty != ctx_ty {
            return None;
        }

        Some(CovalShift {
            covar: ctx_cv,
            ty: coty,
        })
    }
}
