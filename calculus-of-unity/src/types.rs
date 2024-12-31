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
