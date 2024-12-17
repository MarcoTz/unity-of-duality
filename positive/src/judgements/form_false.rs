use super::{Conclusion, Judgement, JudgementKind};
use crate::{context::Context, formula::Formula, linear_context::LinearContext};

pub struct FormFalse {
    context: Context,
    formula: Formula,
}

impl Judgement for FormFalse {
    fn premises(&self) -> Vec<Conclusion> {
        let contexts = self.formula.clone().support();
        contexts
            .into_iter()
            .map(|ctx| Conclusion::Contra(self.context.clone().append(ctx.into())))
            .collect()
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::False(self.context.clone(), self.formula.clone())
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::False
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        let (conc_ctx, cont_form) = conclusion.as_false()?;
        let form_supp = cont_form.clone().support();
        if premises.len() != form_supp.len() {
            return None;
        };

        for (premise, supp_ctx_lin) in premises.into_iter().zip(form_supp.into_iter()) {
            let premise_ctx = premise.as_contra()?;
            let supp_ctx = conc_ctx.clone().append(supp_ctx_lin.into());
            if premise_ctx != supp_ctx {
                return None;
            }
        }

        Some(FormFalse {
            context: conc_ctx,
            formula: cont_form,
        })
    }
}
