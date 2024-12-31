use super::TypeVar;

#[derive(Clone, PartialEq, Eq)]
pub enum Cotype {
    Var(TypeVar),
    Bot,
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
