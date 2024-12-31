use crate::{
    context::LinearContext,
    coterms::Coterm,
    cotypes::Cotype,
    judgement::{Conclusion, Judgement, JudgementKind},
};

pub struct CovalFst {
    linear_context: LinearContext,
    term: Coterm,
    ty_left: Cotype,
    ty_right: Cotype,
}

impl Judgement for CovalFst {
    fn premises(&self) -> Vec<Conclusion> {
        vec![Conclusion::Coval(
            self.linear_context.clone().into(),
            self.term.clone(),
            self.ty_left.clone(),
        )]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Coval(
            self.linear_context.clone().into(),
            Coterm::fst(self.term.clone()),
            Cotype::and(self.ty_left.clone(), self.ty_right.clone()),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::LinearCoval
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if premises.len() != 1 {
            return None;
        }

        let premise = premises.first().unwrap().clone();
        let (prem_ctx, prem_cot, prem_ty) = premise.as_coval()?;
        let prem_ctx_lin = prem_ctx.as_linear()?;

        let (conc_ctx, conc_cot, conc_ty) = conclusion.as_coval()?;
        let conc_ctx_lin = conc_ctx.as_linear()?;
        let conc_cot_inner = conc_cot.as_fst()?;
        let (left, right) = conc_ty.as_and()?;

        if prem_ctx_lin != conc_ctx_lin {
            return None;
        }

        if prem_cot != conc_cot_inner {
            return None;
        }

        if left != prem_ty {
            return None;
        }

        Some(CovalFst {
            linear_context: conc_ctx_lin,
            term: prem_cot,
            ty_left: left,
            ty_right: right,
        })
    }
}
