use crate::{
    context::Context,
    judgement::{Conclusion, Judgement, JudgementKind},
    statements::Statement,
};

pub struct StmtDone {
    context: Context,
}

impl Judgement for StmtDone {
    fn premises(&self) -> Vec<Conclusion> {
        vec![]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Stmt(self.context.clone(), Statement::Done)
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::Statement
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if !premises.is_empty() {
            return None;
        }

        let (ctx, stmt) = conclusion.as_stmt()?;
        if !matches!(stmt, Statement::Done) {
            return None;
        }
        Some(StmtDone { context: ctx })
    }
}
