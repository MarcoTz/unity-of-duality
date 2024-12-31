use crate::{
    context::{Context, LinearContext},
    substitution::Substitution,
    terms::Term,
    types::Type,
};

#[derive(Clone)]
pub enum Conclusion {
    Val(Context, Term, Type),
    Subst(Context, Substitution, LinearContext),
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
}
