use crate::{
    context::Context,
    coterms::Coterm,
    cotypes::Cotype,
    judgement::{Conclusion, Judgement, JudgementKind},
    statements::Statement,
    Covar,
};

pub struct StmtCovarCoval {
    covar: Covar,
    ty: Cotype,
    context: Context,
    term: Coterm,
}

impl Judgement for StmtCovarCoval {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::ContainsCoty(self.covar.clone(), self.ty.clone(), self.context.clone()),
            Conclusion::Coval(self.context.clone(), self.term.clone(), self.ty.clone()),
        ]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Stmt(
            self.context.clone(),
            Statement::CovarCoterm(self.covar.clone(), self.term.clone()),
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
        let (cv_left, ty_left, ctx_left) = premise_left.as_contains_coty()?;

        let premise_right = premises.get(1).unwrap().clone();
        let (ctx_right, t_right, ty_right) = premise_right.as_coval()?;

        let (ctx_conc, stmt_conc) = conclusion.as_stmt()?;
        let (cv_conc, t_conc) = stmt_conc.as_covarcoterm()?;

        if cv_left != cv_conc {
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

        Some(StmtCovarCoval {
            covar: cv_left,
            ty: ty_left,
            context: ctx_left,
            term: t_right,
        })
    }
}
