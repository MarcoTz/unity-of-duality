use crate::{
    context::Context,
    judgement::{Conclusion, Judgement, JudgementKind},
    statements::Statement,
    terms::Term,
    types::Type,
};

pub struct ContPattern {
    ty: Type,
    context: Context,
    statements: Vec<Statement>,
}

impl Judgement for ContPattern {
    fn premises(&self) -> Vec<Conclusion> {
        let mut premises = vec![];
        let patterns = self.ty.clone().patterns();
        for (i, (ctx, _)) in patterns.into_iter().enumerate() {
            premises.push(Conclusion::Stmt(
                self.context.clone().combine(ctx.into()),
                self.statements[i].clone(),
            ));
        }

        premises
    }

    fn conclusion(&self) -> Conclusion {
        let patterns: Vec<(Term, Statement)> = self
            .ty
            .clone()
            .patterns()
            .into_iter()
            .map(|(_, t)| t)
            .zip(self.statements.clone().into_iter())
            .collect();
        Conclusion::Cont(
            self.context.clone(),
            Term::Pattern(patterns),
            self.ty.clone(),
        )
    }

    fn kind(&self) -> JudgementKind {
        JudgementKind::Cont
    }

    fn new(premises: Vec<Conclusion>, conclusion: Conclusion) -> Option<Self> {
        let (ctx, t, ty) = conclusion.as_cont()?;
        let pts = t.as_pat()?;
        let patterns = ty.clone().patterns();

        if premises.len() != patterns.len() {
            return None;
        }

        if pts.len() != premises.len() {
            return None;
        }

        let mut statements = vec![];

        for ((premise, (ctx_lin, t1)), (t2, stmt)) in premises
            .into_iter()
            .zip(patterns.into_iter())
            .zip(pts.into_iter())
        {
            let (prem_ctx, prem_stmt) = premise.as_stmt()?;
            if ctx.clone().combine(ctx_lin.into()) != prem_ctx {
                return None;
            }

            if t1 != t2 {
                return None;
            }

            if prem_stmt != stmt {
                return None;
            }
            statements.push(stmt);
        }

        Some(ContPattern {
            ty,
            context: ctx,
            statements,
        })
    }
}
