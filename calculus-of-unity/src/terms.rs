use super::{Covar, Var};

#[derive(Clone, PartialEq, Eq)]
pub enum Term {
    Var(Var),
    Covar(Covar),
    Unit,
    Pair(Box<Term>, Box<Term>),
    Inl(Box<Term>),
    Inr(Box<Term>),
}

impl Term {
    pub fn pair(fst: Term, snd: Term) -> Term {
        Term::Pair(Box::new(fst), Box::new(snd))
    }

    pub fn inl(t: Term) -> Term {
        Term::Inl(Box::new(t))
    }

    pub fn inr(t: Term) -> Term {
        Term::Inr(Box::new(t))
    }

    pub fn as_var(self) -> Option<Var> {
        if let Term::Var(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_covar(self) -> Option<Covar> {
        if let Term::Covar(cv) = self {
            Some(cv)
        } else {
            None
        }
    }

    pub fn as_pair(self) -> Option<(Term, Term)> {
        if let Term::Pair(fst, snd) = self {
            Some((*fst, *snd))
        } else {
            None
        }
    }

    pub fn as_inl(self) -> Option<Term> {
        if let Term::Inl(t) = self {
            Some(*t)
        } else {
            None
        }
    }

    pub fn as_inr(self) -> Option<Term> {
        if let Term::Inr(t) = self {
            Some(*t)
        } else {
            None
        }
    }
}

impl From<&str> for Term {
    fn from(s: &str) -> Term {
        Term::Var(s.to_owned())
    }
}

impl From<Var> for Term {
    fn from(v: Var) -> Term {
        Term::Var(v)
    }
}
