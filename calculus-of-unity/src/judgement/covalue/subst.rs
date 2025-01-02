use crate::{
    context::{Context, LinearContext},
    coterms::Coterm,
    cotypes::Cotype,
    judgement::{Conclusion, Judgement, JudgementKind},
    substitution::Substitution,
};

pub struct CovalSubst {
    linear_context: LinearContext,
    term: Coterm,
    ty: Cotype,
    context: Context,
    subst: Substitution,
}

impl Judgement for CovalSubst {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::Coval(
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
        Conclusion::Coval(
            self.context.clone(),
            self.subst.clone().apply_coterm(self.term.clone()),
            self.ty.clone(),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::NonLinearCoval
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if premises.len() != 2 {
            return None;
        }

        let premise_left = premises.first().unwrap().clone();
        let (ctx_left, t_left, ty_left) = premise_left.as_coval()?;
        let ctx_left_lin = ctx_left.as_linear()?;

        let premise_right = premises.get(1).unwrap().clone();
        let (ctx_right, subst_right, ctx_right_lin) = premise_right.as_subst()?;

        let (ctx_conc, t_conc, ty_conc) = conclusion.as_coval()?;

        if ctx_left_lin != ctx_right_lin {
            return None;
        }
        if t_conc != subst_right.clone().apply_coterm(t_left.clone()) {
            return None;
        }
        if ty_conc != ty_left {
            return None;
        }
        if ctx_conc != ctx_right {
            return None;
        }

        Some(CovalSubst {
            linear_context: ctx_left_lin,
            term: t_left,
            ty: ty_left,
            context: ctx_right,
            subst: subst_right,
        })
    }
}
