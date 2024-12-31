use crate::{
    context::ContextJudgement,
    judgement::{Conclusion, Judgement, JudgementKind},
    terms::Term,
    types::TypeVar,
    Var,
};

pub struct VarVal {
    var: Var,
    ty: TypeVar,
}

impl Judgement for VarVal {
    fn premises(&self) -> Vec<Conclusion> {
        vec![]
    }
    fn conclusion(&self) -> Conclusion {
        Conclusion::Val(
            ContextJudgement::Value(self.var.clone(), self.ty.clone()).into(),
            Term::var(&self.var),
            self.ty.clone().into(),
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
        let (judg_var, judg_ty) = judgement.as_val()?;

        let term_var = t.as_var()?;
        let ty_var = ty.as_var()?;

        if term_var != judg_var {
            return None;
        }
        if judg_ty != ty_var {
            return None;
        }

        Some(VarVal {
            var: term_var,
            ty: ty_var,
        })
    }
}
