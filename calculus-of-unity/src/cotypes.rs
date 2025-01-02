use super::{
    context::{ContextJudgement, LinearContext},
    coterms::Coterm,
    TypeVar,
};

#[derive(Clone, PartialEq, Eq)]
pub enum Cotype {
    Var(TypeVar),
    Bot,
    Top,
    NegN(Box<Cotype>),
    And(Box<Cotype>, Box<Cotype>),
    Par(Box<Cotype>, Box<Cotype>),
}

impl Cotype {
    pub fn negn(ty: Cotype) -> Cotype {
        Cotype::NegN(Box::new(ty))
    }

    pub fn and(ty1: Cotype, ty2: Cotype) -> Cotype {
        Cotype::And(Box::new(ty1), Box::new(ty2))
    }

    pub fn par(ty1: Cotype, ty2: Cotype) -> Cotype {
        Cotype::Par(Box::new(ty1), Box::new(ty2))
    }

    pub fn as_var(self) -> Option<TypeVar> {
        if let Cotype::Var(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_neg(self) -> Option<Cotype> {
        if let Cotype::NegN(ty) = self {
            Some(*ty)
        } else {
            None
        }
    }

    pub fn as_and(self) -> Option<(Cotype, Cotype)> {
        if let Cotype::And(ty1, ty2) = self {
            Some((*ty1, *ty2))
        } else {
            None
        }
    }

    pub fn as_par(self) -> Option<(Cotype, Cotype)> {
        if let Cotype::Par(ty1, ty2) = self {
            Some((*ty1, *ty2))
        } else {
            None
        }
    }

    pub fn copatterns(self) -> Option<Vec<(LinearContext, Coterm)>> {
        match self {
            Cotype::Var(v) => Some(vec![(
                ContextJudgement::Covalue("x".to_owned(), v).into(),
                Coterm::Var("x".to_owned()),
            )]),
            Cotype::Bot => Some(vec![(LinearContext::default(), Coterm::Counit)]),
            Cotype::Top => None,
            Cotype::And(l, r) => {
                let mut pts = l.copatterns()?;
                pts.extend(r.copatterns()?);
                Some(pts)
            }
            Cotype::Par(l, r) => {
                let pts_left = l.copatterns()?;
                let pts_right = r.copatterns()?;
                let mut pts = vec![];
                for (ctx_left, t_left) in pts_left.iter() {
                    for (ctx_right, t_right) in pts_right.iter() {
                        let new_ctx = ctx_left.clone().combine(ctx_right.clone());
                        let new_t = Coterm::lpair(t_left.clone(), t_right.clone());
                        pts.push((new_ctx, new_t))
                    }
                }
                Some(pts)
            }
            Cotype::NegN(t) => Some(vec![(
                ContextJudgement::Expression("u".to_owned(), *t).into(),
                Coterm::Covar("u".to_owned()),
            )]),
        }
    }
}

impl From<TypeVar> for Cotype {
    fn from(var: TypeVar) -> Cotype {
        Cotype::Var(var)
    }
}

impl From<&str> for Cotype {
    fn from(s: &str) -> Cotype {
        Cotype::Var(s.to_owned())
    }
}
