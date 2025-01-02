use crate::{
    context::LinearContext,
    coterms::Coterm,
    cotypes::Cotype,
    judgement::{Conclusion, Judgement, JudgementKind},
};

pub struct CovalPar {
    context_left: LinearContext,
    term_left: Coterm,
    ty_left: Cotype,
    context_right: LinearContext,
    term_right: Coterm,
    ty_right: Cotype,
}

impl Judgement for CovalPar {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::Coval(
                self.context_left.clone().into(),
                self.term_left.clone(),
                self.ty_left.clone(),
            ),
            Conclusion::Coval(
                self.context_right.clone().into(),
                self.term_right.clone(),
                self.ty_right.clone(),
            ),
        ]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Coval(
            self.context_left
                .clone()
                .combine(self.context_right.clone())
                .into(),
            Coterm::lpair(self.term_left.clone(), self.term_right.clone()),
            Cotype::par(self.ty_left.clone(), self.ty_right.clone()),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::LinearCoval
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if premises.len() != 2 {
            return None;
        }
        let premise_left = premises.first().unwrap().clone();
        let (ctx_left, cot_left, ty_left) = premise_left.as_coval()?;
        let ctx_left_lin = ctx_left.as_linear()?;

        let premise_right = premises.get(1).unwrap().clone();
        let (ctx_right, cot_right, ty_right) = premise_right.as_coval()?;
        let ctx_right_lin = ctx_right.as_linear()?;

        let (ctx_conc, cot_conc, ty_conc) = conclusion.as_coval()?;
        let ctx_conc_lin = ctx_conc.as_linear()?;

        if ctx_left_lin.clone().combine(ctx_right_lin.clone()) != ctx_conc_lin {
            return None;
        }
        if Coterm::lpair(cot_left.clone(), cot_right.clone()) != cot_conc {
            return None;
        }
        if Cotype::par(ty_left.clone(), ty_right.clone()) != ty_conc {
            return None;
        }

        Some(CovalPar {
            context_left: ctx_left_lin,
            term_left: cot_left,
            ty_left,
            context_right: ctx_right_lin,
            term_right: cot_right,
            ty_right,
        })
    }
}
