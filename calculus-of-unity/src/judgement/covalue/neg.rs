use crate::{
    context::ContextJudgement,
    coterms::Coterm,
    judgement::{Conclusion, Judgement, JudgementKind},
    types::Type,
    Covar,
};

pub struct CovalNeg {
    pub ty: Type,
    pub covar: Covar,
}

impl Judgement for CovalNeg {
    fn premises(&self) -> Vec<Conclusion> {
        vec![]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Coval(
            ContextJudgement::Expression(self.covar.clone(), self.ty.clone()).into(),
            Coterm::Covar(self.covar.clone()),
            Type::ng(self.ty.clone()),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::LinearCoval
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if !premises.is_empty() {
            return None;
        }

        let (ctx, cot, ty) = conclusion.as_coval()?;
        let ty_right = ty.as_neg()?;
        let cv_left = cot.as_covar()?;
        let ctx_lin = ctx.as_linear()?;
        let judgement = ctx_lin.as_judgement()?;
        let (cv_right, ty_left) = judgement.as_exp()?;
        if cv_left != cv_right {
            return None;
        }
        if ty_left != ty_right {
            return None;
        }

        Some(CovalNeg {
            ty: ty_left,
            covar: cv_left,
        })
    }
}
