use crate::{
    context::{Context, ContextJudgement, LinearContext},
    negative::formula::Formula,
};

#[derive(Clone, PartialEq, Eq)]
pub enum Conclusion {
    Absurd(Context, Formula),
    True(Context, Formula),
    Ctx(Context, LinearContext),
    Contra(Context),
    Contains(Context, ContextJudgement),
}

impl Conclusion {
    pub fn as_absurd(self) -> Option<(Context, Formula)> {
        if let Conclusion::Absurd(ctx, form) = self {
            Some((ctx, form))
        } else {
            None
        }
    }

    pub fn as_true(self) -> Option<(Context, Formula)> {
        if let Conclusion::True(ctx, form) = self {
            Some((ctx, form))
        } else {
            None
        }
    }

    pub fn as_ctx(self) -> Option<(Context, LinearContext)> {
        if let Conclusion::Ctx(ctx, ctx_lin) = self {
            Some((ctx, ctx_lin))
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
