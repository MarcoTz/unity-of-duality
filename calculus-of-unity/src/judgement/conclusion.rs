use crate::{
    context::{Context, LinearContext},
    substitution::Substitution,
    terms::Term,
    types::Type,
    Covar,
};

#[derive(Clone)]
pub enum Conclusion {
    Val(Context, Term, Type),
    Cont(Context, Term, Type),
    Subst(Context, Substitution, LinearContext),
    Contains(Covar, Type, Context),
}

impl Conclusion {
    pub fn as_val(self) -> Option<(Context, Term, Type)> {
        if let Conclusion::Val(context, term, ty) = self {
            Some((context, term, ty))
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

    pub fn as_contains(self) -> Option<(Covar, Type, Context)> {
        if let Conclusion::Contains(cv, ty, ctx) = self {
            Some((cv, ty, ctx))
        } else {
            None
        }
    }
}
