use super::{Conclusion, Judgement, JudgementKind};
use crate::{context::Context, negative::formula::Formula};

pub struct FormTrue {
    formula: Formula,
    context: Context,
}

impl Judgement for FormTrue {
    fn premises(&self) -> Vec<Conclusion> {
        let supp = self.formula.clone().support();
        supp.into_iter()
            .map(|ctx| Conclusion::Contra(self.context.clone().append(ctx.into())))
            .collect()
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::True(self.context.clone(), self.formula.clone())
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::True
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        let (ctx, form) = conclusion.as_true()?;
        let form_supp = form.clone().support();

        if premises.len() != form_supp.len() {
            return None;
        }

        for (conc, ctx_lin) in premises.into_iter().zip(form_supp.into_iter()) {
            let ctx_combined = ctx.clone().append(ctx_lin.into());
            let conc_expected = Conclusion::Contra(ctx_combined);
            if conc != conc_expected {
                return None;
            }
        }

        Some(FormTrue {
            formula: form,
            context: ctx,
        })
    }
}
