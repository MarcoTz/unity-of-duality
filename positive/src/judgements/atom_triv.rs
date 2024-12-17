use super::{Conclusion, Judgement, JudgementKind};
use crate::{formula::Atom, linear_context::ContextJudgement};

pub struct TrivAtom {
    atom: Atom,
}

impl Judgement for TrivAtom {
    fn premises(&self) -> Vec<Conclusion> {
        vec![]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Triv(
            ContextJudgement::Triv(self.atom.clone()).into(),
            self.atom.clone().into(),
        )
        .into()
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::LinearTriv
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if !premises.is_empty() {
            return None;
        }
        let (ctx, conc) = conclusion.as_triv()?;
        let ctx_lin = ctx.as_linear()?;
        if ctx_lin.judgements.len() != 1 {
            return None;
        };
        let ctx_fst = ctx_lin.judgements.first().unwrap();
        let at_ctx = ctx_fst.clone().as_triv()?;

        let at_conc = conc.as_atm()?;

        if *at_ctx == at_conc {
            Some(TrivAtom { atom: at_conc })
        } else {
            None
        }
    }
}
