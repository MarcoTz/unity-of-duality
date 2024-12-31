use crate::{terms::Term, Covar, Var};

#[derive(Clone, PartialEq, Eq)]
pub enum SubstitutionBinding {
    VarBinding { from: Var, to: Term },
    CovarBinding { from: Covar, to: Term },
}

impl SubstitutionBinding {
    pub fn as_var(self) -> Option<(Var, Term)> {
        if let SubstitutionBinding::VarBinding { from, to } = self {
            Some((from, to))
        } else {
            None
        }
    }

    pub fn as_covar(self) -> Option<(Covar, Term)> {
        if let SubstitutionBinding::CovarBinding { from, to } = self {
            Some((from, to))
        } else {
            None
        }
    }

    pub fn apply(self, term: Term) -> Term {
        match term {
            Term::Var(v) => {
                let (from, to) = if let SubstitutionBinding::VarBinding { from, to } = self {
                    (from, to)
                } else {
                    return Term::Var(v);
                };

                if from == v {
                    to
                } else {
                    Term::Var(v)
                }
            }
            Term::Covar(cv) => {
                let (from, to) = if let SubstitutionBinding::CovarBinding { from, to } = self {
                    (from, to)
                } else {
                    return Term::Covar(cv);
                };
                if cv == from {
                    to
                } else {
                    Term::Covar(cv)
                }
            }
            Term::Unit => term,
            Term::Pair(fst, snd) => Term::pair(self.clone().apply(*fst), self.apply(*snd)),
            Term::Inl(t) => Term::inl(self.apply(*t)),
            Term::Inr(t) => Term::inr(self.apply(*t)),
        }
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Substitution(pub Vec<SubstitutionBinding>);

impl Substitution {
    pub fn add(self, bnd: SubstitutionBinding) -> Substitution {
        let mut new_bnds = self.0;
        new_bnds.insert(0, bnd);
        Substitution(new_bnds)
    }

    pub fn apply(self, term: Term) -> Term {
        let mut t = term;
        for bnd in self.0 {
            t = bnd.apply(t);
        }
        t
    }
}
