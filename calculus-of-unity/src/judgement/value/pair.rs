use crate::{
    context::LinearContext,
    judgement::{Conclusion, Judgement, JudgementKind},
    terms::Term,
    types::Type,
};

pub struct PairVal {
    pub context_left: LinearContext,
    pub term_left: Term,
    pub ty_left: Type,
    pub context_right: LinearContext,
    pub term_right: Term,
    pub ty_right: Type,
}

impl Judgement for PairVal {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::Val(
                self.context_left.clone().into(),
                self.term_left.clone(),
                self.ty_left.clone(),
            ),
            Conclusion::Val(
                self.context_right.clone().into(),
                self.term_right.clone(),
                self.ty_right.clone(),
            ),
        ]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Val(
            self.context_left
                .clone()
                .combine(self.context_right.clone())
                .into(),
            Term::pair(self.term_left.clone(), self.term_right.clone()),
            Type::times(self.ty_left.clone(), self.ty_right.clone()),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::LinearVal
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if premises.len() != 2 {
            return None;
        }
        let premise_left = premises.first().unwrap();
        let (ctx_left, t_left, ty_left) = premise_left.clone().as_val()?;
        let ctx_left_lin = ctx_left.as_linear()?;

        let premise_right = premises.get(1).unwrap();
        let (ctx_right, t_right, ty_right) = premise_right.clone().as_val()?;
        let ctx_right_lin = ctx_right.as_linear()?;
        let ctx_combined = ctx_left_lin.clone().combine(ctx_right_lin.clone());

        let (conc_ctx, cont_t, conc_ty) = conclusion.as_val()?;
        let conc_ctx_lin = conc_ctx.as_linear()?;
        let (conc_left, conc_right) = cont_t.as_pair()?;
        let (conc_ty_left, conc_ty_right) = conc_ty.as_times()?;

        if ctx_combined != conc_ctx_lin {
            return None;
        }

        if conc_left != t_left {
            return None;
        }

        if conc_right != t_right {
            return None;
        }

        if conc_ty_left != ty_left {
            return None;
        }

        if conc_ty_right != ty_right {
            return None;
        }

        Some(PairVal {
            context_left: ctx_left_lin,
            term_left: t_left,
            ty_left,
            context_right: ctx_right_lin,
            term_right: t_right,
            ty_right,
        })
    }
}
