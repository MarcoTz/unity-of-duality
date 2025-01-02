use crate::{
    context::Context,
    coterms::Coterm,
    cotypes::Cotype,
    judgement::{Conclusion, Judgement, JudgementKind},
    Covar,
};

pub struct ExpCovar {
    covar: Covar,
    ty: Cotype,
    context: Context,
}

impl Judgement for ExpCovar {
    fn premises(&self) -> Vec<Conclusion> {
        vec![Conclusion::ContainsCoty(
            self.covar.clone(),
            self.ty.clone(),
            self.context.clone(),
        )]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Exp(
            self.context.clone(),
            Coterm::Covar(self.covar.clone()),
            self.ty.clone(),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::Exp
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if premises.len() != 1 {
            return None;
        }

        let premise = premises.first().unwrap().clone();
        let (prem_cv, prem_ty, prem_ctx) = premise.as_contains_coty()?;

        let (conc_ctx, conc_t, conc_ty) = conclusion.as_exp()?;
        let conc_cv = conc_t.as_covar()?;

        if prem_cv != conc_cv {
            return None;
        }
        if prem_ty != conc_ty {
            return None;
        }
        if prem_ctx != conc_ctx {
            return None;
        }

        Some(ExpCovar {
            covar: prem_cv,
            ty: prem_ty,
            context: prem_ctx,
        })
    }
}
