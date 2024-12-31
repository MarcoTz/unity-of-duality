use crate::{
    context::Context,
    judgement::{Conclusion, Judgement, JudgementKind},
    terms::Term,
    types::Type,
    Covar,
};

pub struct CovarCont {
    covar: Covar,
    context: Context,
    ty: Type,
}

impl Judgement for CovarCont {
    fn premises(&self) -> Vec<Conclusion> {
        vec![Conclusion::Contains(
            self.covar.clone(),
            self.ty.clone(),
            self.context.clone(),
        )]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Cont(
            self.context.clone(),
            Term::covar(&self.covar),
            self.ty.clone(),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::Cont
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if premises.len() != 1 {
            return None;
        }

        let premise = premises.first().unwrap().clone();
        let (covar, ty, ctx) = premise.as_contains()?;
        let (conc_ctx, conc_t, conc_ty) = conclusion.as_cont()?;
        let conc_cv = conc_t.as_covar()?;

        if ctx != conc_ctx {
            return None;
        }

        if conc_cv != covar {
            return None;
        }

        if ty != conc_ty {
            return None;
        }
        Some(CovarCont {
            context: conc_ctx,
            covar,
            ty,
        })
    }
}
