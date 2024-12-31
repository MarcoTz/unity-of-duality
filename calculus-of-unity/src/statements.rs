use super::{terms::Term, Covar};

#[derive(Clone, PartialEq, Eq)]
pub enum Statement {
    CovarTerm(Covar, Term),
    TermTerm(Term, Term),
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
}
