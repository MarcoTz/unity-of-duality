use crate::{
    context::Context,
    judgements::{Conclusion, Judgement, JudgementKind},
    positive::formula::Formula,
};

pub struct OneTriv;

impl Judgement for OneTriv {
    fn premises(&self) -> Vec<Conclusion> {
        vec![]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Triv(Context::default(), Formula::One)
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::LinearTriv
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if !premises.is_empty() {
            return None;
        }

        let (conc_ctx, conc_form) = conclusion.as_triv()?;
        if conc_ctx != Context::default() {
            return None;
        };

        if let Formula::One = conc_form {
            Some(OneTriv)
        } else {
            None
        }
    }
}
