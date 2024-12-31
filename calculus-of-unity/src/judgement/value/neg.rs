use crate::{
    context::ContextJudgement,
    judgement::{Conclusion, Judgement, JudgementKind},
    terms::Term,
    types::Type,
    Covar,
};

pub struct CovarVal {
    covar: Covar,
    ty: Type,
}

impl Judgement for CovarVal {
    fn premises(&self) -> Vec<Conclusion> {
        vec![]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Val(
            ContextJudgement::Continuation(self.covar.clone(), self.ty.clone()).into(),
            Term::covar(&self.covar),
            self.ty.clone(),
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
        let judgement = ctx_lin.as_judgement()?;
        let (covar, cont_ty) = judgement.as_cont()?;

        let covar_term = t.as_covar()?;
        let ty_neg = ty.as_neg()?;

        if covar != covar_term {
            return None;
        }

        if ty_neg != cont_ty {
            return None;
        }

        Some(CovarVal {
            covar: covar_term,
            ty: cont_ty,
        })
    }
}
