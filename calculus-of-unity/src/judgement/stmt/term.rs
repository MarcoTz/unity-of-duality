use crate::{
    context::Context,
    judgement::{Conclusion, Judgement, JudgementKind},
    statements::Statement,
    terms::Term,
    types::Type,
};

pub struct StmtTerm {
    context: Context,
    cont_term: Term,
    ty: Type,
    term: Term,
}

impl Judgement for StmtTerm {
    fn premises(&self) -> Vec<Conclusion> {
        vec![
            Conclusion::Cont(
                self.context.clone(),
                self.cont_term.clone(),
                self.ty.clone(),
            ),
            Conclusion::Val(self.context.clone(), self.term.clone(), self.ty.clone()),
        ]
    }

    fn conclusion(&self) -> Conclusion {
        Conclusion::Stmt(
            self.context.clone(),
            Statement::TermTerm(self.cont_term.clone(), self.term.clone()),
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
        let (ctx_left, cont_term_left, ty_left) = premise_left.as_cont()?;

        let premise_right = premises.get(1).unwrap().clone();
        let (ctx_right, term_right, ty_right) = premise_right.as_val()?;

        let (ctx_conc, stmt_conc) = conclusion.as_stmt()?;
        let (cont_term_conc, term_conc) = stmt_conc.as_termterm()?;

        if ctx_left != ctx_right {
            return None;
        }
        if ctx_right != ctx_conc {
            return None;
        }
        if cont_term_left != cont_term_conc {
            return None;
        }
        if ty_left != ty_right {
            return None;
        }
        if cont_term_left != cont_term_conc {
            return None;
        }
        if term_right != term_conc {
            return None;
        }

        Some(StmtTerm {
            context: ctx_conc,
            cont_term: cont_term_conc,
            ty: ty_left,
            term: term_right,
        })
    }
}
