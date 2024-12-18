use crate::{
    context::{Context, ContextJudgement, LinearContext},
    positive::formula::Formula,
};

#[derive(Clone)]
pub enum Conclusion {
    Triv(Context, Formula),
    False(Context, Formula),
    Ctx(Context, LinearContext),
    Contra(Context),
    Contains(Context, ContextJudgement),
}

impl Conclusion {
    pub fn as_triv(self) -> Option<(Context, Formula)> {
        if let Conclusion::Triv(ctx, form) = self {
            Some((ctx, form))
        } else {
            None
        }
    }

    pub fn as_false(self) -> Option<(Context, Formula)> {
        if let Conclusion::False(ctx, form) = self {
            Some((ctx, form))
        } else {
            None
        }
    }

    pub fn as_ctx(self) -> Option<(Context, LinearContext)> {
        if let Conclusion::Ctx(ctx, lin_ctx) = self {
            Some((ctx, lin_ctx))
        } else {
            None
        }
    }

    pub fn as_contra(self) -> Option<Context> {
        if let Conclusion::Contra(ctx) = self {
            Some(ctx)
        } else {
            None
        }
    }

    pub fn as_contains(self) -> Option<(Context, ContextJudgement)> {
        if let Conclusion::Contains(ctx, judg) = self {
            Some((ctx, judg))
        } else {
            None
        }
    }
}
