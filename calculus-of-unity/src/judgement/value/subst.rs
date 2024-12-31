use crate::{
    context::{Context, LinearContext},
    judgement::{Conclusion, Judgement, JudgementKind},
    substitution::Substitution,
    terms::Term,
    types::Type,
};

pub struct ValSubst {
    pub linear_context: LinearContext,
    pub term: Term,
    pub ty: Type,
    pub context: Context,
    pub subst: Substitution,
}
impl Judgement for ValSubst {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::Val(
                self.linear_context.clone().into(),
                self.term.clone(),
                self.ty.clone(),
            ),
            Conclusion::Subst(
                self.context.clone(),
                self.subst.clone(),
                self.linear_context.clone(),
            ),
        ]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Val(
            self.context.clone(),
            self.subst.clone().apply(self.term.clone()),
            self.ty.clone(),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::NonLinearVal
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if premises.len() != 2 {
            return None;
        }

        let premise_left = premises.first().unwrap().clone();
        let (ctx_left, t_left, ty_left) = premise_left.as_val()?;
        let ctx_left_lin = ctx_left.as_linear()?;

        let premise_right = premises.get(1).unwrap().clone();
        let (ctx_right, subst, ctx_right_lin) = premise_right.as_subst()?;

        let (conc_ctx, conc_t, conc_ty) = conclusion.as_val()?;

        if ctx_left_lin != ctx_right_lin {
            return None;
        }

        if t_left != conc_t {
            return None;
        }

        if ty_left != conc_ty {
            return None;
        }

        if ctx_right != conc_ctx {
            return None;
        }

        if conc_t != subst.clone().apply(t_left) {
            return None;
        }

        Some(ValSubst {
            linear_context: ctx_left_lin,
            term: conc_t,
            ty: conc_ty,
            context: conc_ctx,
            subst,
        })
    }
}
