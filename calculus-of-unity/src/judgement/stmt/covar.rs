use crate::{
    context::Context,
    judgement::{Conclusion, Judgement, JudgementKind},
    statements::Statement,
    terms::Term,
    types::Type,
    Covar,
};

pub struct CovarStmt {
    covar: Covar,
    ty: Type,
    context: Context,
    term: Term,
}

impl Judgement for CovarStmt {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::Contains(self.covar.clone(), self.ty.clone(), self.context.clone()),
            Conclusion::Val(self.context.clone(), self.term.clone(), self.ty.clone()),
        ]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Stmt(
            self.context.clone(),
            Statement::CovarTerm(self.covar.clone(), self.term.clone()),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::Statement
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        if premises.len() != 2 {
            return None;
        }

        let premise_left = premises.first().unwrap().clone();
        let (covar_left, ty_left, ctx_left) = premise_left.as_contains()?;

        let premise_right = premises.get(1).unwrap().clone();
        let (ctx_right, t_right, ty_right) = premise_right.as_val()?;

        let (ctx_conc, stmt) = conclusion.as_stmt()?;
        let (covar_conc, t_conc) = stmt.as_covarterm()?;
        if covar_left != covar_conc {
            return None;
        }
        if ty_left != ty_right {
            return None;
        }
        if ctx_left != ctx_right {
            return None;
        }
        if ctx_right != ctx_conc {
            return None;
        }
        if t_right != t_conc {
            return None;
        }

        Some(CovarStmt {
            covar: covar_conc,
            ty: ty_left,
            context: ctx_conc,
            term: t_conc,
        })
    }
}
