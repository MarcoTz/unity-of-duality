use crate::{
    context::{Context, ContextJudgement, LinearContext},
    judgement::{Conclusion, Judgement, JudgementKind},
    substitution::{Substitution, SubstitutionBinding},
    terms::Term,
    types::TypeVar,
    Var,
};

pub struct SubstVar {
    context: Context,
    subst: Substitution,
    linear_context: LinearContext,
    new_binding_from: Var,
    new_binding_to: Var,
    ty_var: TypeVar,
}

impl Judgement for SubstVar {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::Contains(
                self.new_binding_to.clone(),
                self.ty_var.clone().into(),
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
            self.subst.clone().add(SubstitutionBinding {
                from: self.new_binding_from.clone(),
                to: Term::Var(self.new_binding_to.clone()),
            }),
            self.linear_context.clone().add(ContextJudgement::Value(
                self.new_binding_from.clone(),
                self.ty_var.clone(),
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
        let (var, ty, ctx_left) = premise_left.as_contains()?;
        let ty_var = ty.as_var()?;

        let premise_right = premises.get(1).unwrap().clone();
        let (ctx_right, subst, ctx_lin) = premise_right.as_subst()?;

        let (ctx_conc, mut subst_conc, mut ctx_lin_conc) = conclusion.as_subst()?;
        let judgement_first = ctx_lin_conc.judgements.remove(0);
        let (judg_var, judg_ty) = judgement_first.as_val()?;
        let subst_first = subst_conc.0.remove(0);

        if ctx_left != ctx_right {
            return None;
        }

        if ctx_conc != ctx_left {
            return None;
        }

        if ctx_lin_conc != ctx_lin {
            return None;
        }

        if subst_conc != subst {
            return None;
        }

        if Term::Var(var.clone()) != subst_first.to {
            return None;
        }

        if ty_var != judg_ty {
            return None;
        }

        if judg_var != subst_first.from {
            return None;
        }

        Some(SubstVar {
            context: ctx_conc,
            subst: subst,
            linear_context: ctx_lin,
            new_binding_from: judg_var,
            new_binding_to: var,
            ty_var,
        })
    }
}