use crate::{
    context::Context,
    coterms::Coterm,
    cotypes::Cotype,
    judgement::{Conclusion, Judgement, JudgementKind},
    statements::Statement,
};

pub struct StmtCoterm {
    context: Context,
    left_term: Coterm,
    right_term: Coterm,
    ty: Cotype,
}

impl Judgement for StmtCoterm {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::Exp(
                self.context.clone(),
                self.left_term.clone(),
                self.ty.clone(),
            ),
            Conclusion::Coval(
                self.context.clone(),
                self.right_term.clone(),
                self.ty.clone(),
            ),
        ]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Stmt(
            self.context.clone(),
            Statement::CotermCoterm(self.left_term.clone(), self.right_term.clone()),
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
        let (ctx_left, t_left, ty_left) = premise_left.as_exp()?;

        let premise_right = premises.get(1).unwrap().clone();
        let (ctx_right, t_right, ty_right) = premise_right.as_coval()?;

        let (ctx_conc, stmt_conc) = conclusion.as_stmt()?;
        let (left_conc, right_conc) = stmt_conc.as_cotermcoterm()?;

        if ctx_left != ctx_right {
            return None;
        }
        if ctx_right != ctx_conc {
            return None;
        }
        if t_left != left_conc {
            return None;
        }
        if ty_left != ty_right {
            return None;
        }
        if t_right != right_conc {
            return None;
        }

        Some(StmtCoterm {
            context: ctx_conc,
            left_term: left_conc,
            right_term: right_conc,
            ty: ty_left,
        })
    }
}
