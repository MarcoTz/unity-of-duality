use super::Dual;
use crate::judgements::Conclusion;

impl Dual for Conclusion {
    type Target = Conclusion;
    fn dual(self) -> Self::Target {
        match self {
            Conclusion::Triv(ctx, form) => Conclusion::Absurd(ctx.dual(), form.dual()),
            Conclusion::False(ctx, form) => Conclusion::True(ctx.dual(), form.dual()),
            Conclusion::Ctx(ctx, ctx_lin) => Conclusion::Ctx(ctx.dual(), ctx_lin.dual()),
            Conclusion::Contra(ctx) => Conclusion::Contra(ctx.dual()),
            Conclusion::Contains(ctx, judg) => Conclusion::Contains(ctx.dual(), judg.dual()),
            Conclusion::Absurd(ctx, form) => Conclusion::Triv(ctx.dual(), form.dual()),
            Conclusion::True(ctx, form) => Conclusion::False(ctx.dual(), form.dual()),
        }
    }
}
