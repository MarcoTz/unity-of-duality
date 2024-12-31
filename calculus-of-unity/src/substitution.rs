use crate::{terms::Term, Var};

#[derive(Clone, PartialEq, Eq)]
pub struct SubstitutionBinding {
    pub from: Var,
    pub to: Term,
}

impl SubstitutionBinding {
    pub fn apply(self, term: Term) -> Term {
        match term {
            Term::Var(v) => {
                if self.from == v {
                    self.to
                } else {
                    Term::Var(v)
                }
            }
            Term::Covar(_) => term,
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
        new_bnds.push(bnd);
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
