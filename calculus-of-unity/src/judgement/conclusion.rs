use crate::{context::Context, terms::Term, types::Type};

#[derive(Clone)]
pub enum Conclusion {
    Val(Context, Term, Type),
}

impl Conclusion {
    pub fn as_val(self) -> Option<(Context, Term, Type)> {
        if let Conclusion::Val(context, term, ty) = self {
            Some((context, term, ty))
        } else {
            None
        }
    }
}
