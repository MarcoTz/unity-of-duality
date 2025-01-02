use crate::{coterms::Coterm, statements::Statement, terms::Term, Covar, Var};

#[derive(Clone, PartialEq, Eq)]
pub enum SubstitutionBinding {
    VarTerm { from: Var, to: Term },
    CovarTerm { from: Covar, to: Term },
    VarCoterm { from: Var, to: Coterm },
    CovarCoterm { from: Covar, to: Coterm },
}

impl SubstitutionBinding {
    pub fn as_var_term(self) -> Option<(Var, Term)> {
        if let SubstitutionBinding::VarTerm { from, to } = self {
            Some((from, to))
        } else {
            None
        }
    }

    pub fn as_covar_term(self) -> Option<(Covar, Term)> {
        if let SubstitutionBinding::CovarTerm { from, to } = self {
            Some((from, to))
        } else {
            None
        }
    }

    pub fn as_var_coterm(self) -> Option<(Var, Coterm)> {
        if let SubstitutionBinding::VarCoterm { from, to } = self {
            Some((from, to))
        } else {
            None
        }
    }

    pub fn as_covar_coterm(self) -> Option<(Covar, Coterm)> {
        if let SubstitutionBinding::CovarCoterm { from, to } = self {
            Some((from, to))
        } else {
            None
        }
    }

    pub fn apply_term(self, term: Term) -> Term {
        match term {
            Term::Var(v) => {
                let (from, to) = if let SubstitutionBinding::VarTerm { from, to } = self {
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
                let (from, to) = if let SubstitutionBinding::CovarTerm { from, to } = self {
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
            Term::Pair(fst, snd) => {
                Term::pair(self.clone().apply_term(*fst), self.apply_term(*snd))
            }
            Term::Inl(t) => Term::inl(self.apply_term(*t)),
            Term::Inr(t) => Term::inr(self.apply_term(*t)),
            Term::Pattern(stmts) => Term::Pattern(
                stmts
                    .into_iter()
                    .map(|(t, stmt)| (self.clone().apply_term(t), self.clone().apply_stmt(stmt)))
                    .collect(),
            ),
        }
    }

    pub fn apply_coterm(self, coterm: Coterm) -> Coterm {
        match coterm {
            Coterm::Var(v) => {
                let (from, to) = if let SubstitutionBinding::VarCoterm { from, to } = self {
                    (from, to)
                } else {
                    return Coterm::Var(v);
                };
                if v == from {
                    to
                } else {
                    Coterm::Var(v)
                }
            }
            Coterm::Covar(cv) => {
                let (from, to) = if let SubstitutionBinding::CovarCoterm { from, to } = self {
                    (from, to)
                } else {
                    return Coterm::Covar(cv);
                };
                if cv == from {
                    to
                } else {
                    Coterm::Covar(cv)
                }
            }
            Coterm::Counit => Coterm::Counit,
            Coterm::Fst(t) => Coterm::fst(self.apply_coterm(*t)),
            Coterm::Snd(t) => Coterm::snd(self.apply_coterm(*t)),
            Coterm::LPair(fst, snd) => {
                Coterm::lpair(self.clone().apply_coterm(*fst), self.apply_coterm(*snd))
            }
            Coterm::Copattern(pts) => Coterm::Copattern(
                pts.into_iter()
                    .map(|(cot, stmt)| {
                        (
                            self.clone().apply_coterm(cot),
                            self.clone().apply_stmt(stmt),
                        )
                    })
                    .collect(),
            ),
        }
    }

    pub fn apply_stmt(self, stmt: Statement) -> Statement {
        match stmt {
            Statement::CovarTerm(covar, term) => {
                let term_subst = self.clone().apply_term(term);
                if let SubstitutionBinding::CovarTerm { from, to } = self {
                    if from == covar {
                        Statement::TermTerm(to, term_subst)
                    } else {
                        Statement::CovarTerm(covar, term_subst)
                    }
                } else {
                    Statement::CovarTerm(covar, term_subst)
                }
            }
            Statement::TermTerm(t1, t2) => {
                let t1_subst = self.clone().apply_term(t1);
                let t2_subst = self.apply_term(t2);
                Statement::TermTerm(t1_subst, t2_subst)
            }
            Statement::CovarCoterm(cv, t) => {
                let t_subst = self.clone().apply_coterm(t);
                let (from, to) = if let SubstitutionBinding::CovarCoterm { from, to } = self {
                    (from, to)
                } else {
                    return Statement::CovarCoterm(cv, t_subst);
                };
                if cv == from {
                    Statement::CotermCoterm(to, t_subst)
                } else {
                    Statement::CovarCoterm(cv, t_subst)
                }
            }
            Statement::CotermCoterm(t1, t2) => Statement::CotermCoterm(
                self.clone().apply_coterm(t1),
                self.clone().apply_coterm(t2),
            ),
            Statement::Done => Statement::Done,
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

    pub fn apply_term(self, term: Term) -> Term {
        let mut t = term;
        for bnd in self.0 {
            t = bnd.apply_term(t);
        }
        t
    }

    pub fn apply_coterm(self, coterm: Coterm) -> Coterm {
        let mut t = coterm;
        for bnd in self.0 {
            t = bnd.apply_coterm(t)
        }
        t
    }
}
