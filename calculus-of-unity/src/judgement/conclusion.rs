use crate::{
    context::{Context, LinearContext},
    coterms::Coterm,
    cotypes::Cotype,
    statements::Statement,
    substitution::Substitution,
    terms::Term,
    types::Type,
    Covar,
};

#[derive(Clone)]
pub enum Conclusion {
    Val(Context, Term, Type),
    Cont(Context, Term, Type),
    Coval(Context, Coterm, Cotype),
    Exp(Context, Coterm, Cotype),
    Subst(Context, Substitution, LinearContext),
    ContainsTy(Covar, Type, Context),
    ContainsCoty(Covar, Cotype, Context),
    Stmt(Context, Statement),
}

impl Conclusion {
    pub fn as_val(self) -> Option<(Context, Term, Type)> {
        if let Conclusion::Val(context, term, ty) = self {
            Some((context, term, ty))
        } else {
            None
        }
    }

    pub fn as_coval(self) -> Option<(Context, Coterm, Cotype)> {
        if let Conclusion::Coval(ctx, cot, ty) = self {
            Some((ctx, cot, ty))
        } else {
            None
        }
    }

    pub fn as_subst(self) -> Option<(Context, Substitution, LinearContext)> {
        if let Conclusion::Subst(ctx, subst, ctx_lin) = self {
            Some((ctx, subst, ctx_lin))
        } else {
            None
        }
    }

    pub fn as_cont(self) -> Option<(Context, Term, Type)> {
        if let Conclusion::Cont(ctx, t, ty) = self {
            Some((ctx, t, ty))
        } else {
            None
        }
    }

    pub fn as_exp(self) -> Option<(Context, Coterm, Cotype)> {
        if let Conclusion::Exp(ctx, t, ty) = self {
            Some((ctx, t, ty))
        } else {
            None
        }
    }

    pub fn as_contains_ty(self) -> Option<(Covar, Type, Context)> {
        if let Conclusion::ContainsTy(cv, ty, ctx) = self {
            Some((cv, ty, ctx))
        } else {
            None
        }
    }

    pub fn as_contains_coty(self) -> Option<(Covar, Cotype, Context)> {
        if let Conclusion::ContainsCoty(cv, ty, ctx) = self {
            Some((cv, ty, ctx))
        } else {
            None
        }
    }

    pub fn as_stmt(self) -> Option<(Context, Statement)> {
        if let Conclusion::Stmt(ctx, stmt) = self {
            Some((ctx, stmt))
        } else {
            None
        }
    }
}
