use super::formula::Formula;
use positive::{
    formula::Formula as FormulaPos, judgements::Conclusion, linear_context::ContextJudgement,
};

impl From<FormulaPos> for Formula {
    fn from(pos: FormulaPos) -> Formula {
        match pos {
            FormulaPos::Atom(at) => Formula::Atom(at),
            FormulaPos::One => Formula::True,
            FormulaPos::NegV(f) => Formula::neg((*f).into()),
            FormulaPos::Tensor(l, r) => Formula::and((*l).into(), (*r).into()),
            FormulaPos::Zero => Formula::False,
            FormulaPos::Plus(l, r) => Formula::or((*l).into(), (*r).into()),
        }
    }
}

impl From<Conclusion> for Formula {
    fn from(conc: Conclusion) -> Formula {
        match conc {
            Conclusion::Ctx(_, ctx_lin) => {
                let ctx_terms: Vec<Formula> = ctx_lin
                    .judgements
                    .into_iter()
                    .map(|judgement| judgement.into())
                    .collect();
                ctx_terms
                    .into_iter()
                    .fold(Formula::True, |form, next| Formula::and(form, next))
            }
            Conclusion::Triv(_, f) => f.into(),
            Conclusion::False(_, f) => Formula::neg(f.into()),
            Conclusion::Contra(_) => Formula::False,
            Conclusion::Contains(_, judgement) => judgement.into(),
        }
    }
}

impl From<ContextJudgement> for Formula {
    fn from(judgement: ContextJudgement) -> Formula {
        match judgement {
            ContextJudgement::Triv(at) => at.into(),
            ContextJudgement::False(f) => Formula::neg(f.into()),
        }
    }
}
