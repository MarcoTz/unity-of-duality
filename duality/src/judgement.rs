use super::Dual;
use negative::judgements::Conclusion as NegConc;
use positive::judgements::Conclusion as PosConc;

impl Dual for PosConc {
    type Target = NegConc;
    fn dual(self) -> Self::Target {
        match self {
            PosConc::Triv(ctx, form) => NegConc::Absurd(ctx.dual(), form.dual()),
            PosConc::False(ctx, form) => NegConc::True(ctx.dual(), form.dual()),
            PosConc::Ctx(ctx, ctx_lin) => NegConc::Ctx(ctx.dual(), ctx_lin.dual()),
            PosConc::Contra(ctx) => NegConc::Contra(ctx.dual()),
            PosConc::Contains(ctx, judg) => NegConc::Contains(ctx.dual(), judg.dual()),
        }
    }
}

impl Dual for NegConc {
    type Target = PosConc;
    fn dual(self) -> Self::Target {
        match self {
            NegConc::Absurd(ctx, form) => PosConc::Triv(ctx.dual(), form.dual()),
            NegConc::True(ctx, form) => PosConc::False(ctx.dual(), form.dual()),
            NegConc::Ctx(ctx, ctx_lin) => PosConc::Ctx(ctx.dual(), ctx_lin.dual()),
            NegConc::Contra(ctx) => PosConc::Contra(ctx.dual()),
            NegConc::Contains(ctx, ctx_lin) => PosConc::Contains(ctx.dual(), ctx_lin.dual()),
        }
    }
}
