use crate::{
    context::{ContextJudgement, LinearContext},
    cotypes::Cotype,
    terms::Term,
    TypeVar,
};

#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    Var(TypeVar),
    NegV(Box<Type>),
    One,
    Zero,
    Times(Box<Type>, Box<Type>),
    Plus(Box<Type>, Box<Type>),
    Shift(Box<Cotype>),
}

impl Type {
    pub fn negv(ty: Type) -> Type {
        Type::NegV(Box::new(ty))
    }

    pub fn times(fst: Type, snd: Type) -> Type {
        Type::Times(Box::new(fst), Box::new(snd))
    }

    pub fn plus(left: Type, right: Type) -> Type {
        Type::Plus(Box::new(left), Box::new(right))
    }

    pub fn shift(ty: Cotype) -> Type {
        Type::Shift(Box::new(ty))
    }

    pub fn as_var(self) -> Option<TypeVar> {
        if let Type::Var(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_neg(self) -> Option<Type> {
        if let Type::NegV(ty) = self {
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

    pub fn as_shift(self) -> Option<Cotype> {
        if let Type::Shift(cot) = self {
            Some(*cot)
        } else {
            None
        }
    }

    pub fn patterns(self) -> Option<Vec<(LinearContext, Term)>> {
        match self {
            Type::Var(v) => Some(vec![(
                ContextJudgement::Value("x".to_owned(), v).into(),
                Term::var("x"),
            )]),
            Type::NegV(ty) => Some(vec![(
                ContextJudgement::Continuation("u".to_owned(), *ty).into(),
                Term::covar("u"),
            )]),
            Type::One => Some(vec![(LinearContext::default(), Term::Unit)]),
            Type::Zero => None,
            Type::Times(ty1, ty2) => {
                let fst_patterns = ty1.patterns()?;
                let snd_patterns = ty2.patterns()?;
                let mut times_patterns = vec![];
                for (fst_ctx, fst_term) in fst_patterns.iter() {
                    for (snd_ctx, snd_term) in snd_patterns.iter() {
                        times_patterns.push((
                            fst_ctx.clone().combine(snd_ctx.clone()),
                            Term::pair(fst_term.clone(), snd_term.clone()),
                        ));
                    }
                }
                Some(times_patterns)
            }
            Type::Plus(left, right) => {
                let mut patterns: Vec<(LinearContext, Term)> = left
                    .patterns()?
                    .into_iter()
                    .map(|(ctx, t)| (ctx, Term::inl(t)))
                    .collect();
                patterns.extend(
                    right
                        .patterns()?
                        .into_iter()
                        .map(|(ctx, t)| (ctx, Term::inr(t))),
                );
                Some(patterns)
            }
            Type::Shift(cot) => Some(vec![(
                ContextJudgement::Expression("u".to_owned(), *cot).into(),
                Term::Covar("u".to_owned()),
            )]),
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
