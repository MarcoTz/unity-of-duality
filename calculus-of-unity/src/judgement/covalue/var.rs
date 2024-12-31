use crate::{
    context::ContextJudgement,
    coterms::Coterm,
    judgement::{Conclusion, Judgement, JudgementKind},
    types::TypeVar,
    Var,
};

pub struct CovalVar {
    var: Var,
    ty_var: TypeVar,
}

impl Judgement for CovalVar {
    fn premises(&self) -> Vec<Conclusion> {
        vec![]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Coval(
            ContextJudgement::Covalue(self.var.clone(), self.ty_var.clone()).into(),
            Coterm::Var(self.var.clone()),
            self.ty_var.clone().into(),
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
        let ctx_lin = ctx.as_linear()?;
        let judg = ctx_lin.as_judgement()?;
        let (ctx_var, ctx_ty) = judg.as_coval()?;
        let ty_var = ty.as_var()?;
        let v = cot.as_var()?;

        if ty_var != ctx_ty {
            return None;
        }
        if ctx_var != v {
            return None;
        }
        Some(CovalVar { var: v, ty_var })
    }
}
