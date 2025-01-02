use crate::{
    context::{Context, ContextJudgement, LinearContext},
    coterms::Coterm,
    cotypes::Cotype,
    judgement::{Conclusion, Judgement, JudgementKind},
    substitution::{Substitution, SubstitutionBinding},
    TypeVar, Var,
};

pub struct VarCoval {
    new_binding_from: Var,
    new_binding_to: Var,
    ty: TypeVar,
    context: Context,
    subst: Substitution,
    linear_context: LinearContext,
}

impl Judgement for VarCoval {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::ContainsCoty(
                self.new_binding_to.clone(),
                Cotype::Var(self.ty.clone()),
                self.context.clone(),
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
            self.subst.clone().add(SubstitutionBinding::VarCoterm {
                from: self.new_binding_from.clone(),
                to: Coterm::Var(self.new_binding_to.clone()),
            }),
            self.linear_context.clone().add(ContextJudgement::Covalue(
                self.new_binding_from.clone(),
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
        let (left_v, left_ty, left_ctx) = premise_left.as_contains_coty()?;
        let left_tyvar = left_ty.as_var()?;

        let premise_right = premises.get(1).unwrap().clone();
        let (ctx_right, subst_right, ctx_right_lin) = premise_right.as_subst()?;

        let (ctx_conc, mut subst_conc, mut ctx_lin_conc) = conclusion.as_subst()?;
        if subst_conc.0.is_empty() {
            return None;
        }
        let subst_fst = subst_conc.0.remove(0);
        let (from, to) = subst_fst.as_var_coterm()?;
        let to_var = to.as_var()?;

        if ctx_lin_conc.judgements.is_empty() {
            return None;
        }
        let judgement = ctx_lin_conc.judgements.remove(0);
        let (judg_var, judg_ty) = judgement.as_coval()?;

        if left_ctx != ctx_right {
            return None;
        }
        if ctx_conc != ctx_right {
            return None;
        }
        if subst_conc != subst_right {
            return None;
        }
        if left_v != to_var {
            return None;
        }
        if ctx_lin_conc != ctx_right_lin {
            return None;
        }

        if judg_var != from {
            return None;
        }
        if left_tyvar != judg_ty {
            return None;
        }
        Some(VarCoval {
            new_binding_to: to_var,
            new_binding_from: from,
            ty: left_tyvar,
            context: left_ctx,
            subst: subst_right,
            linear_context: ctx_right_lin,
        })
    }
}
