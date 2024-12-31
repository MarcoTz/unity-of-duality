use crate::{
    context::{ContextJudgement, LinearContext},
    terms::Term,
};
pub type TypeVar = String;

#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    Var(TypeVar),
    Neg(Box<Type>),
    One,
    Times(Box<Type>, Box<Type>),
    Plus(Box<Type>, Box<Type>),
}

impl Type {
    pub fn ng(ty: Type) -> Type {
        Type::Neg(Box::new(ty))
    }

    pub fn times(fst: Type, snd: Type) -> Type {
        Type::Times(Box::new(fst), Box::new(snd))
    }

    pub fn plus(left: Type, right: Type) -> Type {
        Type::Plus(Box::new(left), Box::new(right))
    }

    pub fn as_var(self) -> Option<TypeVar> {
        if let Type::Var(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_neg(self) -> Option<Type> {
        if let Type::Neg(ty) = self {
            Some(*ty)
        } else {
            None
        }
    }

    pub fn as_times(self) -> Option<(Type, Type)> {
        if let Type::Times(ty1, ty2) = self {
            Some((*ty1, *ty2))
        } else {
            None
        }
    }

    pub fn as_plus(self) -> Option<(Type, Type)> {
        if let Type::Plus(left, right) = self {
            Some((*left, *right))
        } else {
            None
        }
    }

    pub fn patterns(self) -> Vec<(LinearContext, Term)> {
        match self {
            Type::Var(v) => vec![(
                ContextJudgement::Value("x".to_owned(), v).into(),
                Term::var("x"),
            )],
            Type::Neg(ty) => vec![(
                ContextJudgement::Continuation("u".to_owned(), *ty).into(),
                Term::covar("u"),
            )],
            Type::One => vec![(LinearContext::default(), Term::Unit)],
            Type::Times(ty1, ty2) => {
                let fst_patterns = ty1.patterns();
                let snd_patterns = ty2.patterns();
                let mut times_patterns = vec![];
                for (fst_ctx, fst_term) in fst_patterns.iter() {
                    for (snd_ctx, snd_term) in snd_patterns.iter() {
                        times_patterns.push((
                            fst_ctx.clone().combine(snd_ctx.clone()),
                            Term::pair(fst_term.clone(), snd_term.clone()),
                        ));
                    }
                }
                times_patterns
            }
            Type::Plus(left, right) => {
                let mut patterns: Vec<(LinearContext, Term)> = left
                    .patterns()
                    .into_iter()
                    .map(|(ctx, t)| (ctx, Term::inl(t)))
                    .collect();
                patterns.extend(
                    right
                        .patterns()
                        .into_iter()
                        .map(|(ctx, t)| (ctx, Term::inr(t))),
                );
                patterns
            }
        }
    }
}

impl From<&str> for Type {
    fn from(s: &str) -> Type {
        Type::Var(s.to_owned())
    }
}

impl From<TypeVar> for Type {
    fn from(v: TypeVar) -> Type {
        Type::Var(v)
    }
}
