use crate::{
    context::{Context, ContextJudgement, LinearContext},
    judgement::{Conclusion, Judgement, JudgementKind},
    substitution::{Substitution, SubstitutionBinding},
    terms::Term,
    types::Type,
    Covar,
};

pub struct SubstCovar {
    covar: Covar,
    context: Context,
    cont_term: Term,
    cont_ty: Type,
    linear_context: LinearContext,
    subst: Substitution,
}

impl Judgement for SubstCovar {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::Cont(
                self.context.clone(),
                self.cont_term.clone(),
                self.cont_ty.clone(),
            ),
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
            self.subst.clone().add(SubstitutionBinding::CovarBinding {
                from: self.covar.clone(),
                to: self.cont_term.clone(),
            }),
            self.linear_context
                .clone()
                .add(ContextJudgement::Continuation(
                    self.covar.clone(),
                    self.cont_ty.clone(),
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
        let (ctx_left, t_left, ty_left) = premise_left.as_cont()?;

        let premise_right = premises.get(1).unwrap().clone();
        let (ctx_right, subst_right, ctx_lin_right) = premise_right.as_subst()?;

        let (ctx_conc, mut subst_conc, mut ctx_lin_conc) = conclusion.as_subst()?;

        if subst_conc.0.is_empty() {
            return None;
        }
        let subst_fst = subst_conc.0.remove(0);
        let (subst_covar, subst_term) = subst_fst.as_covar()?;

        if ctx_lin_conc.judgements.is_empty() {
            return None;
        }
        let judg_fst = ctx_lin_conc.judgements.remove(0);
        let (judg_covar, judg_ty) = judg_fst.as_cont()?;

        if ctx_left != ctx_right {
            return None;
        }

        if ctx_conc != ctx_left {
            return None;
        }

        if subst_conc != subst_right {
            return None;
        }

        if ctx_lin_right != ctx_lin_conc {
            return None;
        }

        if judg_ty != ty_left {
            return None;
        }

        if subst_term != t_left {
            return None;
        }

        if subst_covar != judg_covar {
            return None;
        }

        Some(SubstCovar {
            covar: judg_covar,
            context: ctx_conc,
            cont_term: subst_term,
            cont_ty: ty_left,
            linear_context: ctx_lin_conc,
            subst: subst_conc,
        })
    }
}
