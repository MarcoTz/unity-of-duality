use super::{coterms::Coterm, terms::Term, Covar};

#[derive(Clone, PartialEq, Eq)]
pub enum Statement {
    CovarTerm(Covar, Term),
    TermTerm(Term, Term),
    CovarCoterm(Covar, Coterm),
    CotermCoterm(Coterm, Coterm),
}

impl Statement {
    pub fn as_covarterm(self) -> Option<(Covar, Term)> {
        if let Statement::CovarTerm(cv, t) = self {
            Some((cv, t))
        } else {
            None
        }
    }

    pub fn as_termterm(self) -> Option<(Term, Term)> {
        if let Statement::TermTerm(t1, t2) = self {
            Some((t1, t2))
        } else {
            None
        }
    }

    pub fn as_covarcoterm(self) -> Option<(Covar, Coterm)> {
        if let Statement::CovarCoterm(cv, t) = self {
            Some((cv, t))
        } else {
            None
        }
    }

    pub fn as_cotermcoterm(self) -> Option<(Coterm, Coterm)> {
        if let Statement::CotermCoterm(t1, t2) = self {
            Some((t1, t2))
        } else {
            None
        }
    }
}
