use crate::{
    context::{Context, ContextJudgement, LinearContext},
    coterms::Coterm,
    cotypes::Cotype,
    judgement::{Conclusion, Judgement, JudgementKind},
    substitution::{Substitution, SubstitutionBinding},
    Covar,
};

pub struct SubstCovarExp {
    covar: Covar,
    context: Context,
    term: Coterm,
    ty: Cotype,
    subst: Substitution,
    linear_context: LinearContext,
}

impl Judgement for SubstCovarExp {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::Exp(self.context.clone(), self.term.clone(), self.ty.clone()),
            Conclusion::Subst(
                self.context.clone(),
                self.subst.clone(),
                self.linear_context.clone(),
            ),
        ]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Subst(
            self.context.clone(),
            self.subst
                .clone()
                .add(SubstitutionBinding::CovarCoterm {
                    from: self.covar.clone(),
                    to: self.term.clone(),
                })
                .into(),
            self.linear_context
                .clone()
                .add(ContextJudgement::Expression(
                    self.covar.clone(),
                    self.ty.clone(),
                )),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::Subst
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if premises.len() != 2 {
            return None;
        }
        let premise_left = premises.first().unwrap().clone();
        let (ctx_left, t_left, ty_left) = premise_left.as_exp()?;

        let premise_right = premises.get(1).unwrap().clone();
        let (ctx_right, subst_right, ctx_lin_right) = premise_right.as_subst()?;

        let (ctx_conc, mut subst_conc, mut ctx_lin_conc) = conclusion.as_subst()?;
        if subst_conc.0.is_empty() {
            return None;
        }
        let subst_fst = subst_conc.0.remove(0);
        let (conc_cv, conc_t) = subst_fst.as_covar_coterm()?;
        if ctx_lin_conc.judgements.is_empty() {
            return None;
        }
        let judg = ctx_lin_conc.judgements.remove(0);
        let (judg_cv, judg_ty) = judg.as_exp()?;

        if ctx_left != ctx_right {
            return None;
        }
        if ctx_conc != ctx_left {
            return None;
        }
        if subst_conc != subst_right {
            return None;
        }
        if t_left != conc_t {
            return None;
        }
        if conc_cv != judg_cv {
            return None;
        }
        if ty_left != judg_ty {
            return None;
        }
        if ctx_lin_conc != ctx_lin_right {
            return None;
        }

        Some(SubstCovarExp {
            covar: conc_cv,
            context: ctx_left,
            term: t_left,
            ty: ty_left,
            subst: subst_right,
            linear_context: ctx_lin_right,
        })
    }
}
